package client

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"sync"
	"time"
)

type Cache interface {
	Get(key string) (interface{}, bool)
	Set(key string, value interface{})
	Delete(key string)
}

type InMemoryCache struct {
	mu   sync.RWMutex
	data map[string]interface{}
	ttls map[string]time.Time
}

func NewInMemoryCache() *InMemoryCache {
	return &InMemoryCache{
		data: make(map[string]interface{}),
		ttls: make(map[string]time.Time),
	}
}

func (c *InMemoryCache) Get(key string) (interface{}, bool) {
	c.mu.RLock()
	defer c.mu.RUnlock()
	v, ok := c.data[key]
	return v, ok
}

func (c *InMemoryCache) Set(key string, value interface{}) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.data[key] = value
}

func (c *InMemoryCache) Delete(key string) {
	c.mu.Lock()
	defer c.mu.Unlock()
	delete(c.data, key)
}

type CacheOptions struct {
	TTL time.Duration
}

type ClientCache struct {
	cache  Cache
	scopes map[string]Cache
	mu     sync.RWMutex
}

func NewClientCache() *ClientCache {
	return &ClientCache{
		cache:  NewInMemoryCache(),
		scopes: make(map[string]Cache),
	}
}

func (c *ClientCache) Scope(scope string) Cache {
	c.mu.Lock()
	defer c.mu.Unlock()
	if _, ok := c.scopes[scope]; !ok {
		c.scopes[scope] = NewInMemoryCache()
	}
	return c.scopes[scope]
}

func (c *ClientCache) Get(key string) (interface{}, bool) {
	return c.cache.Get(key)
}

func (c *ClientCache) Set(key string, value interface{}) {
	c.cache.Set(key, value)
}

func (c *ClientCache) Delete(key string) {
	c.cache.Delete(key)
}

type MVRClient struct {
	url      string
	cache    Cache
	pageSize int
	client   *http.Client
	mu       sync.RWMutex
}

type MvrOptions struct {
	URL       string
	Cache     Cache
	PageSize  int
	Overrides map[string]map[string]string
}

type ResolvePackageRequest struct {
	Package string `json:"package"`
}

type ResolvePackageResponse struct {
	Package string `json:"package"`
}

type ResolveTypeRequest struct {
	Type string `json:"type"`
}

type ResolveTypeResponse struct {
	Type string `json:"type"`
}

type ResolveResponse struct {
	Packages map[string]struct {
		Package string `json:"package"`
	} `json:"packages"`
	Types map[string]struct {
		Type string `json:"type"`
	} `json:"types"`
}

type ResolveRequest struct {
	Packages []string `json:"packages,omitempty"`
	Types    []string `json:"types,omitempty"`
}

type CachedResponse struct {
	Data      interface{}   `json:"data"`
	Timestamp time.Time     `json:"timestamp"`
	TTL       time.Duration `json:"ttl"`
}

var DefaultMVRURLs = map[string]string{
	"mainnet": "https://mainnet.mvr.mystenlabs.com",
	"testnet": "https://testnet.mvr.mystenlabs.com",
}

func NewMvrClient(network string, opts MvrOptions) *MVRClient {
	if opts.URL == "" {
		opts.URL = DefaultMVRURLs[network]
	}
	if opts.PageSize <= 0 {
		opts.PageSize = 100
	}

	return &MVRClient{
		url:      opts.URL,
		cache:    opts.Cache,
		pageSize: opts.PageSize,
		client:   &http.Client{Timeout: 30 * time.Second},
	}
}

func (m *MVRClient) ResolvePackage(ctx context.Context, pkg string) (string, error) {
	cacheKey := fmt.Sprintf("mvr.package.%s", pkg)

	if m.cache != nil {
		if cached, found := m.cache.Get(cacheKey); found {
			if cached, ok := cached.(CachedResponse); ok {
				if time.Since(cached.Timestamp) < cached.TTL {
					if resp, ok := cached.Data.(ResolvePackageResponse); ok {
						return resp.Package, nil
					}
				}
			}
		}
	}

	reqBody := ResolvePackageRequest{Package: pkg}
	respBody, err := m.makeRequest(ctx, "/resolve_package", reqBody)
	if err != nil {
		return "", err
	}

	var resp ResolvePackageResponse
	if err := json.Unmarshal(respBody, &resp); err != nil {
		return "", fmt.Errorf("failed to unmarshal response: %w", err)
	}

	if m.cache != nil {
		m.cache.Set(cacheKey, CachedResponse{
			Data:      resp,
			Timestamp: time.Now(),
			TTL:       5 * time.Minute,
		})
	}

	return resp.Package, nil
}

func (m *MVRClient) ResolveType(ctx context.Context, typeStr string) (string, error) {
	cacheKey := fmt.Sprintf("mvr.type.%s", typeStr)

	if m.cache != nil {
		if cached, found := m.cache.Get(cacheKey); found {
			if cached, ok := cached.(CachedResponse); ok {
				if time.Since(cached.Timestamp) < cached.TTL {
					if resp, ok := cached.Data.(ResolveTypeResponse); ok {
						return resp.Type, nil
					}
				}
			}
		}
	}

	reqBody := ResolveTypeRequest{Type: typeStr}
	respBody, err := m.makeRequest(ctx, "/resolve_type", reqBody)
	if err != nil {
		return "", err
	}

	var resp ResolveTypeResponse
	if err := json.Unmarshal(respBody, &resp); err != nil {
		return "", fmt.Errorf("failed to unmarshal response: %w", err)
	}

	if m.cache != nil {
		m.cache.Set(cacheKey, CachedResponse{
			Data:      resp,
			Timestamp: time.Now(),
			TTL:       5 * time.Minute,
		})
	}

	return resp.Type, nil
}

func (m *MVRClient) Resolve(ctx context.Context, pkgs []string, types []string) (*ResolveResponse, error) {
	cacheKey := fmt.Sprintf("mvr.resolve.%v.%v", pkgs, types)

	if m.cache != nil {
		if cached, found := m.cache.Get(cacheKey); found {
			if cached, ok := cached.(CachedResponse); ok {
				if time.Since(cached.Timestamp) < cached.TTL {
					if resp, ok := cached.Data.(*ResolveResponse); ok {
						return resp, nil
					}
				}
			}
		}
	}

	reqBody := ResolveRequest{
		Packages: pkgs,
		Types:    types,
	}
	respBody, err := m.makeRequest(ctx, "/resolve", reqBody)
	if err != nil {
		return nil, err
	}

	var resp ResolveResponse
	if err := json.Unmarshal(respBody, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	if m.cache != nil {
		if len(pkgs) == 0 && len(types) == 0 {
			m.cache.Set(cacheKey, CachedResponse{
				Data:      &resp,
				Timestamp: time.Now(),
				TTL:       10 * time.Minute,
			})
		}
	}

	return &resp, nil
}

func (m *MVRClient) makeRequest(ctx context.Context, path string, body interface{}) ([]byte, error) {
	reqBody, err := json.Marshal(body)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	url := m.url + path

	maxRetries := 3
	for i := 0; i < maxRetries; i++ {
		req, err := http.NewRequestWithContext(ctx, "POST", url, bytes.NewReader(reqBody))
		if err != nil {
			if i == maxRetries-1 {
				return nil, fmt.Errorf("failed to create request after %d retries: %w", maxRetries, err)
			}
			time.Sleep(time.Duration(i+1) * 100 * time.Millisecond)
			continue
		}
		req.Header.Set("Content-Type", "application/json")

		resp, err := m.client.Do(req)
		if err != nil {
			if i == maxRetries-1 {
				return nil, fmt.Errorf("failed to make request after %d retries: %w", maxRetries, err)
			}
			time.Sleep(time.Duration(i+1) * 100 * time.Millisecond)
			continue
		}
		defer resp.Body.Close()

		if resp.StatusCode == http.StatusOK {
			return io.ReadAll(resp.Body)
		}

		if i == maxRetries-1 {
			return nil, fmt.Errorf("request failed with status %d", resp.StatusCode)
		}
		time.Sleep(time.Duration(i+1) * 100 * time.Millisecond)
	}

	return nil, fmt.Errorf("unexpected error")
}

func (m *MVRClient) Close() error {
	m.cache = nil
	return nil
}
