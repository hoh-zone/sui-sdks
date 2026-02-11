package jsonrpc

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"sync"
	"sync/atomic"
	"time"

	"golang.org/x/net/websocket"
)

type WebsocketClientOptions struct {
	Endpoint         string
	Headers          map[string]string
	CallTimeout      time.Duration
	ReconnectTimeout time.Duration
}

type jsonrpcWSMessage struct {
	JSONRPC string           `json:"jsonrpc,omitempty"`
	ID      *int64           `json:"id,omitempty"`
	Method  string           `json:"method,omitempty"`
	Params  *json.RawMessage `json:"params,omitempty"`
	Result  json.RawMessage  `json:"result,omitempty"`
	Error   *JsonRPCError    `json:"error,omitempty"`
}

type subscriptionParams struct {
	Subscription int64           `json:"subscription"`
	Result       json.RawMessage `json:"result"`
}

type WebsocketClient struct {
	endpoint string
	headers  map[string]string

	callTimeout time.Duration

	connMu  sync.RWMutex
	conn    *websocket.Conn
	writeMu sync.Mutex

	seq     int64
	pending map[int64]chan jsonrpcWSMessage
	subs    map[int64]func(json.RawMessage)
	mu      sync.RWMutex
	closed  atomic.Bool
}

func NewWebsocketClient(opts WebsocketClientOptions) *WebsocketClient {
	if opts.CallTimeout <= 0 {
		opts.CallTimeout = 30 * time.Second
	}
	return &WebsocketClient{
		endpoint:    toWebsocketEndpoint(opts.Endpoint),
		headers:     opts.Headers,
		callTimeout: opts.CallTimeout,
		pending:     map[int64]chan jsonrpcWSMessage{},
		subs:        map[int64]func(json.RawMessage){},
	}
}

func (c *WebsocketClient) Connect(ctx context.Context) error {
	c.connMu.Lock()
	defer c.connMu.Unlock()
	if c.conn != nil {
		return nil
	}
	cfg, err := websocket.NewConfig(c.endpoint, "http://localhost/")
	if err != nil {
		return err
	}
	cfg.Header = http.Header{}
	for k, v := range c.headers {
		cfg.Header.Set(k, v)
	}
	conn, err := websocket.DialConfig(cfg)
	if err != nil {
		return err
	}
	c.conn = conn
	go c.readLoop(ctx)
	return nil
}

func (c *WebsocketClient) Close() error {
	c.closed.Store(true)
	c.connMu.Lock()
	defer c.connMu.Unlock()
	if c.conn == nil {
		return nil
	}
	err := c.conn.Close()
	c.conn = nil
	return err
}

func (c *WebsocketClient) MakeRequest(ctx context.Context, method string, params []any, out any) error {
	if err := c.Connect(ctx); err != nil {
		return err
	}
	id := atomic.AddInt64(&c.seq, 1)
	ch := make(chan jsonrpcWSMessage, 1)
	c.mu.Lock()
	c.pending[id] = ch
	c.mu.Unlock()
	defer func() {
		c.mu.Lock()
		delete(c.pending, id)
		c.mu.Unlock()
	}()

	body := map[string]any{
		"jsonrpc": "2.0",
		"id":      id,
		"method":  method,
		"params":  params,
	}
	if err := c.write(body); err != nil {
		return err
	}

	timeout := c.callTimeout
	if dl, ok := ctx.Deadline(); ok {
		timeout = time.Until(dl)
	}
	timer := time.NewTimer(timeout)
	defer timer.Stop()

	select {
	case <-ctx.Done():
		return ctx.Err()
	case <-timer.C:
		return fmt.Errorf("websocket request timeout: %s", method)
	case msg := <-ch:
		if msg.Error != nil {
			return msg.Error
		}
		if out == nil {
			return nil
		}
		return json.Unmarshal(msg.Result, out)
	}
}

func (c *WebsocketClient) Subscribe(ctx context.Context, method, unsubscribeMethod string, params []any, onMessage func(json.RawMessage)) (func() error, error) {
	var subID int64
	if err := c.MakeRequest(ctx, method, params, &subID); err != nil {
		return nil, err
	}
	c.mu.Lock()
	c.subs[subID] = onMessage
	c.mu.Unlock()
	return func() error {
		c.mu.Lock()
		delete(c.subs, subID)
		c.mu.Unlock()
		return c.MakeRequest(context.Background(), unsubscribeMethod, []any{subID}, nil)
	}, nil
}

func (c *WebsocketClient) readLoop(ctx context.Context) {
	_ = ctx
	for {
		if c.closed.Load() {
			return
		}
		c.connMu.RLock()
		conn := c.conn
		c.connMu.RUnlock()
		if conn == nil {
			return
		}
		var raw []byte
		if err := websocket.Message.Receive(conn, &raw); err != nil {
			return
		}
		var msg jsonrpcWSMessage
		if err := json.Unmarshal(raw, &msg); err != nil {
			continue
		}
		if msg.ID != nil {
			c.mu.RLock()
			ch := c.pending[*msg.ID]
			c.mu.RUnlock()
			if ch != nil {
				select {
				case ch <- msg:
				default:
				}
			}
			continue
		}
		if msg.Params != nil {
			var p subscriptionParams
			if err := json.Unmarshal(*msg.Params, &p); err != nil {
				continue
			}
			c.mu.RLock()
			cb := c.subs[p.Subscription]
			c.mu.RUnlock()
			if cb != nil {
				cb(p.Result)
			}
		}
	}
}

func (c *WebsocketClient) write(body any) error {
	c.connMu.RLock()
	conn := c.conn
	c.connMu.RUnlock()
	if conn == nil {
		return fmt.Errorf("websocket is not connected")
	}
	c.writeMu.Lock()
	defer c.writeMu.Unlock()
	data, err := json.Marshal(body)
	if err != nil {
		return err
	}
	return websocket.Message.Send(conn, data)
}

func toWebsocketEndpoint(endpoint string) string {
	if endpoint == "" {
		return ""
	}
	u, err := url.Parse(endpoint)
	if err != nil {
		return endpoint
	}
	switch u.Scheme {
	case "http":
		u.Scheme = "ws"
	case "https":
		u.Scheme = "wss"
	}
	return u.String()
}
