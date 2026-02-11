package deepbookv3

import (
	"context"
	"encoding/base64"
	"fmt"
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

func (c *Client) GetOrder(ctx context.Context, poolKey, orderID string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetOrder(tx, poolKey, orderID)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) GetOrders(ctx context.Context, poolKey string, orderIDs []string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetOrders(tx, poolKey, orderIDs)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) AccountOpenOrders(ctx context.Context, poolKey, managerKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.AccountOpenOrders(tx, poolKey, managerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) VaultBalances(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.VaultBalances(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) GetPoolIDByAssets(ctx context.Context, baseType, quoteType string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetPoolIDByAssets(tx, baseType, quoteType)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) GetPoolIdByAssets(ctx context.Context, baseType, quoteType string) (string, error) {
	return c.GetPoolIDByAssets(ctx, baseType, quoteType)
}

func (c *Client) PoolTradeParams(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.PoolTradeParams(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) PoolBookParams(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.PoolBookParams(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) Account(ctx context.Context, poolKey, managerKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.Account(tx, poolKey, managerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) LockedBalance(ctx context.Context, poolKey, balanceManagerKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.LockedBalance(tx, poolKey, balanceManagerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) GetPoolDeepPrice(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetPoolDeepPrice(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
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

func (c *Client) GetMarginAccountOrderDetails(ctx context.Context, marginManagerKey string) (string, error) {
	tx := stx.NewTransaction()
	c.MarginManager.GetMarginAccountOrderDetails(tx, marginManagerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 1, 0)
}

func (c *Client) GetQuoteQuantityOutInputFee(ctx context.Context, poolKey string, baseQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetQuoteQuantityOutInputFee(tx, poolKey, baseQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	v, _ := readU64(res, 0, 0)
	return map[string]any{"baseQuantity": baseQuantity, "result": v}, nil
}

func (c *Client) GetBaseQuantityOutInputFee(ctx context.Context, poolKey string, quoteQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetBaseQuantityOutInputFee(tx, poolKey, quoteQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	v, _ := readU64(res, 0, 0)
	return map[string]any{"quoteQuantity": quoteQuantity, "result": v}, nil
}

func (c *Client) GetQuantityOutInputFee(ctx context.Context, poolKey string, baseQuantity, quoteQuantity float64) (map[string]any, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetQuantityOutInputFee(tx, poolKey, baseQuantity, quoteQuantity)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return nil, err
	}
	v, _ := readU64(res, 0, 0)
	return map[string]any{"baseQuantity": baseQuantity, "quoteQuantity": quoteQuantity, "result": v}, nil
}

func (c *Client) GetBaseQuantityIn(ctx context.Context, poolKey string, targetQuoteQuantity float64, payWithDeep bool) (uint64, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetBaseQuantityIn(tx, poolKey, targetQuoteQuantity, payWithDeep)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	return readU64(res, 0, 0)
}

func (c *Client) GetQuoteQuantityIn(ctx context.Context, poolKey string, targetBaseQuantity float64, payWithDeep bool) (uint64, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetQuoteQuantityIn(tx, poolKey, targetBaseQuantity, payWithDeep)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	return readU64(res, 0, 0)
}

func (c *Client) GetAccountOrderDetails(ctx context.Context, poolKey, managerKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetAccountOrderDetails(tx, poolKey, managerKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) GetOrderDeepRequired(ctx context.Context, poolKey string, baseQuantity, price float64) (uint64, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetOrderDeepRequired(tx, poolKey, baseQuantity, price)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return 0, err
	}
	return readU64(res, 0, 0)
}

func (c *Client) PoolTradeParamsNext(ctx context.Context, poolKey string) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.PoolTradeParamsNext(tx, poolKey)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
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

func (c *Client) GetLevel2Range(ctx context.Context, poolKey string, priceLow, priceHigh float64, isBid bool) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetLevel2Range(tx, poolKey, priceLow, priceHigh, isBid)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
}

func (c *Client) GetLevel2TicksFromMid(ctx context.Context, poolKey string, ticks uint64) (string, error) {
	tx := stx.NewTransaction()
	c.DeepBook.GetLevel2TicksFromMid(tx, poolKey, ticks)
	res, err := c.simulate(ctx, tx)
	if err != nil {
		return "", err
	}
	return readReturnBCSBase64(res, 0, 0)
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
