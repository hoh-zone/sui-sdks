package transactions

import (
	"math"

	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	stx "github.com/sui-sdks/go-sdks/sui/transactions"
)

func (c *MarginMaintainerContract) maintainerCap() string {
	if c.config.MarginMaintainerCap == "" {
		panic(&utils.ConfigurationError{DeepBookError: utils.DeepBookError{Msg: utils.ErrorMessages.MarginMaintainerCapNotSet}})
	}
	return c.config.MarginMaintainerCap
}

func (c *MarginMaintainerContract) NewProtocolConfig(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::maintainer::new_protocol_config", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.maintainerCap()),
	}, nil)
}

func (c *MarginMaintainerContract) UpdateInterestParams(tx *stx.Transaction, coinKey string, params types.InterestConfigParams) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	baseRate := uint64(math.Round(params.BaseRate * utils.FloatScalar))
	baseSlope := uint64(math.Round(params.BaseSlope * utils.FloatScalar))
	optimalUtilization := uint64(math.Round(params.OptimalUtilization * utils.FloatScalar))
	excessSlope := uint64(math.Round(params.ExcessSlope * utils.FloatScalar))
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::maintainer::update_interest_params", []stx.Argument{
		object,
		pureU64(tx, baseRate),
		pureU64(tx, baseSlope),
		pureU64(tx, optimalUtilization),
		pureU64(tx, excessSlope),
		tx.Object(c.maintainerCap()),
	}, []string{coin.Type})
}

func (c *MarginMaintainerContract) EnableDeepbookPoolForLoan(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::maintainer::enable_deepbook_pool_for_loan", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.config.MarginRegistryID), tx.Object(c.maintainerCap()),
	}, nil)
}

func (c *MarginMaintainerContract) DisableDeepbookPoolForLoan(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	return tx.MoveCall(c.config.MarginPackageID+"::maintainer::disable_deepbook_pool_for_loan", []stx.Argument{
		tx.Object(pool.Address), tx.Object(c.config.MarginRegistryID), tx.Object(c.maintainerCap()),
	}, nil)
}

func (c *MarginMaintainerContract) SetProtocolConfigs(tx *stx.Transaction, poolKey string, params types.PoolConfigParams) stx.Argument {
	pool := c.config.GetPool(poolKey)
	minWithdrawRiskRatio := uint64(math.Round(params.MinWithdrawRiskRatio * utils.FloatScalar))
	minBorrowRiskRatio := uint64(math.Round(params.MinBorrowRiskRatio * utils.FloatScalar))
	liquidationRiskRatio := uint64(math.Round(params.LiquidationRiskRatio * utils.FloatScalar))
	targetLiquidationRiskRatio := uint64(math.Round(params.TargetLiquidationRiskRatio * utils.FloatScalar))
	userLiquidationReward := uint64(math.Round(params.UserLiquidationReward * utils.FloatScalar))
	poolLiquidationReward := uint64(math.Round(params.PoolLiquidationReward * utils.FloatScalar))
	return tx.MoveCall(c.config.MarginPackageID+"::maintainer::set_protocol_configs", []stx.Argument{
		tx.Object(pool.Address),
		pureU64(tx, minWithdrawRiskRatio),
		pureU64(tx, minBorrowRiskRatio),
		pureU64(tx, liquidationRiskRatio),
		pureU64(tx, targetLiquidationRiskRatio),
		pureU64(tx, userLiquidationReward),
		pureU64(tx, poolLiquidationReward),
		tx.Object(c.config.MarginRegistryID),
		tx.Object(c.maintainerCap()),
	}, nil)
}

func (c *MarginMaintainerContract) SetMarginPoolConfigs(tx *stx.Transaction, coinKey string, params types.MarginPoolConfigParams) stx.Argument {
	marginPool, ok := c.config.MarginPools[coinKey]
	coin := c.config.GetCoin(coinKey)
	supplyCap := uint64(math.Round(params.SupplyCap * coin.Scalar))
	maxUtilizationRate := uint64(math.Round(params.MaxUtilizationRate * utils.FloatScalar))
	referralSpread := uint64(math.Round(params.ReferralSpread * utils.FloatScalar))
	minBorrow := uint64(math.Round(params.MinBorrow * coin.Scalar))
	object := tx.Object(c.config.MarginRegistryID)
	if ok {
		object = tx.Object(marginPool.Address)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::maintainer::set_margin_pool_configs", []stx.Argument{
		object,
		pureU64(tx, supplyCap),
		pureU64(tx, maxUtilizationRate),
		pureU64(tx, referralSpread),
		pureU64(tx, minBorrow),
		pureU64(tx, uint64(math.Round(params.RateLimitCapacity))),
		pureU64(tx, uint64(math.Round(params.RateLimitRefillRatePerMs))),
		pureBool(tx, params.RateLimitEnabled),
		tx.Object(c.maintainerCap()),
	}, []string{coin.Type})
}

func (c *MarginMaintainerContract) CreateLiquidationVault(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::create_liquidation_vault", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.maintainerCap()),
	}, nil)
}

func (c *MarginMaintainerContract) LiquidationVaultConfig(tx *stx.Transaction, vaultID string) stx.Argument {
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::config", []stx.Argument{
		tx.Object(vaultID),
	}, nil)
}

func (c *MarginMaintainerContract) SetLiquidationVaultConfig(tx *stx.Transaction, vaultID string, marginRate float64) stx.Argument {
	margin := uint64(math.Round(marginRate * utils.FloatScalar))
	return tx.MoveCall(c.config.LiquidationPackageID+"::liquidation_vault::set_config", []stx.Argument{
		tx.Object(vaultID), pureU64(tx, margin), tx.Object(c.maintainerCap()),
	}, nil)
}

func (c *MarginMaintainerContract) CreateMarginPool(tx *stx.Transaction, coinKey string, poolConfig stx.Argument) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::create_margin_pool", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), poolConfig, tx.Object(c.maintainerCap()), tx.Object("0x6"),
	}, []string{coin.Type})
}

func (c *MarginMaintainerContract) NewMarginPoolConfig(tx *stx.Transaction, coinKey string, params types.MarginPoolConfigParams) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::protocol_config::new_margin_pool_config", []stx.Argument{
		pureU64(tx, uint64(math.Round(params.SupplyCap*coin.Scalar))),
		pureU64(tx, uint64(math.Round(params.MaxUtilizationRate*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.ReferralSpread*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.MinBorrow*coin.Scalar))),
	}, nil)
}

func (c *MarginMaintainerContract) NewMarginPoolConfigWithRateLimit(tx *stx.Transaction, coinKey string, params types.MarginPoolConfigParams) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::protocol_config::new_margin_pool_config_with_rate_limit", []stx.Argument{
		pureU64(tx, uint64(math.Round(params.SupplyCap*coin.Scalar))),
		pureU64(tx, uint64(math.Round(params.MaxUtilizationRate*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.ReferralSpread*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.MinBorrow*coin.Scalar))),
		pureU64(tx, uint64(math.Round(params.RateLimitCapacity*coin.Scalar))),
		pureU64(tx, uint64(math.Round(params.RateLimitRefillRatePerMs*coin.Scalar))),
		pureBool(tx, params.RateLimitEnabled),
	}, nil)
}

func (c *MarginMaintainerContract) NewInterestConfig(tx *stx.Transaction, params types.InterestConfigParams) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::protocol_config::new_interest_config", []stx.Argument{
		pureU64(tx, uint64(math.Round(params.BaseRate*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.BaseSlope*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.OptimalUtilization*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.ExcessSlope*utils.FloatScalar))),
	}, nil)
}

func (c *MarginMaintainerContract) NewProtocolConfigWithParams(tx *stx.Transaction, coinKey string, marginPoolConfig types.MarginPoolConfigParams, interestConfig types.InterestConfigParams) stx.Argument {
	marginCfg := c.NewMarginPoolConfig(tx, coinKey, marginPoolConfig)
	if marginPoolConfig.RateLimitEnabled || marginPoolConfig.RateLimitCapacity > 0 || marginPoolConfig.RateLimitRefillRatePerMs > 0 {
		marginCfg = c.NewMarginPoolConfigWithRateLimit(tx, coinKey, marginPoolConfig)
	}
	interestCfg := c.NewInterestConfig(tx, interestConfig)
	return tx.MoveCall(c.config.MarginPackageID+"::protocol_config::new_protocol_config", []stx.Argument{
		marginCfg, interestCfg,
	}, nil)
}

func (c *MarginMaintainerContract) UpdateInterestParamsWithCap(tx *stx.Transaction, coinKey, marginPoolCap string, params types.InterestConfigParams) stx.Argument {
	marginPool := c.config.GetMarginPool(coinKey)
	interestCfg := c.NewInterestConfig(tx, params)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::update_interest_params", []stx.Argument{
		tx.Object(marginPool.Address), tx.Object(c.config.MarginRegistryID), interestCfg, tx.Object(marginPoolCap), tx.Object("0x6"),
	}, []string{marginPool.Type})
}

func (c *MarginMaintainerContract) UpdateMarginPoolConfig(tx *stx.Transaction, coinKey, marginPoolCap string, params types.MarginPoolConfigParams) stx.Argument {
	marginPool := c.config.GetMarginPool(coinKey)
	marginCfg := c.NewMarginPoolConfig(tx, coinKey, params)
	if params.RateLimitEnabled || params.RateLimitCapacity > 0 || params.RateLimitRefillRatePerMs > 0 {
		marginCfg = c.NewMarginPoolConfigWithRateLimit(tx, coinKey, params)
	}
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::update_margin_pool_config", []stx.Argument{
		tx.Object(marginPool.Address), tx.Object(c.config.MarginRegistryID), marginCfg, tx.Object(marginPoolCap), tx.Object("0x6"),
	}, []string{marginPool.Type})
}

func (c *MarginMaintainerContract) EnableDeepbookPoolForLoanWithCap(tx *stx.Transaction, deepbookPoolKey, coinKey, marginPoolCap string) stx.Argument {
	deepbookPool := c.config.GetPool(deepbookPoolKey)
	marginPool := c.config.GetMarginPool(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::enable_deepbook_pool_for_loan", []stx.Argument{
		tx.Object(marginPool.Address), tx.Object(c.config.MarginRegistryID), pureAddress(tx, deepbookPool.Address), tx.Object(marginPoolCap), tx.Object("0x6"),
	}, []string{marginPool.Type})
}

func (c *MarginMaintainerContract) DisableDeepbookPoolForLoanWithCap(tx *stx.Transaction, deepbookPoolKey, coinKey, marginPoolCap string) stx.Argument {
	deepbookPool := c.config.GetPool(deepbookPoolKey)
	marginPool := c.config.GetMarginPool(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::disable_deepbook_pool_for_loan", []stx.Argument{
		tx.Object(marginPool.Address), tx.Object(c.config.MarginRegistryID), pureAddress(tx, deepbookPool.Address), tx.Object(marginPoolCap), tx.Object("0x6"),
	}, []string{marginPool.Type})
}
