package graphql

import "context"

type NamedQuery struct {
	name   string
	query  string
	result any
}

type NamedQueries struct {
	queries map[string]*NamedQuery
}

func NewNamedQueries() *NamedQueries {
	return &NamedQueries{
		queries: make(map[string]*NamedQuery),
	}
}

func (nq *NamedQueries) Register(name, query string, result any) {
	nq.queries[name] = &NamedQuery{
		name:   name,
		query:  query,
		result: result,
	}
}

func (nq *NamedQueries) Get(name string) (*NamedQuery, bool) {
	query, ok := nq.queries[name]
	return query, ok
}

func (nq *NamedQueries) Unregister(name string) {
	delete(nq.queries, name)
}

func (nq *NamedQueries) List() []string {
	names := make([]string, 0, len(nq.queries))
	for name := range nq.queries {
		names = append(names, name)
	}
	return names
}

type QueryBuilderImpl struct {
	client  *Client
	cache   QueryCache
	plugins []Plugin
}

func NewQueryBuilder(client *Client, cache QueryCache, plugins ...Plugin) *QueryBuilderImpl {
	return &QueryBuilderImpl{
		client:  client,
		cache:   cache,
		plugins: plugins,
	}
}

type QueryBuilderOptions struct {
	Query      string
	Variables  map[string]any
	Extensions map[string]any
	Headers    map[string]string
	CacheKey   string
}

func (qb *QueryBuilderImpl) Build(name string, variables map[string]any) *QueryBuilderOptions {
	return &QueryBuilderOptions{
		Query:      name,
		Variables:  variables,
		Extensions: make(map[string]any),
		Headers:    make(map[string]string),
		CacheKey:   CacheKey(name, variables),
	}
}

func (qb *QueryBuilderImpl) Execute(ctx context.Context, opts *QueryBuilderOptions) (map[string]any, error) {
	cacheKey := ""
	if opts != nil {
		cacheKey = opts.CacheKey
	}

	if qb.cache != nil && cacheKey != "" {
		if result, ok := qb.cache.Get(cacheKey); ok {
			if result.Data != nil {
				if data, ok := result.Data.(map[string]any); ok {
					return data, nil
				}
			}
		}
	}

	if opts == nil {
		opts = &QueryBuilderOptions{}
	}

	queryOpts := QueryOptions{
		Query:      opts.Query,
		Variables:  opts.Variables,
		Extensions: opts.Extensions,
	}

	for _, plugin := range qb.plugins {
		var err error
		queryOpts, err = plugin.BeforeQuery(ctx, queryOpts)
		if err != nil {
			return nil, err
		}
	}

	result, err := qb.client.Query(ctx, queryOpts)
	if err != nil {
		return nil, err
	}

	for _, plugin := range qb.plugins {
		if err := plugin.AfterQuery(ctx, result, nil); err != nil {
			return nil, err
		}
	}

	if qb.cache != nil && cacheKey != "" && result.Data != nil {
		qb.cache.Set(cacheKey, result)
	}

	if result.Data != nil {
		if data, ok := result.Data.(map[string]any); ok {
			return data, nil
		}
	}

	return make(map[string]any), nil
}

type Subscription struct {
	client    *Client
	query     string
	variables map[string]any
}

func NewSubscription(client *Client, query string, variables map[string]any) *Subscription {
	return &Subscription{
		client:    client,
		query:     query,
		variables: variables,
	}
}

func (s *Subscription) Next(ctx context.Context) (map[string]any, error) {
	result, err := s.client.Query(ctx, QueryOptions{
		Query:     s.query,
		Variables: s.variables,
	})
	if err != nil {
		return nil, err
	}

	if result.Data != nil {
		if data, ok := result.Data.(map[string]any); ok {
			return data, nil
		}
	}

	return make(map[string]any), nil
}
