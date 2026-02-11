package transactions

import (
	"testing"

	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	stx "github.com/sui-sdks/go-sdks/sui/transactions"
)

func newTestConfig() *utils.DeepBookConfig {
	return utils.NewDeepBookConfig(utils.ConfigOptions{
		Address: "0x1",
		Network: "testnet",
		BalanceManagers: map[string]types.BalanceManager{
			"m1": {Address: "0x2"},
		},
		MarginManagers: map[string]types.MarginManager{
			"mm1": {Address: "0x3", PoolKey: "DEEP_SUI"},
			"mm2": {Address: "0x4", PoolKey: "DEEP_SUI"},
		},
	})
}

func firstFunction(tx *stx.Transaction) string {
	return commandFunction(tx, 0)
}

func commandFunction(tx *stx.Transaction, idx int) string {
	data := tx.GetData()
	if len(data.Commands) <= idx {
		return ""
	}
	mv := data.Commands[idx]["MoveCall"].(map[string]any)
	return mv["function"].(string)
}

func lastFunction(tx *stx.Transaction) string {
	data := tx.GetData()
	if len(data.Commands) == 0 {
		return ""
	}
	return commandFunction(tx, len(data.Commands)-1)
}

func TestBalanceManagerContract_MethodTargets(t *testing.T) {
	cfg := newTestConfig()
	contract := NewBalanceManagerContract(cfg)
	tx := stx.NewTransaction()
	contract.CheckManagerBalance(tx, "m1", "SUI")
	if got := firstFunction(tx); got != "balance" {
		t.Fatalf("expected balance, got %s", got)
	}
}

func TestDeepBookContract_MethodTargets(t *testing.T) {
	cfg := newTestConfig()
	bm := NewBalanceManagerContract(cfg)
	contract := NewDeepBookContract(cfg, bm)
	tx := stx.NewTransaction()
	contract.GetQuoteQuantityOut(tx, "DEEP_SUI", 1.0)
	if got := firstFunction(tx); got != "get_quote_quantity_out" {
		t.Fatalf("expected get_quote_quantity_out, got %s", got)
	}
}

func TestDeepBookAdminContract_MethodTargets(t *testing.T) {
	cfg := newTestConfig()
	cfg.AdminCap = "0x999"
	contract := NewDeepBookAdminContract(cfg)
	tx := stx.NewTransaction()
	contract.EnableVersion(tx, 1)
	if got := firstFunction(tx); got != "enable_version" {
		t.Fatalf("expected enable_version, got %s", got)
	}
}

func TestPoolProxyContract_MethodTargets(t *testing.T) {
	cfg := newTestConfig()
	contract := NewPoolProxyContract(cfg)
	tx := stx.NewTransaction()
	contract.CancelAllOrders(tx, "mm1")
	if got := firstFunction(tx); got != "cancel_all_orders" {
		t.Fatalf("expected cancel_all_orders, got %s", got)
	}
}

func TestDeepBookContract_MethodMatrix(t *testing.T) {
	cfg := newTestConfig()
	bm := NewBalanceManagerContract(cfg)
	c := NewDeepBookContract(cfg, bm)

	tests := []struct {
		name string
		call func(*stx.Transaction)
		want string
	}{
		{"CancelOrders", func(tx *stx.Transaction) { c.CancelOrders(tx, "DEEP_SUI", "m1", []string{"1", "2"}) }, "cancel_orders"},
		{"CancelAllOrders", func(tx *stx.Transaction) { c.CancelAllOrders(tx, "DEEP_SUI", "m1") }, "cancel_all_orders"},
		{"WithdrawSettledAmounts", func(tx *stx.Transaction) { c.WithdrawSettledAmounts(tx, "DEEP_SUI", "m1") }, "withdraw_settled_amounts"},
		{"GetOrder", func(tx *stx.Transaction) { c.GetOrder(tx, "DEEP_SUI", "1") }, "get_order"},
		{"GetOrders", func(tx *stx.Transaction) { c.GetOrders(tx, "DEEP_SUI", []string{"1", "2"}) }, "get_orders"},
		{"MidPrice", func(tx *stx.Transaction) { c.MidPrice(tx, "DEEP_SUI") }, "mid_price"},
		{"GetBaseQuantityOut", func(tx *stx.Transaction) { c.GetBaseQuantityOut(tx, "DEEP_SUI", 1) }, "get_base_quantity_out"},
		{"GetQuantityOutInputFee", func(tx *stx.Transaction) { c.GetQuantityOutInputFee(tx, "DEEP_SUI", 1, 0) }, "get_quantity_out_input_fee"},
		{"GetBaseQuantityIn", func(tx *stx.Transaction) { c.GetBaseQuantityIn(tx, "DEEP_SUI", 1, true) }, "get_base_quantity_in"},
		{"CanPlaceLimitOrder", func(tx *stx.Transaction) {
			c.CanPlaceLimitOrder(tx, types.CanPlaceLimitOrderParams{
				PoolKey: "DEEP_SUI", BalanceManagerKey: "m1", Price: 1, Quantity: 1, IsBid: true, PayWithDeep: true, ExpireTimestamp: 100,
			})
		}, "can_place_limit_order"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.call(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestGovernanceAndFlashLoanTargets(t *testing.T) {
	cfg := newTestConfig()
	bm := NewBalanceManagerContract(cfg)
	g := NewGovernanceContract(cfg, bm)
	f := NewFlashLoanContract(cfg)

	tx1 := stx.NewTransaction()
	g.Vote(tx1, "DEEP_SUI", "m1", "7")
	if got := lastFunction(tx1); got != "vote" {
		t.Fatalf("expected vote, got %s", got)
	}

	tx2 := stx.NewTransaction()
	f.ReturnBaseAsset(tx2, "DEEP_SUI", tx2.Object("0xa"), tx2.Object("0xb"))
	if got := firstFunction(tx2); got != "return_flashloan_base" {
		t.Fatalf("expected return_flashloan_base, got %s", got)
	}
}

func TestMarginContractsTargets(t *testing.T) {
	cfg := newTestConfig()

	mm := NewMarginManagerContract(cfg)
	tx1 := stx.NewTransaction()
	mm.GetMarginAccountOrderDetails(tx1, "mm1")
	if got := commandFunction(tx1, 0); got != "balance_manager" {
		t.Fatalf("expected first call balance_manager, got %s", got)
	}
	if got := commandFunction(tx1, 1); got != "get_account_order_details" {
		t.Fatalf("expected second call get_account_order_details, got %s", got)
	}

	mp := NewMarginPoolContract(cfg)
	tx2 := stx.NewTransaction()
	mp.InterestRate(tx2, "SUI")
	if got := firstFunction(tx2); got != "interest_rate" {
		t.Fatalf("expected interest_rate, got %s", got)
	}

	mr := NewMarginRegistryContract(cfg)
	tx3 := stx.NewTransaction()
	mr.GetMarginManagerIDs(tx3, "0x1")
	if got := firstFunction(tx3); got != "get_margin_manager_ids" {
		t.Fatalf("expected get_margin_manager_ids, got %s", got)
	}

	tpsl := NewMarginTPSLContract(cfg)
	tx4 := stx.NewTransaction()
	tpsl.CancelAllConditionalOrders(tx4, "mm1")
	if got := firstFunction(tx4); got != "cancel_all_conditional_orders" {
		t.Fatalf("expected cancel_all_conditional_orders, got %s", got)
	}

	tx5 := stx.NewTransaction()
	tpsl.CancelConditionalOrder(tx5, "mm1", "1")
	if got := firstFunction(tx5); got != "cancel_conditional_order" {
		t.Fatalf("expected cancel_conditional_order, got %s", got)
	}

	tx6 := stx.NewTransaction()
	tpsl.ExecuteConditionalOrders(tx6, "0x3", "DEEP_SUI", 5)
	if got := firstFunction(tx6); got != "execute_conditional_orders" {
		t.Fatalf("expected execute_conditional_orders, got %s", got)
	}

	tx7 := stx.NewTransaction()
	tpsl.ConditionalOrder(tx7, "DEEP_SUI", "0x3", "1")
	if got := firstFunction(tx7); got != "conditional_order" {
		t.Fatalf("expected conditional_order, got %s", got)
	}
}

func TestSwapMethods(t *testing.T) {
	cfg := newTestConfig()
	bm := NewBalanceManagerContract(cfg)
	db := NewDeepBookContract(cfg, bm)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"SwapExactBaseForQuote", func(tx *stx.Transaction) {
			db.SwapExactBaseForQuote(tx, types.SwapParams{
				PoolKey: "DEEP_SUI", Amount: 100, MinOut: 95,
			})
		}, "swap_exact_base_for_quote"},
		{"SwapExactQuoteForBase", func(tx *stx.Transaction) {
			db.SwapExactQuoteForBase(tx, types.SwapParams{
				PoolKey: "DEEP_SUI", Amount: 100, MinOut: 95,
			})
		}, "swap_exact_quote_for_base"},
		{"SwapExactQuantityBaseToQuote", func(tx *stx.Transaction) {
			db.SwapExactQuantityBaseToQuote(tx, types.SwapParams{
				PoolKey: "DEEP_SUI", Amount: 100, MinOut: 95,
			})
		}, "swap_exact_quantity"},
		{"SwapExactQuantityQuoteToBase", func(tx *stx.Transaction) {
			db.SwapExactQuantityQuoteToBase(tx, types.SwapParams{
				PoolKey: "DEEP_SUI", Amount: 100, MinOut: 95,
			})
		}, "swap_exact_quantity"},
		{"SwapExactQuantityBoth", func(tx *stx.Transaction) {
			db.SwapExactQuantityBoth(tx, types.SwapParams{
				PoolKey: "DEEP_SUI", Amount: 100, MinOut: 95,
			}, 50, 50)
		}, "swap_exact_quantity"},
		{"WithdrawSettledAmountsPermissionless", func(tx *stx.Transaction) {
			db.WithdrawSettledAmountsPermissionless(tx, "DEEP_SUI", "m1", "0x999")
		}, "withdraw_settled_amounts_permissionless"},
		{"UpdatePoolReferralMultiplier", func(tx *stx.Transaction) {
			db.UpdatePoolReferralMultiplier(tx, "DEEP_SUI", "0x123", 0.5)
		}, "update_pool_referral_multiplier"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestMarginPoolMethods(t *testing.T) {
	cfg := newTestConfig()
	mp := NewMarginPoolContract(cfg)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"SupplyToMarginPool", func(tx *stx.Transaction) {
			mp.SupplyToMarginPool(tx, "SUI", 100)
		}, "supply"},
		{"WithdrawFromMarginPool", func(tx *stx.Transaction) {
			mp.WithdrawFromMarginPool(tx, "SUI", 50)
		}, "withdraw"},
		{"MintSupplyReferral", func(tx *stx.Transaction) {
			mp.MintSupplyReferral(tx, "SUI")
		}, "mint_supply_referral"},
		{"WithdrawReferralFees", func(tx *stx.Transaction) {
			mp.WithdrawReferralFees(tx, "SUI", "0x123")
		}, "withdraw_referral_fees"},
		{"UpdateInterestWeight", func(tx *stx.Transaction) {
			mp.UpdateInterestWeight(tx, "SUI", 0.05)
		}, "update_interest_weight"},
		{"SetMaxUtilizationRate", func(tx *stx.Transaction) {
			mp.SetMaxUtilizationRate(tx, "SUI", 0.8)
		}, "set_max_utilization_rate"},
		{"GetUserSupplyShares", func(tx *stx.Transaction) {
			mp.GetUserSupplyShares(tx, "SUI", "0x123")
		}, "user_supply_shares"},
		{"GetUserBorrowShares", func(tx *stx.Transaction) {
			mp.GetUserBorrowShares(tx, "SUI", "0x123")
		}, "user_borrow_shares"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestMarginLiquidationsMethods(t *testing.T) {
	cfg := newTestConfig()
	cfg.MarginMaintainerCap = "0x999"
	ml := NewMarginLiquidationsContract(cfg)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"CreateLiquidationVault", func(tx *stx.Transaction) {
			ml.CreateLiquidationVault(tx, "0x999")
		}, "create_liquidation_vault"},
		{"Deposit", func(tx *stx.Transaction) {
			ml.Deposit(tx, "0x123", "SUI", 100)
		}, "deposit"},
		{"Withdraw", func(tx *stx.Transaction) {
			ml.Withdraw(tx, "0x123", "SUI", 50)
		}, "withdraw"},
		{"LiquidateBase", func(tx *stx.Transaction) {
			ml.LiquidateBase(tx, "0x123", "mm1", "DEEP_SUI", 100)
		}, "liquidate_base"},
		{"LiquidateQuote", func(tx *stx.Transaction) {
			ml.LiquidateQuote(tx, "0x123", "mm1", "DEEP_SUI", 100)
		}, "liquidate_quote"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestMarginManagerMethods(t *testing.T) {
	cfg := newTestConfig()
	mm := NewMarginManagerContract(cfg)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"ShareMarginManager", func(tx *stx.Transaction) {
			mm.ShareMarginManager(tx, "DEEP_SUI", "mm1")
		}, "share_margin_manager"},
		{"WithdrawDeep", func(tx *stx.Transaction) {
			mm.WithdrawDeep(tx, "mm1", "DEEP_SUI", 100)
		}, "withdraw_deep"},
		{"Liquidate", func(tx *stx.Transaction) {
			mm.Liquidate(tx, "mm1", "mm2", "DEEP_SUI", true)
		}, "liquidate"},
		{"HasQuoteDebt", func(tx *stx.Transaction) {
			mm.HasQuoteDebt(tx, "DEEP_SUI", "0x3")
		}, "has_quote_debt"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestPoolProxyMethods(t *testing.T) {
	cfg := newTestConfig()
	pp := NewPoolProxyContract(cfg)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"PlaceReduceOnlyLimitOrder", func(tx *stx.Transaction) {
			pp.PlaceReduceOnlyLimitOrder(tx, types.PlaceMarginLimitOrderParams{
				MarginManagerKey: "mm1", ClientOrderID: "1", Price: 1.0, Quantity: 100, IsBid: true,
			})
		}, "place_reduce_only_limit_order"},
		{"PlaceReduceOnlyMarketOrder", func(tx *stx.Transaction) {
			pp.PlaceReduceOnlyMarketOrder(tx, types.PlaceMarginMarketOrderParams{
				MarginManagerKey: "mm1", ClientOrderID: "1", Quantity: 100, IsBid: true,
			})
		}, "place_reduce_only_market_order"},
		{"ModifyOrder", func(tx *stx.Transaction) {
			pp.ModifyOrder(tx, "mm1", "123456789", 50)
		}, "modify_order"},
		{"SubmitProposal", func(tx *stx.Transaction) {
			pp.SubmitProposal(tx, "mm1", types.ProposalParams{
				PoolKey: "DEEP_SUI", TakerFee: 0.001, MakerFee: 0.001, StakeRequired: 1000,
			})
		}, "submit_proposal"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestMarginAdminMethods(t *testing.T) {
	cfg := newTestConfig()
	cfg.MarginAdminCap = "0x999"
	ma := NewMarginAdminContract(cfg)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"MintMaintainerCap", func(tx *stx.Transaction) {
			ma.MintMaintainerCap(tx)
		}, "mint_maintainer_cap"},
		{"PauseMarginManager", func(tx *stx.Transaction) {
			ma.PauseMarginManager(tx, "mm1")
		}, "pause_margin_manager"},
		{"RegisterDeepbookPool", func(tx *stx.Transaction) {
			ma.RegisterDeepbookPool(tx, "DEEP_SUI")
		}, "register_deepbook_pool"},
		{"PausePool", func(tx *stx.Transaction) {
			ma.PausePool(tx, "DEEP_SUI")
		}, "pause_pool"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestMarginMaintainerMethods(t *testing.T) {
	cfg := newTestConfig()
	cfg.MarginMaintainerCap = "0x999"
	mm := NewMarginMaintainerContract(cfg)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"NewProtocolConfig", func(tx *stx.Transaction) {
			mm.NewProtocolConfig(tx)
		}, "new_protocol_config"},
		{"UpdateInterestParams", func(tx *stx.Transaction) {
			mm.UpdateInterestParams(tx, "SUI", types.InterestConfigParams{})
		}, "update_interest_params"},
		{"EnableDeepbookPoolForLoan", func(tx *stx.Transaction) {
			mm.EnableDeepbookPoolForLoan(tx, "DEEP_SUI")
		}, "enable_deepbook_pool_for_loan"},
		{"CreateLiquidationVault", func(tx *stx.Transaction) {
			mm.CreateLiquidationVault(tx)
		}, "create_liquidation_vault"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestBalanceManagerMethods(t *testing.T) {
	cfg := newTestConfig()
	bm := NewBalanceManagerContract(cfg)

	tests := []struct {
		name string
		fn   func(*stx.Transaction)
		want string
	}{
		{"DepositWithCap", func(tx *stx.Transaction) {
			bm.DepositWithCap(tx, "m1", "SUI", 100, "0x123")
		}, "deposit_with_cap"},
		{"WithdrawWithCap", func(tx *stx.Transaction) {
			bm.WithdrawWithCap(tx, "m1", "SUI", 100, "0x123")
		}, "withdraw_with_cap"},
		{"SetBalanceManagerReferral", func(tx *stx.Transaction) {
			bm.SetBalanceManagerReferral(tx, "m1", "0xabc")
		}, "set_balance_manager_referral"},
		{"UnsetBalanceManagerReferral", func(tx *stx.Transaction) {
			bm.UnsetBalanceManagerReferral(tx, "m1")
		}, "unset_balance_manager_referral"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tx := stx.NewTransaction()
			tc.fn(tx)
			if got := lastFunction(tx); got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}
