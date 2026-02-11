package deepbookv3

import (
	"context"
	"encoding/base64"
	"fmt"
	"math/big"
	"time"

	"github.com/sui-sdks/go-sdks/bcs"
	"github.com/sui-sdks/go-sdks/deepbook_v3/pyth"
	"github.com/sui-sdks/go-sdks/deepbook_v3/transactions"
	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	stx "github.com/sui-sdks/go-sdks/sui/transactions"
	suiutils "github.com/sui-sdks/go-sdks/sui/utils"
)

type CompatibleClient interface {
	Call(ctx context.Context, method string, params []any, out any) error
	Network() string
}

type Options struct {
	Address             string
	BalanceManagers     map[string]types.BalanceManager
	MarginManagers      map[string]types.MarginManager
	Coins               utils.CoinMap
	Pools               utils.PoolMap
	AdminCap            string
	MarginAdminCap      string
	MarginMaintainerCap string
}

type ClientOptions struct {
	Client  CompatibleClient
	Network string
	Options
}

type Client struct {
	client             CompatibleClient
	config             *utils.DeepBookConfig
	Address            string
	BalanceManager     *transactions.BalanceManagerContract
	DeepBook           *transactions.DeepBookContract
	DeepBookAdmin      *transactions.DeepBookAdminContract
	FlashLoans         *transactions.FlashLoanContract
	Governance         *transactions.GovernanceContract
	MarginAdmin        *transactions.MarginAdminContract
	MarginMaintainer   *transactions.MarginMaintainerContract
	MarginPool         *transactions.MarginPoolContract
	MarginManager      *transactions.MarginManagerContract
	MarginRegistry     *transactions.MarginRegistryContract
	MarginLiquidations *transactions.MarginLiquidationsContract
	PoolProxy          *transactions.PoolProxyContract
	MarginTPSL         *transactions.MarginTPSLContract
}

func NewClient(opts ClientOptions) *Client {
	address := suiutils.NormalizeSuiAddress(opts.Address)
	config := utils.NewDeepBookConfig(utils.ConfigOptions{
		Address:             address,
		Network:             opts.Network,
		BalanceManagers:     opts.BalanceManagers,
		MarginManagers:      opts.MarginManagers,
		Coins:               opts.Coins,
		Pools:               opts.Pools,
		AdminCap:            opts.AdminCap,
		MarginAdminCap:      opts.MarginAdminCap,
		MarginMaintainerCap: opts.MarginMaintainerCap,
	})
	balanceManager := transactions.NewBalanceManagerContract(config)
	return &Client{
		client:             opts.Client,
		config:             config,
		Address:            address,
		BalanceManager:     balanceManager,
		DeepBook:           transactions.NewDeepBookContract(config, balanceManager),
		DeepBookAdmin:      transactions.NewDeepBookAdminContract(config),
		FlashLoans:         transactions.NewFlashLoanContract(config),
		Governance:         transactions.NewGovernanceContract(config, balanceManager),
		MarginAdmin:        transactions.NewMarginAdminContract(config),
		MarginMaintainer:   transactions.NewMarginMaintainerContract(config),
		MarginPool:         transactions.NewMarginPoolContract(config),
		MarginManager:      transactions.NewMarginManagerContract(config),
		MarginRegistry:     transactions.NewMarginRegistryContract(config),
		MarginLiquidations: transactions.NewMarginLiquidationsContract(config),
		PoolProxy:          transactions.NewPoolProxyContract(config),
		MarginTPSL:         transactions.NewMarginTPSLContract(config),
	}
}

func (c *Client) Config() *utils.DeepBookConfig { return c.config }

func (c *Client) CheckManagerBalance(ctx context.Context, managerKey, coinKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.BalanceManager.CheckManagerBalance(tx, managerKey, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	v, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	coin := c.config.GetCoin(coinKey)
	return map[string]any{
		"coinType": coin.Type,
		"balance":  float64(v) / coin.Scalar,
	}, nil
}

func (c *Client) Whitelisted(ctx context.Context, poolKey string) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.Whitelisted(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	b, err := extractReturnBCS(res, 0, 0)
	if err != nil {
		return false, err
	}
	return len(b) > 0 && b[0] == 1, nil
}

func (c *Client) GetQuoteQuantityOut(ctx context.Context, poolKey string, baseQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetQuoteQuantityOut(tx, poolKey, baseQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseOut, _ := readU64(res, 0, 0)
	quoteOut, _ := readU64(res, 0, 1)
	deepRequired, _ := readU64(res, 0, 2)
	return map[string]any{
		"baseQuantity": baseQuantity,
		"baseOut":      float64(baseOut) / baseScalar,
		"quoteOut":     float64(quoteOut) / quoteScalar,
		"deepRequired": float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetBaseQuantityOut(ctx context.Context, poolKey string, quoteQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetBaseQuantityOut(tx, poolKey, quoteQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseOut, _ := readU64(res, 0, 0)
	quoteOut, _ := readU64(res, 0, 1)
	deepRequired, _ := readU64(res, 0, 2)
	return map[string]any{
		"quoteQuantity": quoteQuantity,
		"baseOut":       float64(baseOut) / baseScalar,
		"quoteOut":      float64(quoteOut) / quoteScalar,
		"deepRequired":  float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetQuantityOut(ctx context.Context, poolKey string, baseQuantity, quoteQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetQuantityOut(tx, poolKey, baseQuantity, quoteQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseOut, _ := readU64(res, 0, 0)
	quoteOut, _ := readU64(res, 0, 1)
	deepRequired, _ := readU64(res, 0, 2)
	return map[string]any{
		"baseQuantity":  baseQuantity,
		"quoteQuantity": quoteQuantity,
		"baseOut":       float64(baseOut) / baseScalar,
		"quoteOut":      float64(quoteOut) / quoteScalar,
		"deepRequired":  float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) MidPrice(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	c.DeepBook.MidPrice(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	v, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(v) * base.Scalar / (utils.FloatScalar * quote.Scalar), nil
}

func (c *Client) GetOrder(ctx context.Context, poolKey, orderID string) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetOrder(tx, poolKey, orderID)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	bytes, err := extractReturnBCS(res, 0, 0)
	if err != nil {
		return nil, err
	}
	return parseOrder(bytes)
}

func (c *Client) GetOrderRaw(ctx context.Context, poolKey, orderID string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetOrder(tx, poolKey, orderID)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) GetOrderNormalized(ctx context.Context, poolKey, orderID string) (map[string]any, error) {
	order, err := c.GetOrder(ctx, poolKey, orderID)
	if err != nil {
		return nil, err
	}
	pool := c.config.GetPool(poolKey)
	baseCoin := c.config.GetCoin(pool.BaseCoin)
	quoteCoin := c.config.GetCoin(pool.QuoteCoin)
	isBid, rawPrice, _ := decodeOrderID(order["order_id"].(string))
	normPrice := float64(rawPrice) * baseCoin.Scalar / (quoteCoin.Scalar * utils.FloatScalar)
	order["isBid"] = isBid
	order["normalized_price"] = fmt.Sprintf("%.9f", normPrice)
	order["quantity"] = formatTokenAmount(order["quantity_raw"].(uint64), baseCoin.Scalar, 9)
	order["filled_quantity"] = formatTokenAmount(order["filled_quantity_raw"].(uint64), baseCoin.Scalar, 9)
	if odp, ok := order["order_deep_price"].(map[string]any); ok {
		if raw, ok := odp["deep_per_asset_raw"].(uint64); ok {
			odp["deep_per_asset"] = fmt.Sprintf("%.9f", float64(raw)/utils.DeepScalar)
		}
	}
	return order, nil
}

func (c *Client) GetOrders(ctx context.Context, poolKey string, orderIDs []string) ([]map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetOrders(tx, poolKey, orderIDs)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	bytes, err := extractReturnBCS(res, 0, 0)
	if err != nil {
		return nil, err
	}
	return parseVecOrders(bytes)
}

func (c *Client) AccountOpenOrders(ctx context.Context, poolKey, managerKey string) ([]string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.AccountOpenOrders(tx, poolKey, managerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	return readVecU128String(res, 0, 0)
}

func (c *Client) VaultBalances(ctx context.Context, poolKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.VaultBalances(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseInVault, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteInVault, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepInVault, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"base":  float64(baseInVault) / baseScalar,
		"quote": float64(quoteInVault) / quoteScalar,
		"deep":  float64(deepInVault) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetPoolIDByAssets(ctx context.Context, baseType, quoteType string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetPoolIDByAssets(tx, baseType, quoteType)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) GetPoolIdByAssets(ctx context.Context, baseType, quoteType string) (string, error) {
	return c.GetPoolIDByAssets(ctx, baseType, quoteType)
}

func (c *Client) PoolTradeParams(ctx context.Context, poolKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.PoolTradeParams(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	takerFee, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	makerFee, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	stakeRequired, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"takerFee":      float64(takerFee) / utils.FloatScalar,
		"makerFee":      float64(makerFee) / utils.FloatScalar,
		"stakeRequired": float64(stakeRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) PoolBookParams(ctx context.Context, poolKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.PoolBookParams(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	tickSize, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	lotSize, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	minSize, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"tickSize": float64(tickSize) * baseScalar / (quoteScalar * utils.FloatScalar),
		"lotSize":  float64(lotSize) / baseScalar,
		"minSize":  float64(minSize) / baseScalar,
	}, nil
}

func (c *Client) Account(ctx context.Context, poolKey, managerKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.Account(tx, poolKey, managerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	bytes, err := extractReturnBCS(res, 0, 0)
	if err != nil {
		return nil, err
	}
	return parseAccount(bytes, baseScalar, quoteScalar)
}

func (c *Client) LockedBalance(ctx context.Context, poolKey, balanceManagerKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.LockedBalance(tx, poolKey, balanceManagerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseLocked, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteLocked, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepLocked, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"base":  float64(baseLocked) / baseScalar,
		"quote": float64(quoteLocked) / quoteScalar,
		"deep":  float64(deepLocked) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetPoolDeepPrice(ctx context.Context, poolKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseCoin := c.config.GetCoin(pool.BaseCoin)
	quoteCoin := c.config.GetCoin(pool.QuoteCoin)
	deepCoin := c.config.GetCoin("DEEP")
	c.DeepBook.GetPoolDeepPrice(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	bytes, err := extractReturnBCS(res, 0, 0)
	if err != nil {
		return nil, err
	}
	reader := bcs.NewReader(bytes)
	assetIsBaseByte, err := reader.Read8()
	if err != nil {
		return nil, err
	}
	deepPerAsset, err := reader.Read64()
	if err != nil {
		return nil, err
	}
	assetIsBase := assetIsBaseByte == 1
	if assetIsBase {
		return map[string]any{
			"asset_is_base":  true,
			"deep_per_base":  (float64(deepPerAsset) / utils.FloatScalar) * baseCoin.Scalar / deepCoin.Scalar,
			"deep_per_quote": nil,
		}, nil
	}
	return map[string]any{
		"asset_is_base":  false,
		"deep_per_quote": (float64(deepPerAsset) / utils.FloatScalar) * quoteCoin.Scalar / deepCoin.Scalar,
		"deep_per_base":  nil,
	}, nil
}

func (c *Client) BalanceManagerReferralOwner(ctx context.Context, referral string) (string, error) {
	tx := stx.NewTransaction()
	c.BalanceManager.BalanceManagerReferralOwner(tx, referral)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) BalanceManagerReferralPoolID(ctx context.Context, referral string) (string, error) {
	tx := stx.NewTransaction()
	c.BalanceManager.BalanceManagerReferralPoolID(tx, referral)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) BalanceManagerReferralPoolId(ctx context.Context, referral string) (string, error) {
	return c.BalanceManagerReferralPoolID(ctx, referral)
}

func (c *Client) GetBalanceManagerReferralID(ctx context.Context, managerKey, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.BalanceManager.GetBalanceManagerReferralID(tx, managerKey, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	addr, ok, err := readOptionAddress(res, 0, 0)
	if err != nil || !ok {
		return "", err
	}
	return addr, nil
}

func (c *Client) GetBalanceManagerReferralId(ctx context.Context, managerKey, poolKey string) (string, error) {
	return c.GetBalanceManagerReferralID(ctx, managerKey, poolKey)
}

func (c *Client) GetBalanceManagerIDs(ctx context.Context, owner string) ([]string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetBalanceManagerIDs(tx, owner)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	return readVecAddress(res, 0, 0)
}

func (c *Client) GetBalanceManagerIds(ctx context.Context, owner string) ([]string, error) {
	return c.GetBalanceManagerIDs(ctx, owner)
}

func (c *Client) GetPoolReferralBalances(ctx context.Context, poolKey, referral string) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetPoolReferralBalances(tx, poolKey, referral)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseBal, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteBal, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepBal, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"base":  float64(baseBal) / baseScalar,
		"quote": float64(quoteBal) / quoteScalar,
		"deep":  float64(deepBal) / utils.DeepScalar,
	}, nil
}

func (c *Client) PoolReferralMultiplier(ctx context.Context, poolKey, referral string) (float64, error) {
	tx := stx.NewTransaction()
	c.DeepBook.PoolReferralMultiplier(tx, poolKey, referral)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) StablePool(ctx context.Context, poolKey string) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.StablePool(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) RegisteredPool(ctx context.Context, poolKey string) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.RegisteredPool(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) GetPriceInfoObjectAge(_ context.Context, coinKey string) (int64, error) {
	coin := c.config.GetCoin(coinKey)
	if coin.PriceInfoObjectID == "" {
		return -1, nil
	}
	return time.Now().UnixMilli(), nil
}

func (c *Client) GetPriceInfoObject(_ context.Context, coinKey string) (string, error) {
	coin := c.config.GetCoin(coinKey)
	if coin.PriceInfoObjectID == "" {
		return "", fmt.Errorf("price info object not found for %s", coinKey)
	}
	return suiutils.NormalizeSuiAddress(coin.PriceInfoObjectID), nil
}

func (c *Client) GetPriceInfoObjects(ctx context.Context, coinKeys []string) (map[string]string, error) {
	out := make(map[string]string, len(coinKeys))
	for _, coinKey := range coinKeys {
		id, err := c.GetPriceInfoObject(ctx, coinKey)
		if err != nil {
			return nil, err
		}
		out[coinKey] = id
	}
	return out, nil
}

func (c *Client) GetMarginAccountOrderDetails(ctx context.Context, marginManagerKey string) ([]map[string]any, error) {
	tx := stx.NewTransaction()
	c.MarginManager.GetMarginAccountOrderDetails(tx, marginManagerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	bytes, err := extractReturnBCS(res, 1, 0)
	if err != nil {
		return nil, err
	}
	return parseVecOrders(bytes)
}

func (c *Client) GetQuoteQuantityOutInputFee(ctx context.Context, poolKey string, baseQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetQuoteQuantityOutInputFee(tx, poolKey, baseQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseOut, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteOut, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepRequired, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"baseQuantity": baseQuantity,
		"baseOut":      float64(baseOut) / baseScalar,
		"quoteOut":     float64(quoteOut) / quoteScalar,
		"deepRequired": float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetBaseQuantityOutInputFee(ctx context.Context, poolKey string, quoteQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetBaseQuantityOutInputFee(tx, poolKey, quoteQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseOut, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteOut, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepRequired, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"quoteQuantity": quoteQuantity,
		"baseOut":       float64(baseOut) / baseScalar,
		"quoteOut":      float64(quoteOut) / quoteScalar,
		"deepRequired":  float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetQuantityOutInputFee(ctx context.Context, poolKey string, baseQuantity, quoteQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetQuantityOutInputFee(tx, poolKey, baseQuantity, quoteQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseOut, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteOut, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepRequired, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"baseQuantity":  baseQuantity,
		"quoteQuantity": quoteQuantity,
		"baseOut":       float64(baseOut) / baseScalar,
		"quoteOut":      float64(quoteOut) / quoteScalar,
		"deepRequired":  float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetBaseQuantityIn(ctx context.Context, poolKey string, targetQuoteQuantity float64, payWithDeep bool) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetBaseQuantityIn(tx, poolKey, targetQuoteQuantity, payWithDeep)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseIn, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteOut, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepRequired, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"baseIn":       float64(baseIn) / baseScalar,
		"quoteOut":     float64(quoteOut) / quoteScalar,
		"deepRequired": float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetQuoteQuantityIn(ctx context.Context, poolKey string, targetBaseQuantity float64, payWithDeep bool) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseScalar := c.config.GetCoin(pool.BaseCoin).Scalar
	quoteScalar := c.config.GetCoin(pool.QuoteCoin).Scalar
	c.DeepBook.GetQuoteQuantityIn(tx, poolKey, targetBaseQuantity, payWithDeep)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseOut, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteIn, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	deepRequired, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"baseOut":      float64(baseOut) / baseScalar,
		"quoteIn":      float64(quoteIn) / quoteScalar,
		"deepRequired": float64(deepRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) GetAccountOrderDetails(ctx context.Context, poolKey, managerKey string) ([]map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetAccountOrderDetails(tx, poolKey, managerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	bytes, err := extractReturnBCS(res, 0, 0)
	if err != nil {
		return nil, err
	}
	return parseVecOrders(bytes)
}

func (c *Client) GetOrderDeepRequired(ctx context.Context, poolKey string, baseQuantity, price float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetOrderDeepRequired(tx, poolKey, baseQuantity, price)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	deepRequiredTaker, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	deepRequiredMaker, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"deepRequiredTaker": float64(deepRequiredTaker) / utils.DeepScalar,
		"deepRequiredMaker": float64(deepRequiredMaker) / utils.DeepScalar,
	}, nil
}

func (c *Client) PoolTradeParamsNext(ctx context.Context, poolKey string) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.PoolTradeParamsNext(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	takerFee, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	makerFee, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	stakeRequired, err := readU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"takerFee":      float64(takerFee) / utils.FloatScalar,
		"makerFee":      float64(makerFee) / utils.FloatScalar,
		"stakeRequired": float64(stakeRequired) / utils.DeepScalar,
	}, nil
}

func (c *Client) AccountExists(ctx context.Context, poolKey, managerKey string) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.AccountExists(tx, poolKey, managerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) Quorum(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.DeepBook.Quorum(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.DeepScalar, nil
}

func (c *Client) PoolID(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.PoolID(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) PoolId(ctx context.Context, poolKey string) (string, error) {
	return c.PoolID(ctx, poolKey)
}

func (c *Client) CanPlaceLimitOrder(ctx context.Context, params types.CanPlaceLimitOrderParams) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.CanPlaceLimitOrder(tx, params)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) CanPlaceMarketOrder(ctx context.Context, params types.CanPlaceMarketOrderParams) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.CanPlaceMarketOrder(tx, params)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) CheckMarketOrderParams(ctx context.Context, poolKey string, quantity float64) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.CheckMarketOrderParams(tx, poolKey, quantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) CheckLimitOrderParams(ctx context.Context, poolKey string, price, quantity float64, expireTimestamp uint64) (bool, error) {
	tx := stx.NewTransaction()
	c.DeepBook.CheckLimitOrderParams(tx, poolKey, price, quantity, expireTimestamp)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) GetMarginPoolID(ctx context.Context, coinKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.GetID(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) GetMarginPoolId(ctx context.Context, coinKey string) (string, error) {
	return c.GetMarginPoolID(ctx, coinKey)
}

func (c *Client) IsPoolEnabledForMargin(ctx context.Context, poolKey string) (bool, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.PoolEnabled(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) GetMarginManagerIDsForOwner(ctx context.Context, owner string) ([]string, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.GetMarginManagerIDs(tx, owner)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	return readVecAddress(res, 0, 0)
}

func (c *Client) GetMarginManagerIdsForOwner(ctx context.Context, owner string) ([]string, error) {
	return c.GetMarginManagerIDsForOwner(ctx, owner)
}

func (c *Client) GetConditionalOrderIDs(ctx context.Context, marginManagerKey string) ([]string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginTPSL.ConditionalOrderIDs(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	return readVecU64String(res, 0, 0)
}

func (c *Client) GetConditionalOrderIds(ctx context.Context, marginManagerKey string) ([]string, error) {
	return c.GetConditionalOrderIDs(ctx, marginManagerKey)
}

func (c *Client) GetLowestTriggerAbovePrice(ctx context.Context, marginManagerKey string) (uint64, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginTPSL.LowestTriggerAbovePrice(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	return readU64(res, 0, 0)
}

func (c *Client) GetHighestTriggerBelowPrice(ctx context.Context, marginManagerKey string) (uint64, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginTPSL.HighestTriggerBelowPrice(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	return readU64(res, 0, 0)
}

func (c *Client) GetLevel2Range(ctx context.Context, poolKey string, priceLow, priceHigh float64, isBid bool) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseCoin := c.config.GetCoin(pool.BaseCoin)
	quoteCoin := c.config.GetCoin(pool.QuoteCoin)
	c.DeepBook.GetLevel2Range(tx, poolKey, priceLow, priceHigh, isBid)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	pricesRaw, err := readVecU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	qtysRaw, err := readVecU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	prices := make([]float64, 0, len(pricesRaw))
	for _, p := range pricesRaw {
		prices = append(prices, float64(p)*baseCoin.Scalar/(utils.FloatScalar*quoteCoin.Scalar))
	}
	quantities := make([]float64, 0, len(qtysRaw))
	for _, q := range qtysRaw {
		quantities = append(quantities, float64(q)/baseCoin.Scalar)
	}
	return map[string]any{
		"prices":     prices,
		"quantities": quantities,
	}, nil
}

func (c *Client) GetLevel2TicksFromMid(ctx context.Context, poolKey string, ticks uint64) (map[string]any, error) {
	tx := stx.NewTransaction()
	pool := c.config.GetPool(poolKey)
	baseCoin := c.config.GetCoin(pool.BaseCoin)
	quoteCoin := c.config.GetCoin(pool.QuoteCoin)
	c.DeepBook.GetLevel2TicksFromMid(tx, poolKey, ticks)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	bidPricesRaw, err := readVecU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	bidQtyRaw, err := readVecU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	askPricesRaw, err := readVecU64(res, 0, 2)
	if err != nil {
		return nil, err
	}
	askQtyRaw, err := readVecU64(res, 0, 3)
	if err != nil {
		return nil, err
	}
	bidPrices := make([]float64, 0, len(bidPricesRaw))
	for _, p := range bidPricesRaw {
		bidPrices = append(bidPrices, float64(p)*baseCoin.Scalar/(utils.FloatScalar*quoteCoin.Scalar))
	}
	bidQty := make([]float64, 0, len(bidQtyRaw))
	for _, q := range bidQtyRaw {
		bidQty = append(bidQty, float64(q)/baseCoin.Scalar)
	}
	askPrices := make([]float64, 0, len(askPricesRaw))
	for _, p := range askPricesRaw {
		askPrices = append(askPrices, float64(p)*baseCoin.Scalar/(utils.FloatScalar*quoteCoin.Scalar))
	}
	askQty := make([]float64, 0, len(askQtyRaw))
	for _, q := range askQtyRaw {
		askQty = append(askQty, float64(q)/baseCoin.Scalar)
	}
	return map[string]any{
		"bid_prices":     bidPrices,
		"bid_quantities": bidQty,
		"ask_prices":     askPrices,
		"ask_quantities": askQty,
	}, nil
}

func (c *Client) IsDeepbookPoolAllowed(ctx context.Context, coinKey, deepbookPoolID string) (bool, error) {
	tx := stx.NewTransaction()
	c.MarginPool.DeepbookPoolAllowed(tx, coinKey, deepbookPoolID)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) GetMarginPoolTotalSupply(ctx context.Context, coinKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.TotalSupply(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetMarginPoolSupplyShares(ctx context.Context, coinKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.SupplyShares(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetMarginPoolTotalBorrow(ctx context.Context, coinKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.TotalBorrow(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetMarginPoolBorrowShares(ctx context.Context, coinKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.BorrowShares(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetMarginPoolLastUpdateTimestamp(ctx context.Context, coinKey string) (uint64, error) {
	tx := stx.NewTransaction()
	c.MarginPool.LastUpdateTimestamp(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	return readU64(res, 0, 0)
}

func (c *Client) GetMarginPoolSupplyCap(ctx context.Context, coinKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.SupplyCap(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetMarginPoolMaxUtilizationRate(ctx context.Context, coinKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginPool.MaxUtilizationRate(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetMarginPoolProtocolSpread(ctx context.Context, coinKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginPool.ProtocolSpread(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetMarginPoolMinBorrow(ctx context.Context, coinKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.MinBorrow(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetMarginPoolInterestRate(ctx context.Context, coinKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginPool.InterestRate(tx, coinKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetUserSupplyShares(ctx context.Context, coinKey, supplierCapID string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.UserSupplyShares(tx, coinKey, supplierCapID)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetUserSupplyAmount(ctx context.Context, coinKey, supplierCapID string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginPool.UserSupplyAmount(tx, coinKey, supplierCapID)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(coinKey).Scalar, 6), nil
}

func (c *Client) GetBaseMarginPoolID(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.BaseMarginPoolID(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) GetBaseMarginPoolId(ctx context.Context, poolKey string) (string, error) {
	return c.GetBaseMarginPoolID(ctx, poolKey)
}

func (c *Client) GetQuoteMarginPoolID(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.QuoteMarginPoolID(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) GetQuoteMarginPoolId(ctx context.Context, poolKey string) (string, error) {
	return c.GetQuoteMarginPoolID(ctx, poolKey)
}

func (c *Client) GetMinWithdrawRiskRatio(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.MinWithdrawRiskRatio(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetMinBorrowRiskRatio(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.MinBorrowRiskRatio(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetLiquidationRiskRatio(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.LiquidationRiskRatio(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetTargetLiquidationRiskRatio(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.TargetLiquidationRiskRatio(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetUserLiquidationReward(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.UserLiquidationReward(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetPoolLiquidationReward(ctx context.Context, poolKey string) (float64, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.PoolLiquidationReward(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return 0, err
	}
	return float64(raw) / utils.FloatScalar, nil
}

func (c *Client) GetAllowedMaintainers(ctx context.Context) ([]string, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.AllowedMaintainers(tx)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	return readVecAddress(res, 0, 0)
}

func (c *Client) GetAllowedPauseCaps(ctx context.Context) ([]string, error) {
	tx := stx.NewTransaction()
	c.MarginRegistry.AllowedPauseCaps(tx)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	return readVecAddress(res, 0, 0)
}

func (c *Client) GetMarginManagerOwner(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.OwnerByPoolKey(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) GetMarginManagerDeepbookPool(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.DeepbookPool(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) GetMarginManagerMarginPoolID(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.MarginPoolID(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	addr, ok, err := readOptionAddress(res, 0, 0)
	if err != nil || !ok {
		return "", err
	}
	return addr, nil
}

func (c *Client) GetMarginManagerMarginPoolId(ctx context.Context, marginManagerKey string) (string, error) {
	return c.GetMarginManagerMarginPoolID(ctx, marginManagerKey)
}

func (c *Client) GetMarginManagerBorrowedShares(ctx context.Context, marginManagerKey string) (map[string]string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.BorrowedShares(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseShares, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteShares, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	return map[string]string{
		"baseShares":  fmt.Sprintf("%d", baseShares),
		"quoteShares": fmt.Sprintf("%d", quoteShares),
	}, nil
}

func (c *Client) GetMarginManagerBorrowedBaseShares(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.BorrowedBaseShares(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	v, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("%d", v), nil
}

func (c *Client) GetMarginManagerBorrowedQuoteShares(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.BorrowedQuoteShares(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	v, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("%d", v), nil
}

func (c *Client) GetMarginManagerHasBaseDebt(ctx context.Context, marginManagerKey string) (bool, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.HasBaseDebt(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return false, err
	}
	return readBool(res, 0, 0)
}

func (c *Client) GetMarginManagerBalanceManagerID(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.BalanceManagerByPoolKey(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readAddress(res, 0, 0)
}

func (c *Client) GetMarginManagerBalanceManagerId(ctx context.Context, marginManagerKey string) (string, error) {
	return c.GetMarginManagerBalanceManagerID(ctx, marginManagerKey)
}

func (c *Client) GetMarginManagerAssets(ctx context.Context, marginManagerKey string) (map[string]string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	tx := stx.NewTransaction()
	c.MarginManager.CalculateAssets(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseRaw, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteRaw, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	return map[string]string{
		"baseAsset":  formatTokenAmount(baseRaw, c.config.GetCoin(pool.BaseCoin).Scalar, 6),
		"quoteAsset": formatTokenAmount(quoteRaw, c.config.GetCoin(pool.QuoteCoin).Scalar, 6),
	}, nil
}

func (c *Client) GetMarginManagerDebts(ctx context.Context, marginManagerKey string) (map[string]string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	hasBaseDebt, err := c.GetMarginManagerHasBaseDebt(ctx, marginManagerKey)
	if err != nil {
		return nil, err
	}
	debtCoinKey := pool.QuoteCoin
	if hasBaseDebt {
		debtCoinKey = pool.BaseCoin
	}
	tx := stx.NewTransaction()
	c.MarginManager.CalculateDebts(tx, manager.PoolKey, debtCoinKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	baseRaw, err := readU64(res, 0, 0)
	if err != nil {
		return nil, err
	}
	quoteRaw, err := readU64(res, 0, 1)
	if err != nil {
		return nil, err
	}
	return map[string]string{
		"baseDebt":  formatTokenAmount(baseRaw, c.config.GetCoin(pool.BaseCoin).Scalar, 6),
		"quoteDebt": formatTokenAmount(quoteRaw, c.config.GetCoin(pool.QuoteCoin).Scalar, 6),
	}, nil
}

func (c *Client) GetMarginManagerBaseBalance(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	tx := stx.NewTransaction()
	c.MarginManager.BaseBalance(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(pool.BaseCoin).Scalar, 6), nil
}

func (c *Client) GetMarginManagerQuoteBalance(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	tx := stx.NewTransaction()
	c.MarginManager.QuoteBalance(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, c.config.GetCoin(pool.QuoteCoin).Scalar, 6), nil
}

func (c *Client) GetMarginManagerDeepBalance(ctx context.Context, marginManagerKey string) (string, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.DeepBalance(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	raw, err := readU64(res, 0, 0)
	if err != nil {
		return "", err
	}
	return formatTokenAmount(raw, utils.DeepScalar, 6), nil
}

func (c *Client) GetMarginManagerState(ctx context.Context, marginManagerKey string) (map[string]any, error) {
	manager := c.config.GetMarginManager(marginManagerKey)
	tx := stx.NewTransaction()
	c.MarginManager.ManagerState(tx, manager.PoolKey, manager.Address)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	return parseMarginManagerStateFromResult(res, 0, c.config.GetPool(manager.PoolKey), c.config)
}

func (c *Client) GetMarginManagerStates(ctx context.Context, marginManagerKeys []string) ([]map[string]any, error) {
	tx := stx.NewTransaction()
	pools := make([]types.Pool, 0, len(marginManagerKeys))
	for _, key := range marginManagerKeys {
		manager := c.config.GetMarginManager(key)
		pool := c.config.GetPool(manager.PoolKey)
		pools = append(pools, pool)
		c.MarginManager.ManagerState(tx, manager.PoolKey, manager.Address)
	}
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	out := make([]map[string]any, 0, len(marginManagerKeys))
	for i := range marginManagerKeys {
		state, err := parseMarginManagerStateFromResult(res, i, pools[i], c.config)
		if err != nil {
			return nil, err
		}
		out = append(out, state)
	}
	return out, nil
}

func (c *Client) GetPythClient(pythStateID, wormholeStateID string) *pyth.SuiPythClient {
	return pyth.NewSuiPythClient(c.client, pythStateID, wormholeStateID)
}

func (c *Client) simulate(ctx context.Context, tx *stx.Transaction) (map[string]any, error) {
	built, err := tx.BuildBase64()
	if err != nil {
		return nil, err
	}
	var out map[string]any
	err = c.client.Call(ctx, "sui_dryRunTransactionBlock", []any{built}, &out)
	return out, err
}

func readU64(res map[string]any, cmd, ret int) (uint64, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return 0, err
	}
	reader := bcs.NewReader(bytes)
	return reader.Read64()
}

func readBool(res map[string]any, cmd, ret int) (bool, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return false, err
	}
	return len(bytes) > 0 && bytes[0] == 1, nil
}

func readAddress(res map[string]any, cmd, ret int) (string, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return "", err
	}
	reader := bcs.NewReader(bytes)
	addrBytes, err := reader.ReadBytes(32)
	if err != nil {
		return "", err
	}
	return suiutils.NormalizeSuiAddress(fmt.Sprintf("0x%x", addrBytes)), nil
}

func readOptionAddress(res map[string]any, cmd, ret int) (string, bool, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return "", false, err
	}
	reader := bcs.NewReader(bytes)
	tag, err := reader.Read8()
	if err != nil {
		return "", false, err
	}
	if tag == 0 {
		return "", false, nil
	}
	if tag != 1 {
		return "", false, fmt.Errorf("invalid option tag: %d", tag)
	}
	addrBytes, err := reader.ReadBytes(32)
	if err != nil {
		return "", false, err
	}
	return suiutils.NormalizeSuiAddress(fmt.Sprintf("0x%x", addrBytes)), true, nil
}

func readVecAddress(res map[string]any, cmd, ret int) ([]string, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return nil, err
	}
	reader := bcs.NewReader(bytes)
	n, err := reader.ReadULEB()
	if err != nil {
		return nil, err
	}
	out := make([]string, 0, n)
	for i := uint64(0); i < n; i++ {
		addrBytes, err := reader.ReadBytes(32)
		if err != nil {
			return nil, err
		}
		out = append(out, suiutils.NormalizeSuiAddress(fmt.Sprintf("0x%x", addrBytes)))
	}
	return out, nil
}

func readVecU64String(res map[string]any, cmd, ret int) ([]string, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return nil, err
	}
	reader := bcs.NewReader(bytes)
	n, err := reader.ReadULEB()
	if err != nil {
		return nil, err
	}
	out := make([]string, 0, n)
	for i := uint64(0); i < n; i++ {
		v, err := reader.Read64()
		if err != nil {
			return nil, err
		}
		out = append(out, fmt.Sprintf("%d", v))
	}
	return out, nil
}

func readVecU128String(res map[string]any, cmd, ret int) ([]string, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return nil, err
	}
	reader := bcs.NewReader(bytes)
	n, err := reader.ReadULEB()
	if err != nil {
		return nil, err
	}
	out := make([]string, 0, n)
	for i := uint64(0); i < n; i++ {
		le, err := reader.ReadBytes(16)
		if err != nil {
			return nil, err
		}
		be := make([]byte, 16)
		for j := 0; j < 16; j++ {
			be[15-j] = le[j]
		}
		out = append(out, new(big.Int).SetBytes(be).String())
	}
	return out, nil
}

func readVecU64(res map[string]any, cmd, ret int) ([]uint64, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return nil, err
	}
	reader := bcs.NewReader(bytes)
	n, err := reader.ReadULEB()
	if err != nil {
		return nil, err
	}
	out := make([]uint64, 0, n)
	for i := uint64(0); i < n; i++ {
		v, err := reader.Read64()
		if err != nil {
			return nil, err
		}
		out = append(out, v)
	}
	return out, nil
}

func formatTokenAmount(raw uint64, scalar float64, decimals int) string {
	if scalar <= 0 {
		return fmt.Sprintf("%d", raw)
	}
	scalarInt := uint64(scalar)
	if scalarInt == 0 {
		return fmt.Sprintf("%d", raw)
	}
	intPart := raw / scalarInt
	fracPart := raw % scalarInt
	if fracPart == 0 {
		return fmt.Sprintf("%d", intPart)
	}
	scaleDigits := len(fmt.Sprintf("%d", scalarInt)) - 1
	frac := fmt.Sprintf("%0*d", scaleDigits, fracPart)
	if decimals < len(frac) {
		frac = frac[:decimals]
	}
	for len(frac) > 0 && frac[len(frac)-1] == '0' {
		frac = frac[:len(frac)-1]
	}
	if frac == "" {
		return fmt.Sprintf("%d", intPart)
	}
	return fmt.Sprintf("%d.%s", intPart, frac)
}

func parseOrder(bytes []byte) (map[string]any, error) {
	return parseOrderFromReader(bcs.NewReader(bytes))
}

func parseOrderFromReader(r *bcs.Reader) (map[string]any, error) {
	bmBytes, err := r.ReadBytes(32)
	if err != nil {
		return nil, err
	}
	orderID128, err := r.Read128()
	if err != nil {
		return nil, err
	}
	clientOrderID, err := r.Read64()
	if err != nil {
		return nil, err
	}
	quantity, err := r.Read64()
	if err != nil {
		return nil, err
	}
	filledQuantity, err := r.Read64()
	if err != nil {
		return nil, err
	}
	feeIsDeep, err := r.Read8()
	if err != nil {
		return nil, err
	}
	assetIsBase, err := r.Read8()
	if err != nil {
		return nil, err
	}
	deepPerAsset, err := r.Read64()
	if err != nil {
		return nil, err
	}
	epoch, err := r.Read64()
	if err != nil {
		return nil, err
	}
	status, err := r.Read8()
	if err != nil {
		return nil, err
	}
	expireTimestamp, err := r.Read64()
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"balance_manager_id":  suiutils.NormalizeSuiAddress(fmt.Sprintf("0x%x", bmBytes)),
		"order_id":            u128ToDecimalString(orderID128),
		"client_order_id":     fmt.Sprintf("%d", clientOrderID),
		"quantity_raw":        quantity,
		"filled_quantity_raw": filledQuantity,
		"fee_is_deep":         feeIsDeep == 1,
		"order_deep_price": map[string]any{
			"asset_is_base":      assetIsBase == 1,
			"deep_per_asset_raw": deepPerAsset,
		},
		"epoch":            fmt.Sprintf("%d", epoch),
		"status":           status,
		"expire_timestamp": fmt.Sprintf("%d", expireTimestamp),
	}, nil
}

func parseVecOrders(bytes []byte) ([]map[string]any, error) {
	r := bcs.NewReader(bytes)
	n, err := r.ReadULEB()
	if err != nil {
		return nil, err
	}
	out := make([]map[string]any, 0, n)
	for i := uint64(0); i < n; i++ {
		order, err := parseOrderFromReader(r)
		if err != nil {
			return nil, err
		}
		out = append(out, order)
	}
	return out, nil
}

func parseAccount(bytes []byte, baseScalar, quoteScalar float64) (map[string]any, error) {
	r := bcs.NewReader(bytes)
	epoch, err := r.Read64()
	if err != nil {
		return nil, err
	}
	openOrders, err := readVecU128StringFromReader(r)
	if err != nil {
		return nil, err
	}
	takerVol128, err := r.Read128()
	if err != nil {
		return nil, err
	}
	makerVol128, err := r.Read128()
	if err != nil {
		return nil, err
	}
	activeStake, err := r.Read64()
	if err != nil {
		return nil, err
	}
	inactiveStake, err := r.Read64()
	if err != nil {
		return nil, err
	}
	createdProposalByte, err := r.Read8()
	if err != nil {
		return nil, err
	}
	votedProposalTag, err := r.Read8()
	if err != nil {
		return nil, err
	}
	votedProposal := any(nil)
	if votedProposalTag == 1 {
		addrBytes, err := r.ReadBytes(32)
		if err != nil {
			return nil, err
		}
		votedProposal = suiutils.NormalizeSuiAddress(fmt.Sprintf("0x%x", addrBytes))
	}
	unclaimedRebates, err := parseBalancesFromReader(r, baseScalar, quoteScalar)
	if err != nil {
		return nil, err
	}
	settledBalances, err := parseBalancesFromReader(r, baseScalar, quoteScalar)
	if err != nil {
		return nil, err
	}
	owedBalances, err := parseBalancesFromReader(r, baseScalar, quoteScalar)
	if err != nil {
		return nil, err
	}

	return map[string]any{
		"epoch":             fmt.Sprintf("%d", epoch),
		"open_orders":       openOrders,
		"taker_volume":      u128ToFloat64(takerVol128) / baseScalar,
		"maker_volume":      u128ToFloat64(makerVol128) / baseScalar,
		"active_stake":      float64(activeStake) / utils.DeepScalar,
		"inactive_stake":    float64(inactiveStake) / utils.DeepScalar,
		"created_proposal":  createdProposalByte == 1,
		"voted_proposal":    votedProposal,
		"unclaimed_rebates": unclaimedRebates,
		"settled_balances":  settledBalances,
		"owed_balances":     owedBalances,
	}, nil
}

func parseBalancesFromReader(r *bcs.Reader, baseScalar, quoteScalar float64) (map[string]any, error) {
	base, err := r.Read64()
	if err != nil {
		return nil, err
	}
	quote, err := r.Read64()
	if err != nil {
		return nil, err
	}
	deep, err := r.Read64()
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"base":  float64(base) / baseScalar,
		"quote": float64(quote) / quoteScalar,
		"deep":  float64(deep) / utils.DeepScalar,
	}, nil
}

func readVecU128StringFromReader(r *bcs.Reader) ([]string, error) {
	n, err := r.ReadULEB()
	if err != nil {
		return nil, err
	}
	out := make([]string, 0, n)
	for i := uint64(0); i < n; i++ {
		le, err := r.ReadBytes(16)
		if err != nil {
			return nil, err
		}
		be := make([]byte, 16)
		for j := 0; j < 16; j++ {
			be[15-j] = le[j]
		}
		out = append(out, new(big.Int).SetBytes(be).String())
	}
	return out, nil
}

func u128ToDecimalString(v [16]byte) string {
	b := make([]byte, 16)
	for i := 0; i < 16; i++ {
		b[15-i] = v[i]
	}
	return new(big.Int).SetBytes(b).String()
}

func u128ToFloat64(v [16]byte) float64 {
	b := make([]byte, 16)
	for i := 0; i < 16; i++ {
		b[15-i] = v[i]
	}
	f, _ := new(big.Float).SetInt(new(big.Int).SetBytes(b)).Float64()
	return f
}

func decodeOrderID(orderID string) (bool, uint64, uint64) {
	n := new(big.Int)
	if _, ok := n.SetString(orderID, 10); !ok {
		return false, 0, 0
	}
	isBid := n.Bit(127) == 0
	priceMask := new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), 63), big.NewInt(1))
	priceBig := new(big.Int).Rsh(new(big.Int).Set(n), 64)
	priceBig.And(priceBig, priceMask)
	orderIDMask := new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), 64), big.NewInt(1))
	orderLow := new(big.Int).And(new(big.Int).Set(n), orderIDMask)
	return isBid, priceBig.Uint64(), orderLow.Uint64()
}

func parseMarginManagerStateFromResult(res map[string]any, cmd int, pool types.Pool, cfg *utils.DeepBookConfig) (map[string]any, error) {
	baseCoin := cfg.GetCoin(pool.BaseCoin)
	quoteCoin := cfg.GetCoin(pool.QuoteCoin)

	managerID, err := readAddress(res, cmd, 0)
	if err != nil {
		return nil, err
	}
	deepbookPoolID, err := readAddress(res, cmd, 1)
	if err != nil {
		return nil, err
	}
	riskRatioRaw, err := readU64(res, cmd, 2)
	if err != nil {
		return nil, err
	}
	baseAssetRaw, err := readU64(res, cmd, 3)
	if err != nil {
		return nil, err
	}
	quoteAssetRaw, err := readU64(res, cmd, 4)
	if err != nil {
		return nil, err
	}
	baseDebtRaw, err := readU64(res, cmd, 5)
	if err != nil {
		return nil, err
	}
	quoteDebtRaw, err := readU64(res, cmd, 6)
	if err != nil {
		return nil, err
	}
	basePythPrice, err := readU64(res, cmd, 7)
	if err != nil {
		return nil, err
	}
	basePythDecimalsBytes, err := extractReturnBCS(res, cmd, 8)
	if err != nil {
		return nil, err
	}
	quotePythPrice, err := readU64(res, cmd, 9)
	if err != nil {
		return nil, err
	}
	quotePythDecimalsBytes, err := extractReturnBCS(res, cmd, 10)
	if err != nil {
		return nil, err
	}
	currentPrice, err := readU64(res, cmd, 11)
	if err != nil {
		return nil, err
	}
	lowestTriggerAbovePrice, err := readU64(res, cmd, 12)
	if err != nil {
		return nil, err
	}
	highestTriggerBelowPrice, err := readU64(res, cmd, 13)
	if err != nil {
		return nil, err
	}

	basePythDecimals := uint8(0)
	if len(basePythDecimalsBytes) > 0 {
		basePythDecimals = basePythDecimalsBytes[0]
	}
	quotePythDecimals := uint8(0)
	if len(quotePythDecimalsBytes) > 0 {
		quotePythDecimals = quotePythDecimalsBytes[0]
	}

	return map[string]any{
		"managerId":                managerID,
		"deepbookPoolId":           deepbookPoolID,
		"riskRatio":                float64(riskRatioRaw) / utils.FloatScalar,
		"baseAsset":                formatTokenAmount(baseAssetRaw, baseCoin.Scalar, 6),
		"quoteAsset":               formatTokenAmount(quoteAssetRaw, quoteCoin.Scalar, 6),
		"baseDebt":                 formatTokenAmount(baseDebtRaw, baseCoin.Scalar, 6),
		"quoteDebt":                formatTokenAmount(quoteDebtRaw, quoteCoin.Scalar, 6),
		"basePythPrice":            fmt.Sprintf("%d", basePythPrice),
		"basePythDecimals":         int(basePythDecimals),
		"quotePythPrice":           fmt.Sprintf("%d", quotePythPrice),
		"quotePythDecimals":        int(quotePythDecimals),
		"currentPrice":             fmt.Sprintf("%d", currentPrice),
		"lowestTriggerAbovePrice":  fmt.Sprintf("%d", lowestTriggerAbovePrice),
		"highestTriggerBelowPrice": fmt.Sprintf("%d", highestTriggerBelowPrice),
	}, nil
}

func readReturnBCSBase64(res map[string]any, cmd, ret int) (string, error) {
	bytes, err := extractReturnBCS(res, cmd, ret)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(bytes), nil
}

func extractReturnBCS(res map[string]any, cmd, ret int) ([]byte, error) {
	commandResults, ok := res["commandResults"].([]any)
	if !ok || len(commandResults) <= cmd {
		return nil, fmt.Errorf("missing commandResults[%d]", cmd)
	}
	cr, ok := commandResults[cmd].(map[string]any)
	if !ok {
		return nil, fmt.Errorf("invalid command result")
	}
	returnValues, ok := cr["returnValues"].([]any)
	if !ok || len(returnValues) <= ret {
		return nil, fmt.Errorf("missing returnValues[%d]", ret)
	}
	rv, ok := returnValues[ret].(map[string]any)
	if !ok {
		return nil, fmt.Errorf("invalid return value")
	}
	b64, ok := rv["bcs"].(string)
	if !ok {
		return nil, fmt.Errorf("missing bcs")
	}
	return base64.StdEncoding.DecodeString(b64)
}
