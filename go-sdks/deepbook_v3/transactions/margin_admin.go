package transactions

import (
	"encoding/hex"
	"math"
	"strings"

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

func (c *MarginAdminContract) RevokeMaintainerCap(tx *stx.Transaction, maintainerCapID string) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::revoke_maintainer_cap", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), tx.Object(maintainerCapID), tx.Object("0x6"),
	}, nil)
}

func (c *MarginAdminContract) RegisterDeepbookPoolWithConfig(tx *stx.Transaction, poolKey string, poolConfig stx.Argument) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::register_deepbook_pool", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), tx.Object(pool.Address), poolConfig, tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) EnableDeepbookPool(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::enable_deepbook_pool", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), tx.Object(pool.Address), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) DisableDeepbookPool(tx *stx.Transaction, poolKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::disable_deepbook_pool", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), tx.Object(pool.Address), tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) UpdateRiskParams(tx *stx.Transaction, poolKey string, poolConfig stx.Argument) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::update_risk_params", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), tx.Object(pool.Address), poolConfig, tx.Object("0x6"),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) AddConfig(tx *stx.Transaction, config stx.Argument) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::add_config", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), config,
	}, []string{c.config.MarginPackageID + "::oracle::PythConfig"})
}

func (c *MarginAdminContract) RemoveConfig(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::remove_config", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()),
	}, []string{c.config.MarginPackageID + "::oracle::PythConfig"})
}

func (c *MarginAdminContract) EnableVersion(tx *stx.Transaction, version uint64) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::enable_version", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureU64(tx, version), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) DisableVersion(tx *stx.Transaction, version uint64) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::disable_version", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureU64(tx, version), tx.Object(c.adminCap()),
	}, nil)
}

func (c *MarginAdminContract) NewPoolConfig(tx *stx.Transaction, poolKey string, params types.PoolConfigParams) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::new_pool_config", []stx.Argument{
		tx.Object(c.config.MarginRegistryID),
		pureU64(tx, uint64(math.Round(params.MinWithdrawRiskRatio*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.MinBorrowRiskRatio*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.LiquidationRiskRatio*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.TargetLiquidationRiskRatio*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.UserLiquidationReward*utils.FloatScalar))),
		pureU64(tx, uint64(math.Round(params.PoolLiquidationReward*utils.FloatScalar))),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) NewPoolConfigWithLeverage(tx *stx.Transaction, poolKey string, leverage float64) stx.Argument {
	pool := c.config.GetPool(poolKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::new_pool_config_with_leverage", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureU64(tx, uint64(math.Round(leverage*utils.FloatScalar))),
	}, []string{base.Type, quote.Type})
}

func (c *MarginAdminContract) NewCoinTypeData(tx *stx.Transaction, coinKey string, maxConfBps, maxEwmaDifferenceBps uint64) stx.Argument {
	coin := c.config.GetCoin(coinKey)
	feed := strings.TrimPrefix(coin.Feed, "0x")
	feedBytes, _ := hex.DecodeString(feed)
	return tx.MoveCall(c.config.MarginPackageID+"::oracle::new_coin_type_data_from_currency", []stx.Argument{
		tx.Object(coin.CurrencyID), tx.PureBytes(feedBytes), pureU64(tx, maxConfBps), pureU64(tx, maxEwmaDifferenceBps),
	}, []string{coin.Type})
}

func (c *MarginAdminContract) NewPythConfig(tx *stx.Transaction, setups []types.CoinTypeSetup, maxAgeSeconds uint64) stx.Argument {
	args := make([]stx.Argument, 0, len(setups))
	for _, s := range setups {
		args = append(args, c.NewCoinTypeData(tx, s.CoinKey, s.MaxConfBps, s.MaxEwmaDifferenceBps))
	}
	vec := tx.AddCommand(stx.TransactionCommands.MakeMoveVec(nil, args))
	return tx.MoveCall(c.config.MarginPackageID+"::oracle::new_pyth_config", []stx.Argument{
		vec, pureU64(tx, maxAgeSeconds),
	}, nil)
}

func (c *MarginAdminContract) MintPauseCap(tx *stx.Transaction) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::mint_pause_cap", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), tx.Object("0x6"),
	}, nil)
}

func (c *MarginAdminContract) RevokePauseCap(tx *stx.Transaction, pauseCapID string) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::revoke_pause_cap", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()), tx.Object("0x6"), pureAddress(tx, pauseCapID),
	}, nil)
}

func (c *MarginAdminContract) DisableVersionPauseCap(tx *stx.Transaction, version uint64, pauseCapID string) stx.Argument {
	return tx.MoveCall(c.config.MarginPackageID+"::margin_registry::disable_version_pause_cap", []stx.Argument{
		tx.Object(c.config.MarginRegistryID), pureU64(tx, version), tx.Object(pauseCapID),
	}, nil)
}

func (c *MarginAdminContract) AdminWithdrawDefaultReferralFees(tx *stx.Transaction, coinKey string) stx.Argument {
	marginPool := c.config.GetMarginPool(coinKey)
	coin := c.config.GetCoin(coinKey)
	return tx.MoveCall(c.config.MarginPackageID+"::margin_pool::admin_withdraw_default_referral_fees", []stx.Argument{
		tx.Object(marginPool.Address), tx.Object(c.config.MarginRegistryID), tx.Object(c.adminCap()),
	}, []string{coin.Type})
}
