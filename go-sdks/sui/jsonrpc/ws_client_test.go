package jsonrpc

import (
	"context"
	"encoding/json"
	"net/http/httptest"
	"net/url"
	"testing"

	"golang.org/x/net/websocket"
)

func TestWebsocketClientRequest(t *testing.T) {
	srv := httptest.NewServer(websocket.Handler(func(conn *websocket.Conn) {
		defer conn.Close()
		for {
			var payload []byte
			if err := websocket.Message.Receive(conn, &payload); err != nil {
				return
			}
			var req map[string]any
			if err := json.Unmarshal(payload, &req); err != nil {
				return
			}
			id, _ := req["id"].(float64)
			method, _ := req["method"].(string)
			resp := map[string]any{
				"jsonrpc": "2.0",
				"id":      int64(id),
				"result":  map[string]any{"method": method},
			}
			out, _ := json.Marshal(resp)
			_ = websocket.Message.Send(conn, out)
		}
	}))
	defer srv.Close()

	u, err := url.Parse(srv.URL)
	if err != nil {
		t.Fatalf("parse url failed: %v", err)
	}
	u.Scheme = "ws"

	client := NewWebsocketClient(WebsocketClientOptions{Endpoint: u.String()})
	var out map[string]any
	if err := client.MakeRequest(context.Background(), "sui_ping", []any{}, &out); err != nil {
		t.Fatalf("make request failed: %v", err)
	}
	if out["method"] != "sui_ping" {
		t.Fatalf("unexpected method result: %v", out)
	}
}
