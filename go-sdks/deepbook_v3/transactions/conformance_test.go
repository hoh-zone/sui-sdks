package transactions

import (
	"testing"

	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	stx "github.com/sui-sdks/go-sdks/sui/transactions"
)

func TestConformance_GovernanceTargets(t *testing.T) {
	cfg := newTestConfig()
	bm := NewBalanceManagerContract(cfg)
	g := NewGovernanceContract(cfg, bm)

	tests := []struct {
		name string
		call func(*stx.Transaction)
		want string
	}{
		{"Stake", func(tx *stx.Transaction) { g.Stake(tx, "DEEP_SUI", "m1", 10) }, "stake"},
		{"Unstake", func(tx *stx.Transaction) { g.Unstake(tx, "DEEP_SUI", "m1") }, "unstake"},
		{"SubmitProposal", func(tx *stx.Transaction) {
			g.SubmitProposal(tx, types.ProposalParams{
				PoolKey: "DEEP_SUI", BalanceManagerKey: "m1", TakerFee: 0.001, MakerFee: 0.001, StakeRequired: 10,
			})
		}, "submit_proposal"},
		{"Vote", func(tx *stx.Transaction) { g.Vote(tx, "DEEP_SUI", "m1", "0x7") }, "vote"},
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

func TestConformance_NewParityMethods(t *testing.T) {
	cfg := newTestConfig()
	cfg.AdminCap = "0x999"
	cfg.MarginAdminCap = "0x998"
	cfg.MarginMaintainerCap = "0x997"

	bm := NewBalanceManagerContract(cfg)
	db := NewDeepBookContract(cfg, bm)
	dba := NewDeepBookAdminContract(cfg)
	mm := NewMarginManagerContract(cfg)
	mr := NewMarginRegistryContract(cfg)
	pp := NewPoolProxyContract(cfg)

	tests := []struct {
		name string
		call func(*stx.Transaction)
		want string
	}{
		{"AdjustMinLotSize", func(tx *stx.Transaction) {
			dba.AdjustMinLotSize(tx, "DEEP_SUI", 1, 1)
		}, "adjust_min_lot_size_admin"},
		{"CreatePermissionlessPool", func(tx *stx.Transaction) {
			db.CreatePermissionlessPool(tx, types.CreatePermissionlessPoolParams{
				BaseCoinKey: "DEEP", QuoteCoinKey: "SUI", TickSize: 0.01, LotSize: 1, MinSize: 1,
			})
		}, "create_permissionless_pool"},
		{"DepositDeep", func(tx *stx.Transaction) {
			mm.DepositDeep(tx, types.DepositParams{ManagerKey: "mm1", Amount: 5})
		}, "deposit"},
		{"GetDeepbookPoolMarginPoolIDs", func(tx *stx.Transaction) {
			mr.GetDeepbookPoolMarginPoolIDs(tx, "DEEP_SUI")
		}, "get_deepbook_pool_margin_pool_ids"},
		{"WithdrawMarginSettledAmounts", func(tx *stx.Transaction) {
			pp.WithdrawMarginSettledAmounts(tx, "DEEP_SUI", "0x3")
		}, "withdraw_settled_amounts_permissionless"},
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
