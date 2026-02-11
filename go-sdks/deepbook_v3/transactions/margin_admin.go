package transactions

import (
	"math"

	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	stx "github.com/sui-sdks/go-sdks/sui/transactions"
)

func (c *MarginAdminContract) adminCap() string {
	if c.config.MarginAdminCap == "" {
		panic(&utils.ConfigurationError{DeepBookError: utils.DeepBookError{Msg: utils.ErrorMessages.MarginAdminCapNotSet}})
	}
	return c.config.MarginAdminCap
}

func (c *MarginAdminContract) MintMaintainerCap(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::mint_maintainer_cap", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) PauseMarginManager(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::pause_margin_manager", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.adminCap()),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) UnpauseMarginManager(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::unpause_margin_manager", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.adminCap()),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) RegisterDeepbookPool(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::register_deepbook_pool", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) UnregisterDeepbookPool(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::unregister_deepbook_pool", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) PausePool(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::pause_pool", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.adminCap()),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) UnpausePool(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::unpause_pool", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.adminCap()),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) SetPausedCap(tx *stx.Transaction, pausedCap string) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::set_paused_cap", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(pausedCap), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) UpdatePoolConfig(tx *stx.Transaction, poolKey string, params types.PoolConfigParams) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	minWithdrawRiskRatio := uint64(math.Round(params.MinWithdrawRiskRatio * utils.FloatScalar))
	minBorrowRiskRatio := uint64(math.Round(params.MinBorrowRiskRatio * utils.FloatScalar))
	liquidationRiskRatio := uint64(math.Round(params.LiquidationRiskRatio * utils.FloatScalar))
	targetLiquidationRiskRatio := uint64(math.Round(params.TargetLiquidationRiskRatio * utils.FloatScalar))
	userLiquidationReward := uint64(math.Round(params.UserLiquidationReward * utils.FloatScalar))
	poolLiquidationReward := uint64(math.Round(params.PoolLiquidationReward * utils.FloatScalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::update_pool_config", []stx.Argument{
		tx.Object(pool.Address),
		pureU64(tx, minWithdrawRiskRatio),
		pureU64(tx, minBorrowRiskRatio),
		pureU64(tx, liquidationRiskRatio),
		pureU64(tx, targetLiquidationRiskRatio),
		pureU64(tx, userLiquidationReward),
		pureU64(tx, poolLiquidationReward),
		tx.Object(c.adminCap()),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) ForceWithdrawMarginManager(tx *stx.Transaction, marginManagerKey string) stx.Argument {
	manager := c.config.GetMarginManager(marginManagerKey)
	pool := c.config.GetPool(manager.PoolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::force_withdraw_margin_manager", []stx.Argument{
		tx.Object(manager.Address), tx.Object(c.adminCap()),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) WithdrawWithdrawalFee(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::withdraw_withdrawal_fee", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.adminCap()),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) PauseMarginAsset(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::pause_margin_asset", []stx.Argument{
		object, tx.Object(c.adminCap()),
	}, []string{coin.Type})
}

func (c *MarginAdminContract) UnpauseMarginAsset(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::unpause_margin_asset", []stx.Argument{
		object, tx.Object(c.adminCap()),
	}, []string{coin.Type})
}

func (c *MarginAdminContract) UpdateInterestWeightConfig(tx *stx.Transaction, coinKey string, weight float64) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	w := uint64(math.Round(weight * utils.FloatScalar))
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::update_interest_weight", []stx.Argument{
		object, pureU64(tx, w), tx.Object(c.adminCap()),
	}, []string{coin.Type})
}

func (c *MarginAdminContract) SetMaxUtilizationRate(tx *stx.Transaction, coinKey string, rate float64) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	r := uint64(math.Round(rate * utils.FloatScalar))
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::set_max_utilization_rate", []stx.Argument{
		object, pureU64(tx, r), tx.Object(c.adminCap()),
	}, []string{coin.Type})
}

func (c *MarginAdminContract) EmergencyUnpauseAllPools(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::emergency_unpause_all_pools", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) SetProtocolFeeBps(tx *stx.Transaction, feeBps float64) stx.Argument {
	fee := uint64(math.Round(feeBps * utils.FloatScalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::set_protocol_fee_bps", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureU64(tx, fee), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) SetMinProtocolFee(tx *stx.Transaction, feeBps float64) stx.Argument {
	fee := uint64(math.Round(feeBps * utils.FloatScalar))
	return tx.MoveCall(c.config.MarginPackageID+"::margin_admin::set_min_protocol_fee", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureU64(tx, fee), tx.Object(c.adminCap()),
	}, nil)
}
