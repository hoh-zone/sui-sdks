package jsonrpc

import (
	"context"
	"encoding/hex"
	"fmt"
	"strings"

	"github.com/sui-sdks/go-sdks/bcs"
)

type PaginationArguments struct {
	Cursor any
	Limit  *int
}

type OrderArguments struct {
	Order *string
}

type ClientOptions struct {
	Network   string
	URL       string
	Transport Transport
}

type Client struct {
	network   string
	transport Transport
}

func NewClient(opts ClientOptions) (*Client, error) {
	transport := opts.Transport
	if transport == nil {
		url := opts.URL
		if url == "" {
			var err error
			url, err = GetJSONRPCFullnodeURL(opts.Network)
			if err != nil {
				return nil, err
			}
		}
		transport = NewHTTPTransport(HTTPTransportOptions{URL: url})
	}
	return &Client{network: opts.Network, transport: transport}, nil
}

func (c *Client) Network() string { return c.network }

func (c *Client) GetRPCAPIVersion(ctx context.Context) (string, error) {
	var out struct {
		Info struct {
			Version string `json:"version"`
		} `json:"info"`
	}
	if err := c.transport.Request(TransportRequest{Method: "rpc.discover", Params: []any{}, Ctx: ctx}, &out); err != nil {
		return "", err
	}
	return out.Info.Version, nil
}

func (c *Client) Call(ctx context.Context, method string, params []any, out any) error {
	return c.transport.Request(TransportRequest{Method: method, Params: params, Ctx: ctx}, out)
}

func (c *Client) GetCoins(ctx context.Context, owner, coinType string, cursor any, limit *int) (map[string]any, error) {
	if !isValidSuiAddress(owner) {
		return nil, fmt.Errorf("invalid Sui address")
	}
	var out map[string]any
	err := c.Call(ctx, "suix_getCoins", []any{owner, emptyToNil(coinType), cursor, intOrNil(limit)}, &out)
	return out, err
}

func (c *Client) GetAllCoins(ctx context.Context, owner string, cursor any, limit *int) (map[string]any, error) {
	if !isValidSuiAddress(owner) {
		return nil, fmt.Errorf("invalid Sui address")
	}
	var out map[string]any
	err := c.Call(ctx, "suix_getAllCoins", []any{owner, cursor, intOrNil(limit)}, &out)
	return out, err
}

func (c *Client) GetBalance(ctx context.Context, owner, coinType string) (map[string]any, error) {
	if !isValidSuiAddress(owner) {
		return nil, fmt.Errorf("invalid Sui address")
	}
	var out map[string]any
	err := c.Call(ctx, "suix_getBalance", []any{owner, emptyToNil(coinType)}, &out)
	return out, err
}

func (c *Client) GetAllBalances(ctx context.Context, owner string) ([]map[string]any, error) {
	if !isValidSuiAddress(owner) {
		return nil, fmt.Errorf("invalid Sui address")
	}
	var out []map[string]any
	err := c.Call(ctx, "suix_getAllBalances", []any{owner}, &out)
	return out, err
}

func (c *Client) GetCoinMetadata(ctx context.Context, coinType string) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getCoinMetadata", []any{coinType}, &out)
	return out, err
}

func (c *Client) GetTotalSupply(ctx context.Context, coinType string) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getTotalSupply", []any{coinType}, &out)
	return out, err
}

func (c *Client) GetObject(ctx context.Context, objectID string, options map[string]any) (map[string]any, error) {
	if !isValidSuiObjectID(objectID) {
		return nil, fmt.Errorf("invalid Sui object id")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_getObject", []any{objectID, options}, &out)
	return out, err
}

func (c *Client) MultiGetObjects(ctx context.Context, objectIDs []string, options map[string]any) ([]map[string]any, error) {
	for _, id := range objectIDs {
		if !isValidSuiObjectID(id) {
			return nil, fmt.Errorf("invalid Sui object id: %s", id)
		}
	}
	var out []map[string]any
	err := c.Call(ctx, "sui_multiGetObjects", []any{objectIDs, options}, &out)
	return out, err
}

func (c *Client) GetTransactionBlock(ctx context.Context, digest string, options map[string]any) (map[string]any, error) {
	if !isValidTransactionDigest(digest) {
		return nil, fmt.Errorf("invalid transaction digest")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_getTransactionBlock", []any{digest, options}, &out)
	return out, err
}

func (c *Client) ExecuteTransactionBlock(ctx context.Context, txBytesBase64 string, signatures []string, options map[string]any, requestType string) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "sui_executeTransactionBlock", []any{txBytesBase64, signatures, options, requestType}, &out)
	return out, err
}

func (c *Client) GetReferenceGasPrice(ctx context.Context) (string, error) {
	var out string
	err := c.Call(ctx, "suix_getReferenceGasPrice", []any{}, &out)
	return out, err
}

func (c *Client) QueryTransactionBlocks(ctx context.Context, query map[string]any, cursor any, limit *int, descendingOrder bool) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_queryTransactionBlocks", []any{query, cursor, intOrNil(limit), descendingOrder}, &out)
	return out, err
}

func (c *Client) QueryEvents(ctx context.Context, query map[string]any, cursor any, limit *int, descendingOrder bool) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_queryEvents", []any{query, cursor, intOrNil(limit), descendingOrder}, &out)
	return out, err
}

func (c *Client) DryRunTransactionBlock(ctx context.Context, txBytesBase64 string) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "sui_dryRunTransactionBlock", []any{txBytesBase64}, &out)
	return out, err
}

func (c *Client) DevInspectTransactionBlock(ctx context.Context, sender string, txBytesBase64 string, gasPrice *string, epoch *string) (map[string]any, error) {
	if !isValidSuiAddress(sender) {
		return nil, fmt.Errorf("invalid Sui address")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_devInspectTransactionBlock", []any{sender, txBytesBase64, gasPrice, epoch}, &out)
	return out, err
}

func (c *Client) MultiGetTransactionBlocks(ctx context.Context, digests []string, options map[string]any) ([]map[string]any, error) {
	for _, d := range digests {
		if !isValidTransactionDigest(d) {
			return nil, fmt.Errorf("invalid transaction digest: %s", d)
		}
	}
	var out []map[string]any
	err := c.Call(ctx, "sui_multiGetTransactionBlocks", []any{digests, options}, &out)
	return out, err
}

func (c *Client) GetTotalTransactionBlocks(ctx context.Context) (string, error) {
	var out string
	err := c.Call(ctx, "sui_getTotalTransactionBlocks", []any{}, &out)
	return out, err
}

func (c *Client) TryGetPastObject(ctx context.Context, objectID string, version string, options map[string]any) (map[string]any, error) {
	if !isValidSuiObjectID(objectID) {
		return nil, fmt.Errorf("invalid Sui object id")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_tryGetPastObject", []any{objectID, version, options}, &out)
	return out, err
}

func (c *Client) GetLatestCheckpointSequenceNumber(ctx context.Context) (string, error) {
	var out string
	err := c.Call(ctx, "sui_getLatestCheckpointSequenceNumber", []any{}, &out)
	return out, err
}

func (c *Client) GetCheckpoint(ctx context.Context, checkpointID string) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "sui_getCheckpoint", []any{checkpointID}, &out)
	return out, err
}

func (c *Client) GetCheckpoints(ctx context.Context, cursor any, limit *int, descendingOrder bool) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "sui_getCheckpoints", []any{cursor, intOrNil(limit), descendingOrder}, &out)
	return out, err
}

func (c *Client) GetCommitteeInfo(ctx context.Context, epoch *string) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getCommitteeInfo", []any{epoch}, &out)
	return out, err
}

func (c *Client) GetNetworkMetrics(ctx context.Context) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getNetworkMetrics", []any{}, &out)
	return out, err
}

func (c *Client) GetLatestAddressMetrics(ctx context.Context) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getLatestAddressMetrics", []any{}, &out)
	return out, err
}

func (c *Client) GetEpochMetrics(ctx context.Context, cursor any, limit *int, descendingOrder bool) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getEpochMetrics", []any{cursor, intOrNil(limit), descendingOrder}, &out)
	return out, err
}

func (c *Client) GetAllEpochAddressMetrics(ctx context.Context, descendingOrder bool) ([]map[string]any, error) {
	var out []map[string]any
	err := c.Call(ctx, "suix_getAllEpochAddressMetrics", []any{descendingOrder}, &out)
	return out, err
}

func (c *Client) GetEpochs(ctx context.Context, cursor any, limit *int, descendingOrder bool) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getEpochs", []any{cursor, intOrNil(limit), descendingOrder}, &out)
	return out, err
}

func (c *Client) GetMoveCallMetrics(ctx context.Context) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getMoveCallMetrics", []any{}, &out)
	return out, err
}

func (c *Client) GetCurrentEpoch(ctx context.Context) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getCurrentEpoch", []any{}, &out)
	return out, err
}

func (c *Client) GetValidatorsApy(ctx context.Context) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "suix_getValidatorsApy", []any{}, &out)
	return out, err
}

func (c *Client) GetStakes(ctx context.Context, owner string) ([]map[string]any, error) {
	if !isValidSuiAddress(owner) {
		return nil, fmt.Errorf("invalid Sui address")
	}
	var out []map[string]any
	err := c.Call(ctx, "suix_getStakes", []any{owner}, &out)
	return out, err
}

func (c *Client) GetStakesByIds(ctx context.Context, stakedSuiIds []string) ([]map[string]any, error) {
	for _, id := range stakedSuiIds {
		if !isValidSuiObjectID(id) {
			return nil, fmt.Errorf("invalid Sui object id: %s", id)
		}
	}
	var out []map[string]any
	err := c.Call(ctx, "suix_getStakesByIds", []any{stakedSuiIds}, &out)
	return out, err
}

func (c *Client) ResolveNameServiceAddress(ctx context.Context, name string) (string, error) {
	var out string
	err := c.Call(ctx, "suix_resolveNameServiceAddress", []any{name}, &out)
	return out, err
}

func (c *Client) ResolveNameServiceNames(ctx context.Context, address string, cursor any, limit *int) (map[string]any, error) {
	if !isValidSuiAddress(address) {
		return nil, fmt.Errorf("invalid Sui address")
	}
	var out map[string]any
	err := c.Call(ctx, "suix_resolveNameServiceNames", []any{address, cursor, intOrNil(limit)}, &out)
	return out, err
}

func (c *Client) GetProtocolConfig(ctx context.Context, version *string) (map[string]any, error) {
	var out map[string]any
	err := c.Call(ctx, "sui_getProtocolConfig", []any{version}, &out)
	return out, err
}

func (c *Client) GetNormalizedMoveModulesByPackage(ctx context.Context, packageID string) (map[string]any, error) {
	if !isValidSuiObjectID(packageID) && !isValidNamedPackage(packageID) {
		return nil, fmt.Errorf("invalid package id")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_getNormalizedMoveModulesByPackage", []any{packageID}, &out)
	return out, err
}

func (c *Client) GetNormalizedMoveModule(ctx context.Context, packageID, moduleName string) (map[string]any, error) {
	if !isValidSuiObjectID(packageID) && !isValidNamedPackage(packageID) {
		return nil, fmt.Errorf("invalid package id")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_getNormalizedMoveModule", []any{packageID, moduleName}, &out)
	return out, err
}

func (c *Client) GetNormalizedMoveFunction(ctx context.Context, packageID, moduleName, functionName string) (map[string]any, error) {
	if !isValidSuiObjectID(packageID) && !isValidNamedPackage(packageID) {
		return nil, fmt.Errorf("invalid package id")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_getNormalizedMoveFunction", []any{packageID, moduleName, functionName}, &out)
	return out, err
}

func (c *Client) GetMoveFunctionArgTypes(ctx context.Context, packageID, moduleName, functionName string) (map[string]any, error) {
	if !isValidSuiObjectID(packageID) && !isValidNamedPackage(packageID) {
		return nil, fmt.Errorf("invalid package id")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_getMoveFunctionArgTypes", []any{packageID, moduleName, functionName}, &out)
	return out, err
}

func (c *Client) GetNormalizedMoveStruct(ctx context.Context, packageID, moduleName, structName string) (map[string]any, error) {
	if !isValidSuiObjectID(packageID) && !isValidNamedPackage(packageID) {
		return nil, fmt.Errorf("invalid package id")
	}
	var out map[string]any
	err := c.Call(ctx, "sui_getNormalizedMoveStruct", []any{packageID, moduleName, structName}, &out)
	return out, err
}

func emptyToNil(v string) any {
	if strings.TrimSpace(v) == "" {
		return nil
	}
	return v
}

func intOrNil(v *int) any {
	if v == nil {
		return nil
	}
	return *v
}

func normalizeHexAddress(addr string) string {
	addr = strings.TrimSpace(strings.ToLower(addr))
	if strings.HasPrefix(addr, "0x") {
		addr = addr[2:]
	}
	return addr
}

func isValidSuiAddress(addr string) bool {
	a := normalizeHexAddress(addr)
	if len(a) == 0 || len(a) > 64 {
		return false
	}
	_, err := hex.DecodeString(a)
	return err == nil
}

func isValidSuiObjectID(id string) bool {
	return isValidSuiAddress(id)
}

func isValidTransactionDigest(digest string) bool {
	if digest == "" {
		return false
	}
	_, err := bcs.FromBase58(digest)
	return err == nil
}

func isValidNamedPackage(name string) bool {
	parts := strings.Split(name, "/")
	if len(parts) < 2 || len(parts) > 3 {
		return false
	}
	org, app := parts[0], parts[1]
	if org == "" || app == "" {
		return false
	}
	for _, c := range org {
		if !((c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')) {
			return false
		}
	}
	for _, c := range app {
		if !((c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '-') {
			return false
		}
	}
	if len(parts) == 3 {
		if parts[2] == "" {
			return false
		}
		for _, c := range parts[2] {
			if c < '0' || c > '9' {
				return false
			}
		}
	}
	return true
}
