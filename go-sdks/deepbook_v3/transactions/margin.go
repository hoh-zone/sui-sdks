package transactions

import (
	"math"

	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	stx "github.com/sui-sdks/go-sdks/sui/transactions"
)

type MarginAdminContract struct{ config *utils.DeepBookConfig }
type MarginMaintainerContract struct{ config *utils.DeepBookConfig }
type MarginManagerContract struct{ config *utils.DeepBookConfig }
type MarginPoolContract struct{ config *utils.DeepBookConfig }
type MarginRegistryContract struct{ config *utils.DeepBookConfig }
type MarginLiquidationsContract struct{ config *utils.DeepBookConfig }
type PoolProxyContract struct{ config *utils.DeepBookConfig }
type MarginTPSLContract struct{ config *utils.DeepBookConfig }

func NewMarginAdminContract(config *utils.DeepBookConfig) *MarginAdminContract {
	return &MarginAdminContract{config: config}
}
func NewMarginMaintainerContract(config *utils.DeepBookConfig) *MarginMaintainerContract {
	return &MarginMaintainerContract{config: config}
}
func NewMarginManagerContract(config *utils.DeepBookConfig) *MarginManagerContract {
	return &MarginManagerContract{config: config}
}
func NewMarginPoolContract(config *utils.DeepBookConfig) *MarginPoolContract {
	return &MarginPoolContract{config: config}
}
func NewMarginRegistryContract(config *utils.DeepBookConfig) *MarginRegistryContract {
	return &MarginRegistryContract{config: config}
}
func NewMarginLiquidationsContract(config *utils.DeepBookConfig) *MarginLiquidationsContract {
	return &MarginLiquidationsContract{config: config}
}
func NewPoolProxyContract(config *utils.DeepBookConfig) *PoolProxyContract {
	return &PoolProxyContract{config: config}
}
func NewMarginTPSLContract(config *utils.DeepBookConfig) *MarginTPSLContract {
	return &MarginTPSLContract{config: config}
}

func (c *MarginManagerContract) managerTypes(managerKey string) (types.Coin, types.Coin, types.MarginManager, types.Pool) {
	manager := c.config.GetMarginManager(managerKey)
	pool := c.config.GetPool(manager.PoolKey)
	return c.config.GetCoin(pool.BaseCoin), c.config.GetCoin(pool.QuoteCoin), manager, pool
}

func (c *MarginManagerContract) NewMarginManager(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::new", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.config.RegistryID), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) NewMarginManagerWithInitializer(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::new_with_initializer", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.config.RegistryID), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) DepositBase(tx *stx.Transaction, params types.DepositParams) stx.Argument {
	base, quote, manager, _ := c.managerTypes(params.ManagerKey)
	amount := uint64(math.Round(params.Amount * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::deposit", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, amount), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type, base.Type})
}

func (c *MarginManagerContract) DepositQuote(tx *stx.Transaction, params types.DepositParams) stx.Argument {
	base, quote, manager, _ := c.managerTypes(params.ManagerKey)
	amount := uint64(math.Round(params.Amount * quote.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::deposit", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, amount), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type, quote.Type})
}

func (c *MarginManagerContract) WithdrawBase(tx *stx.Transaction, managerKey string, amount float64) stx.Argument {
	base, quote, manager, _ := c.managerTypes(managerKey)
	input := uint64(math.Round(amount * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::withdraw", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, input), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type, base.Type})
}

func (c *MarginManagerContract) WithdrawQuote(tx *stx.Transaction, managerKey string, amount float64) stx.Argument {
	base, quote, manager, _ := c.managerTypes(managerKey)
	input := uint64(math.Round(amount * quote.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::withdraw", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, input), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type, quote.Type})
}

func (c *MarginManagerContract) BorrowBase(tx *stx.Transaction, managerKey string, amount float64) stx.Argument {
	base, quote, manager, _ := c.managerTypes(managerKey)
	input := uint64(math.Round(amount * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::borrow_base", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, input), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) BorrowQuote(tx *stx.Transaction, managerKey string, amount float64) stx.Argument {
	base, quote, manager, _ := c.managerTypes(managerKey)
	input := uint64(math.Round(amount * quote.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::borrow_quote", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, input), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) RepayBase(tx *stx.Transaction, managerKey string, amount float64) stx.Argument {
	base, quote, manager, _ := c.managerTypes(managerKey)
	input := uint64(math.Round(amount * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::repay_base", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, input), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) RepayQuote(tx *stx.Transaction, managerKey string, amount float64) stx.Argument {
	base, quote, manager, _ := c.managerTypes(managerKey)
	input := uint64(math.Round(amount * quote.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::repay_quote", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, input), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) SetMarginManagerReferral(tx *stx.Transaction, managerKey, referral string) stx.Argument {
	_, _, manager, _ := c.managerTypes(managerKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::set_margin_manager_referral", []stx.Argument{
		tx.Object(manager.Address), tx.Object(referral), tx.Object(c.config.MarginRegistryID),
	}, nil)
}

func (c *MarginManagerContract) UnsetMarginManagerReferral(tx *stx.Transaction, managerKey, poolKey string) stx.Argument {
	_, _, manager, pool := c.managerTypes(managerKey)
	_ = poolKey
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::unset_margin_manager_referral", []stx.Argument{
		tx.Object(manager.Address), tx.Object(pool.Address), tx.Object(c.config.MarginRegistryID),
	}, nil)
}

func (c *MarginManagerContract) GetMarginAccountOrderDetails(tx *stx.Transaction, managerKey string) stx.Argument {
	base, quote, manager, pool := c.managerTypes(managerKey)
	bm := tx.MoveCall(c.config.MarginPackageID+"::margin_manager::balance_manager", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
	return tx.MoveCall(c.config.DeepbookPackageID+"::pool::get_account_order_details", []stx.Argument{
		tx.Object(pool.Address), bm,
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) OwnerByPoolKey(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::owner", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) DeepbookPool(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::deepbook_pool", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) MarginPoolID(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::margin_pool_id", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) BorrowedShares(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::borrowed_shares", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) BorrowedBaseShares(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::borrowed_base_shares", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) BorrowedQuoteShares(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::borrowed_quote_shares", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) HasBaseDebt(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::has_base_debt", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) BalanceManagerByPoolKey(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::balance_manager", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) CalculateAssets(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::calculate_assets", []stx.Argument{
		tx.Object(marginManagerID), tx.Object(pool.Address),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) CalculateDebts(tx *stx.Transaction, poolKey, coinKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	debtCoin := c.config.GetCoin(coinKey)
	marginPool, ok := c.config.MarginPools[coinKey]
	marginPoolObject := tx.Object(c.config.MarginRegistryID)
	if ok {
		marginPoolObject = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::calculate_debts", []stx.Argument{
		tx.Object(marginManagerID), marginPoolObject, tx.Object("0x6"),
	}, []string{base.Type, quote.Type, debtCoin.Type})
}

func (c *MarginManagerContract) ManagerState(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	baseMarginPool, hasBase := c.config.MarginPools[pool.BaseCoin]
	quoteMarginPool, hasQuote := c.config.MarginPools[pool.QuoteCoin]
	baseMarginPoolObject := tx.Object(c.config.MarginRegistryID)
	quoteMarginPoolObject := tx.Object(c.config.MarginRegistryID)
	if hasBase {
		baseMarginPoolObject = tx.Object(baseMarginPool.Address)
	}
	if hasQuote {
		quoteMarginPoolObject = tx.Object(quoteMarginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::manager_state", []stx.Argument{
		tx.Object(marginManagerID),
		tx.Object(c.config.MarginRegistryID),
		tx.Object(base.PriceInfoObjectID),
		tx.Object(quote.PriceInfoObjectID),
		tx.Object(pool.Address),
		baseMarginPoolObject,
		quoteMarginPoolObject,
		tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) BaseBalance(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::base_balance", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) QuoteBalance(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::quote_balance", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) DeepBalance(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::deep_balance", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) ShareMarginManager(tx *stx.Transaction, poolKey, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::share_margin_manager", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) DepositDuringInitialization(tx *stx.Transaction, params types.DepositDuringInitParams) stx.Argument {
	pool := c.config.GetPool(params.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	amount := uint64(math.Round(params.Amount * base.Scalar))
	if params.CoinType == pool.QuoteCoin {
		amount = uint64(math.Round(params.Amount * quote.Scalar))
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::deposit_during_initialization", []stx.Argument{
		params.Manager, params.Coin, pureU64(tx, amount), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) WithdrawDeep(tx *stx.Transaction, marginManagerKey, poolKey string, amount float64) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	q := uint64(math.Round(amount * utils.DeepScalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::withdraw_deep", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, q), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) Liquidate(tx *stx.Transaction, liquidatorManagerKey, targetManagerKey, poolKey string, isBase bool) stx.Argument {
	liquidator := c.config.GetMarginManager(liquidatorManagerKey)
	target := c.config.GetMarginManager(targetManagerKey)
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::liquidate", []stx.Argument{
		tx.Object(liquidator.Address), tx.Object(target.Address), tx.Object(pool.Address), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginManagerContract) HasQuoteDebt(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::has_quote_debt", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginPoolContract) MintSupplierCap(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::mint_supplier_cap", []stx.Argument{tx.Object(c.config.MarginRegistryID), tx.Object("0x6")}, nil)
}

func (c *MarginPoolContract) GetID(tx *stx.Transaction, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::id", []stx.Argument{tx.Object(c.config.MarginRegistryID)}, []string{coin.Type})
}

func (c *MarginPoolContract) TotalSupply(tx *stx.Transaction, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::total_supply", []stx.Argument{tx.Object(c.config.MarginRegistryID)}, []string{coin.Type})
}

func (c *MarginPoolContract) SupplyShares(tx *stx.Transaction, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::supply_shares", []stx.Argument{tx.Object(c.config.MarginRegistryID)}, []string{coin.Type})
}

func (c *MarginPoolContract) TotalBorrow(tx *stx.Transaction, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::total_borrow", []stx.Argument{tx.Object(c.config.MarginRegistryID)}, []string{coin.Type})
}

func (c *MarginPoolContract) BorrowShares(tx *stx.Transaction, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::borrow_shares", []stx.Argument{tx.Object(c.config.MarginRegistryID)}, []string{coin.Type})
}

func (c *MarginPoolContract) InterestRate(tx *stx.Transaction, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::interest_rate", []stx.Argument{tx.Object(c.config.MarginRegistryID)}, []string{coin.Type})
}

func (c *MarginPoolContract) DeepbookPoolAllowed(tx *stx.Transaction, coinKey, deepbookPoolID string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::deepbook_pool_allowed", []stx.Argument{
			tx.Object(c.config.MarginRegistryID), pureAddress(tx, deepbookPoolID),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::deepbook_pool_allowed", []stx.Argument{
		tx.Object(marginPool.Address), pureAddress(tx, deepbookPoolID),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) LastUpdateTimestamp(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::last_update_timestamp", []stx.Argument{
			tx.Object(c.config.MarginRegistryID),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::last_update_timestamp", []stx.Argument{
		tx.Object(marginPool.Address),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) SupplyCap(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::supply_cap", []stx.Argument{
			tx.Object(c.config.MarginRegistryID),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::supply_cap", []stx.Argument{
		tx.Object(marginPool.Address),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) MaxUtilizationRate(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::max_utilization_rate", []stx.Argument{
			tx.Object(c.config.MarginRegistryID),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::max_utilization_rate", []stx.Argument{
		tx.Object(marginPool.Address),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) ProtocolSpread(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::protocol_spread", []stx.Argument{
			tx.Object(c.config.MarginRegistryID),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::protocol_spread", []stx.Argument{
		tx.Object(marginPool.Address),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) MinBorrow(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::min_borrow", []stx.Argument{
			tx.Object(c.config.MarginRegistryID),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::min_borrow", []stx.Argument{
		tx.Object(marginPool.Address),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) UserSupplyShares(tx *stx.Transaction, coinKey, supplierCapID string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::user_supply_shares", []stx.Argument{
			tx.Object(c.config.MarginRegistryID), pureAddress(tx, supplierCapID),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::user_supply_shares", []stx.Argument{
		tx.Object(marginPool.Address), pureAddress(tx, supplierCapID),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) UserSupplyAmount(tx *stx.Transaction, coinKey, supplierCapID string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	if !ok {
		coin := c.config.GetCoin(coinKey)
		return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::user_supply_amount", []stx.Argument{
			tx.Object(c.config.MarginRegistryID), pureAddress(tx, supplierCapID), tx.Object("0x6"),
		}, []string{coin.Type})
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::user_supply_amount", []stx.Argument{
		tx.Object(marginPool.Address), pureAddress(tx, supplierCapID), tx.Object("0x6"),
	}, []string{marginPool.Type})
}

func (c *MarginPoolContract) SupplyToMarginPool(tx *stx.Transaction, coinKey string, amount float64) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::supply", []stx.Argument{
		object, tx.PureBytes([]byte{}), tx.Object("0x6"),
	}, []string{coin.Type})
}

func (c *MarginPoolContract) WithdrawFromMarginPool(tx *stx.Transaction, coinKey string, amount float64) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	q := uint64(math.Round(amount * coin.Scalar))
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::withdraw", []stx.Argument{
		object, tx.PureBytes([]byte{}), pureU64(tx, q), tx.Object(c.config.MarginRegistryID),
	}, []string{coin.Type})
}

func (c *MarginPoolContract) MintSupplyReferral(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::mint_supply_referral", []stx.Argument{
		object, tx.Object("0x6"),
	}, []string{coin.Type})
}

func (c *MarginPoolContract) WithdrawReferralFees(tx *stx.Transaction, coinKey, referralID string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::withdraw_referral_fees", []stx.Argument{
		object, tx.PureBytes([]byte{}), pureAddress(tx, referralID), tx.Object(c.config.MarginRegistryID),
	}, []string{coin.Type})
}

func (c *MarginPoolContract) UpdateInterestWeight(tx *stx.Transaction, coinKey string, rate float64) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	r := uint64(math.Round(rate * utils.FloatScalar))
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::update_interest_weight", []stx.Argument{
		object, pureU64(tx, r), tx.Object("0x6"),
	}, []string{coin.Type})
}

func (c *MarginPoolContract) SetMaxUtilizationRate(tx *stx.Transaction, coinKey string, rate float64) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	r := uint64(math.Round(rate * utils.FloatScalar))
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::set_max_utilization_rate", []stx.Argument{
		object, pureU64(tx, r), tx.Object("0x6"),
	}, []string{coin.Type})
}

func (c *MarginPoolContract) GetUserSupplyShares(tx *stx.Transaction, coinKey, supplierCapID string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::user_supply_shares", []stx.Argument{
		object, pureAddress(tx, supplierCapID),
	}, []string{coin.Type})
}

func (c *MarginPoolContract) GetUserBorrowShares(tx *stx.Transaction, coinKey, collateralID string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::user_borrow_shares", []stx.Argument{
		object, pureAddress(tx, collateralID),
	}, []string{coin.Type})
}

func (c *MarginRegistryContract) PoolEnabled(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::pool_enabled", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginRegistryContract) GetMarginPoolID(tx *stx.Transaction, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::get_margin_pool_id", []stx.Argument{
		tx.Object(c.config.MarginRegistryID),
	}, []string{coin.Type})
}

func (c *MarginRegistryContract) GetMarginManagerIDs(tx *stx.Transaction, owner string) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::get_margin_manager_ids", []stx.Argument{
		tx.PureBytes([]byte(owner)), tx.Object(c.config.MarginRegistryID),
	}, nil)
}

func (c *MarginRegistryContract) BaseMarginPoolID(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::base_margin_pool_id", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) QuoteMarginPoolID(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::quote_margin_pool_id", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) MinWithdrawRiskRatio(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::min_withdraw_risk_ratio", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) MinBorrowRiskRatio(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::min_borrow_risk_ratio", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) LiquidationRiskRatio(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::liquidation_risk_ratio", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) TargetLiquidationRiskRatio(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::target_liquidation_risk_ratio", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) UserLiquidationReward(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::user_liquidation_reward", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) PoolLiquidationReward(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::pool_liquidation_reward", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureAddress(tx, pool.Address),
	}, nil)
}

func (c *MarginRegistryContract) AllowedMaintainers(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::allowed_maintainers", []stx.Argument{
		tx.Object(c.config.MarginRegistryID),
	}, nil)
}

func (c *MarginRegistryContract) AllowedPauseCaps(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::allowed_pause_caps", []stx.Argument{
		tx.Object(c.config.MarginRegistryID),
	}, nil)
}

func (c *MarginLiquidationsContract) CreateLiquidationVault(tx *stx.Transaction, liquidationAdminCap string) stx.Argument {
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::create_liquidation_vault", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(liquidationAdminCap),
	}, nil)
}

func (c *MarginLiquidationsContract) Balance(tx *stx.Transaction, vaultID, coinKey string) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::balance", []stx.Argument{
		tx.Object(vaultID),
	}, []string{coin.Type})
}

func (c *MarginLiquidationsContract) Deposit(tx *stx.Transaction, vaultID, coinKey string, amount float64) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	q := uint64(math.Round(amount * coin.Scalar))
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::deposit", []stx.Argument{
		tx.Object(vaultID), pureU64(tx, q),
	}, []string{coin.Type})
}

func (c *MarginLiquidationsContract) Withdraw(tx *stx.Transaction, vaultID, coinKey string, amount float64) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	q := uint64(math.Round(amount * coin.Scalar))
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::withdraw", []stx.Argument{
		tx.Object(vaultID), pureU64(tx, q),
	}, []string{coin.Type})
}

func (c *MarginLiquidationsContract) LiquidateBase(tx *stx.Transaction, vaultID, marginManagerID, poolKey string, amount float64) stx.Argument {
	marginManager := c.config.GetMarginManager(marginManagerID)
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	q := uint64(math.Round(amount * base.Scalar))
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::liquidate_base", []stx.Argument{
		tx.Object(vaultID), pureU64(tx, q), tx.Object(pool.Address), tx.Object(marginManager.Address),
	}, []string{base.Type, quote.Type})
}

func (c *MarginLiquidationsContract) LiquidateQuote(tx *stx.Transaction, vaultID, marginManagerID, poolKey string, amount float64) stx.Argument {
	marginManager := c.config.GetMarginManager(marginManagerID)
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	q := uint64(math.Round(amount * quote.Scalar))
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::liquidate_quote", []stx.Argument{
		tx.Object(vaultID), pureU64(tx, q), tx.Object(pool.Address), tx.Object(marginManager.Address),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) PlaceLimitOrder(tx *stx.Transaction, params types.PlaceMarginLimitOrderParams) stx.Argument {
	manager := c.config.GetMarginManager(params.MarginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	price := uint64(math.Round((params.Price * utils.FloatScalar * quote.Scalar) / base.Scalar))
	quantity := uint64(math.Round(params.Quantity * base.Scalar))
	exp := params.Expiration
	if exp == 0 {
		exp = utils.MaxTimestamp
	}
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::place_limit_order", []stx.Argument{
		tx.Object(manager.Address),
		pureU64(tx, parseU64(params.ClientOrderID)),
		pureU8(tx, uint8(params.OrderType)),
		pureU8(tx, uint8(params.SelfMatchingOption)),
		pureU64(tx, price),
		pureU64(tx, quantity),
		pureBool(tx, params.IsBid),
		pureBool(tx, params.PayWithDeep),
		pureU64(tx, exp),
		tx.Object(c.config.MarginRegistryID),
		tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) PlaceMarketOrder(tx *stx.Transaction, params types.PlaceMarginMarketOrderParams) stx.Argument {
	manager := c.config.GetMarginManager(params.MarginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	quantity := uint64(math.Round(params.Quantity * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::place_market_order", []stx.Argument{
		tx.Object(manager.Address),
		pureU64(tx, parseU64(params.ClientOrderID)),
		pureU8(tx, uint8(params.SelfMatchingOption)),
		pureU64(tx, quantity),
		pureBool(tx, params.IsBid),
		pureBool(tx, params.PayWithDeep),
		tx.Object(c.config.MarginRegistryID),
		tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) CancelOrder(tx *stx.Transaction, marginManagerKey, orderID string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::cancel_order", []stx.Argument{
		tx.Object(manager.Address), pureU128String(tx, orderID), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) CancelOrders(tx *stx.Transaction, marginManagerKey string, orderIDs []string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::cancel_orders", []stx.Argument{
		tx.Object(manager.Address), pureVecU128(tx, orderIDs), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) CancelAllOrders(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::cancel_all_orders", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) WithdrawSettledAmounts(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::withdraw_settled_amounts", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) Stake(tx *stx.Transaction, marginManagerKey string, stakeAmount float64) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	stake := uint64(math.Round(stakeAmount * utils.DeepScalar))
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::stake", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, stake), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) Unstake(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::unstake", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) Vote(tx *stx.Transaction, marginManagerKey, proposalID string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::vote", []stx.Argument{
		tx.Object(manager.Address), pureU128String(tx, proposalID), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) ClaimRebate(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::claim_rebate", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) AddConditionalOrder(tx *stx.Transaction, params types.AddConditionalOrderParams) stx.Argument {
	manager := c.config.GetMarginManager(params.MarginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	condition := c.NewCondition(tx, manager.PoolKey, params.TriggerBelowPrice, params.TriggerPrice)

	var pending stx.Argument
	switch {
	case params.PendingLimitOrder != nil:
		pending = c.NewPendingLimitOrder(tx, manager.PoolKey, *params.PendingLimitOrder)
	case params.PendingMarketOrder != nil:
		pending = c.NewPendingMarketOrder(tx, manager.PoolKey, *params.PendingMarketOrder)
	default:
		panic("either PendingLimitOrder or PendingMarketOrder must be provided")
	}

	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::add_conditional_order", []stx.Argument{
		tx.Object(manager.Address),
		tx.Object(pool.Address),
		tx.Object(base.PriceInfoObjectID),
		tx.Object(quote.PriceInfoObjectID),
		tx.Object(c.config.MarginRegistryID),
		pureU64(tx, parseU64(params.ConditionalOrderID)),
		condition,
		pending,
		tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) NewCondition(tx *stx.Transaction, poolKey string, triggerBelowPrice bool, triggerPrice float64) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	inputPrice := uint64(math.Round((triggerPrice * utils.FloatScalar * quote.Scalar) / base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::tpsl::new_condition", []stx.Argument{
		pureBool(tx, triggerBelowPrice), pureU64(tx, inputPrice),
	}, nil)
}

func (c *MarginTPSLContract) NewPendingLimitOrder(tx *stx.Transaction, poolKey string, params types.PendingLimitOrderParams) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	inputPrice := uint64(math.Round((params.Price * utils.FloatScalar * quote.Scalar) / base.Scalar))
	inputQuantity := uint64(math.Round(params.Quantity * base.Scalar))
	expireTimestamp := params.ExpireTimestamp
	if expireTimestamp == 0 {
		expireTimestamp = utils.MaxTimestamp
	}
	return tx.MoveCall(c.config.MarginPackageID+"::tpsl::new_pending_limit_order", []stx.Argument{
		pureU64(tx, parseU64(params.ClientOrderID)),
		pureU8(tx, uint8(params.OrderType)),
		pureU8(tx, uint8(params.SelfMatchingOption)),
		pureU64(tx, inputPrice),
		pureU64(tx, inputQuantity),
		pureBool(tx, params.IsBid),
		pureBool(tx, params.PayWithDeep),
		pureU64(tx, expireTimestamp),
	}, nil)
}

func (c *MarginTPSLContract) NewPendingMarketOrder(tx *stx.Transaction, poolKey string, params types.PendingMarketOrderParams) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	inputQuantity := uint64(math.Round(params.Quantity * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::tpsl::new_pending_market_order", []stx.Argument{
		pureU64(tx, parseU64(params.ClientOrderID)),
		pureU8(tx, uint8(params.SelfMatchingOption)),
		pureU64(tx, inputQuantity),
		pureBool(tx, params.IsBid),
		pureBool(tx, params.PayWithDeep),
	}, nil)
}

func (c *MarginTPSLContract) CancelAllConditionalOrders(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::cancel_all_conditional_orders", []stx.Argument{
		tx.Object(manager.Address), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) CancelConditionalOrder(tx *stx.Transaction, marginManagerKey, conditionalOrderID string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::cancel_conditional_order", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, parseU64(conditionalOrderID)), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) ExecuteConditionalOrders(tx *stx.Transaction, managerAddress, poolKey string, maxOrdersToExecute uint64) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::execute_conditional_orders", []stx.Argument{
		tx.Object(managerAddress),
		tx.Object(pool.Address),
		tx.Object(base.PriceInfoObjectID),
		tx.Object(quote.PriceInfoObjectID),
		tx.Object(c.config.MarginRegistryID),
		pureU64(tx, maxOrdersToExecute),
		tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) ConditionalOrderIDs(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::conditional_order_ids", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) ConditionalOrder(tx *stx.Transaction, poolKey, marginManagerID, conditionalOrderID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::conditional_order", []stx.Argument{
		tx.Object(marginManagerID), pureU64(tx, parseU64(conditionalOrderID)),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) LowestTriggerAbovePrice(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::lowest_trigger_above_price", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *MarginTPSLContract) HighestTriggerBelowPrice(tx *stx.Transaction, poolKey, marginManagerID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_manager::highest_trigger_below_price", []stx.Argument{
		tx.Object(marginManagerID),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) PlaceReduceOnlyLimitOrder(tx *stx.Transaction, params types.PlaceMarginLimitOrderParams) stx.Argument {
	manager := c.config.GetMarginManager(params.MarginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	price := uint64(math.Round((params.Price * utils.FloatScalar * quote.Scalar) / base.Scalar))
	quantity := uint64(math.Round(params.Quantity * base.Scalar))
	exp := params.Expiration
	if exp == 0 {
		exp = utils.MaxTimestamp
	}
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::place_reduce_only_limit_order", []stx.Argument{
		tx.Object(manager.Address),
		pureU64(tx, parseU64(params.ClientOrderID)),
		pureU8(tx, uint8(params.OrderType)),
		pureU8(tx, uint8(params.SelfMatchingOption)),
		pureU64(tx, price),
		pureU64(tx, quantity),
		pureBool(tx, params.IsBid),
		pureU64(tx, exp),
		tx.Object(c.config.MarginRegistryID),
		tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) PlaceReduceOnlyMarketOrder(tx *stx.Transaction, params types.PlaceMarginMarketOrderParams) stx.Argument {
	manager := c.config.GetMarginManager(params.MarginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	quantity := uint64(math.Round(params.Quantity * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::place_reduce_only_market_order", []stx.Argument{
		tx.Object(manager.Address),
		pureU64(tx, parseU64(params.ClientOrderID)),
		pureU8(tx, uint8(params.SelfMatchingOption)),
		pureU64(tx, quantity),
		pureBool(tx, params.IsBid),
		tx.Object(c.config.MarginRegistryID),
		tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) ModifyOrder(tx *stx.Transaction, marginManagerKey, orderID string, newQuantity float64) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	qty := uint64(math.Round(newQuantity * base.Scalar))
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::modify_order", []stx.Argument{
		tx.Object(manager.Address), pureU128String(tx, orderID), pureU64(tx, qty), tx.Object(c.config.MarginRegistryID), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *PoolProxyContract) SubmitProposal(tx *stx.Transaction, marginManagerKey string, params types.ProposalParams) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	takerFee := uint64(math.Round(params.TakerFee * utils.FloatScalar))
	makerFee := uint64(math.Round(params.MakerFee * utils.FloatScalar))
	stakeRequired := uint64(math.Round(params.StakeRequired * utils.DeepScalar))
	return tx.MoveCall(c.config.MarginPackageID+"::pool_proxy::submit_proposal", []stx.Argument{
		tx.Object(manager.Address), pureU64(tx, takerFee), pureU64(tx, makerFee), pureU64(tx, stakeRequired), tx.Object(c.config.MarginRegistryID),
	}, []string{base.Type, quote.Type})
}
