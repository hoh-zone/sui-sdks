package graphql

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
)

type Query struct {
	Document string
}

type TypedQuery[Result any, Variables any] struct {
	Query         string
	OperationName string
	Variables     map[string]any
	Extensions    map[string]any
}

type QueryBuilder[Result any, Variables any] struct {
	query      string
	operation  string
	variables  map[string]any
	extensions map[string]any
}

func (qb *QueryBuilder[Result, Variables]) Build() *TypedQuery[Result, Variables] {
	return &TypedQuery[Result, Variables]{
		Query:         qb.query,
		OperationName: qb.operation,
		Variables:     qb.variables,
		Extensions:    qb.extensions,
	}
}

type Plugin interface {
	BeforeQuery(ctx context.Context, opts QueryOptions) (QueryOptions, error)
	AfterQuery(ctx context.Context, result *QueryResult, err error) error
}

type QueryExecutor struct {
	plugins []Plugin
}

func NewQueryExecutor(plugins ...Plugin) *QueryExecutor {
	return &QueryExecutor{
		plugins: plugins,
	}
}

func (e *QueryExecutor) Execute(ctx context.Context, opts QueryOptions) (*QueryResult, error) {
	for _, plugin := range e.plugins {
		var err error
		opts, err = plugin.BeforeQuery(ctx, opts)
		if err != nil {
			return nil, fmt.Errorf("before query plugin error: %w", err)
		}
	}

	result, err := executeQuery(ctx, opts)
	if err != nil {
		return nil, err
	}

	for _, plugin := range e.plugins {
		if err := plugin.AfterQuery(ctx, result, nil); err != nil {
			return nil, fmt.Errorf("after query plugin error: %w", err)
		}
	}

	return result, nil
}

func executeQuery(ctx context.Context, opts QueryOptions) (*QueryResult, error) {
	payload, err := json.Marshal(opts)
	if err != nil {
		return nil, err
	}

	result := &QueryResult{}
	if err := json.Unmarshal(payload, &result); err != nil {
		return nil, err
	}

	return result, nil
}

type QueryValidator interface {
	Validate(query string) error
	ValidateVariables(vars map[string]any) error
}

type QueryCache interface {
	Get(key string) (*QueryResult, bool)
	Set(key string, result *QueryResult)
	Invalidate(key string)
}

type InMemoryQueryCache struct {
	data map[string]*QueryResult
	mu   sync.RWMutex
}

func NewInMemoryQueryCache() *InMemoryQueryCache {
	return &InMemoryQueryCache{
		data: make(map[string]*QueryResult),
	}
}

func (c *InMemoryQueryCache) Get(key string) (*QueryResult, bool) {
	c.mu.RLock()
	defer c.mu.RUnlock()
	result, ok := c.data[key]
	return result, ok
}

func (c *InMemoryQueryCache) Set(key string, result *QueryResult) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.data[key] = result
}

func (c *InMemoryQueryCache) Invalidate(key string) {
	c.mu.Lock()
	defer c.mu.Unlock()
	delete(c.data, key)
}

func CacheKey(query string, variables map[string]any) string {
	key := query
	if len(variables) > 0 {
		bs, _ := json.Marshal(variables)
		key += string(bs)
	}
	return key
}
