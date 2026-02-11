package client

type Client struct {
	network string
	mvr     *MVRClient
	cache   *ClientCache
}

type ClientOptions struct {
	Network string
	BaseURL string
	Mvr     MvrOptions
	Cache   bool
}

func NewClient(opts ClientOptions) (*Client, error) {
	cache := NewClientCache()
	if !opts.Cache {
		cache = nil
	}

	mvrClient := NewMvrClient(opts.Network, opts.Mvr)

	return &Client{
		network: opts.Network,
		mvr:     mvrClient,
		cache:   cache,
	}, nil
}

func (c *Client) Mvr() *MVRClient {
	return c.mvr
}

func (c *Client) Close() error {
	if c.mvr != nil {
		return c.mvr.Close()
	}
	return nil
}
