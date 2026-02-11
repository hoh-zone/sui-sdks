package deepbookv3

import (
	"context"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"strings"
	"testing"

	"github.com/sui-sdks/go-sdks/bcs"
	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	suiutils "github.com/sui-sdks/go-sdks/sui/utils"
)

type deepbookMethodMockClient struct{}

func (m deepbookMethodMockClient) Network() string { return "testnet" }

func encodeU64(v uint64) string {
	w := bcs.NewWriter(nil)
	_ = w.Write64(v)
	return base64.StdEncoding.EncodeToString(w.ToBytes())
}

func encodeAddressHex(addr string) string {
	norm := suiutils.NormalizeSuiAddress(addr)
	raw, _ := hex.DecodeString(strings.TrimPrefix(norm, "0x"))
	return base64.StdEncoding.EncodeToString(raw)
}

func encodeVecAddress(addrs []string) string {
	w := bcs.NewWriter(nil)
	_ = w.WriteULEB(uint64(len(addrs)))
	for _, addr := range addrs {
		norm := suiutils.NormalizeSuiAddress(addr)
		raw, _ := hex.DecodeString(strings.TrimPrefix(norm, "0x"))
		_ = w.WriteBytes(raw)
	}
	return base64.StdEncoding.EncodeToString(w.ToBytes())
}

func encodeVecU64(vals []uint64) string {
	w := bcs.NewWriter(nil)
	_ = w.WriteULEB(uint64(len(vals)))
	for _, v := range vals {
		_ = w.Write64(v)
	}
	return base64.StdEncoding.EncodeToString(w.ToBytes())
}

func encodeOptionAddress(addr string) string {
	w := bcs.NewWriter(nil)
	_ = w.Write8(1)
	norm := suiutils.NormalizeSuiAddress(addr)
	raw, _ := hex.DecodeString(strings.TrimPrefix(norm, "0x"))
	_ = w.WriteBytes(raw)
	return base64.StdEncoding.EncodeToString(w.ToBytes())
}

func moveFunctionFromTxBase64(txb64 string) string {
	raw, err := base64.StdEncoding.DecodeString(txb64)
	if err != nil {
		return ""
	}
	var payload map[string]any
	if err := json.Unmarshal(raw, &payload); err != nil {
		return ""
	}
	cmds, ok := payload["Commands"].([]any)
	if !ok || len(cmds) == 0 {
		return ""
	}
	cmd, ok := cmds[0].(map[string]any)
	if !ok {
		return ""
	}
	mv, ok := cmd["MoveCall"].(map[string]any)
	if !ok {
		return ""
	}
	fn, _ := mv["function"].(string)
	return fn
}

func (m deepbookMethodMockClient) Call(ctx context.Context, method string, params []any, out any) error {
	_ = ctx
	if method != "sui_dryRunTransactionBlock" {
		return nil
	}
	fn := ""
	if len(params) > 0 {
		if txb64, ok := params[0].(string); ok {
			fn = moveFunctionFromTxBase64(txb64)
		}
	}

	firstRet := []any{map[string]any{"bcs": encodeU64(100)}}
	if fn == "whitelisted" || fn == "stable_pool" || fn == "registered_pool" || fn == "account_exists" || fn == "can_place_limit_order" || fn == "can_place_market_order" || fn == "check_market_order_params" || fn == "check_limit_order_params" || fn == "pool_enabled" {
		firstRet = []any{map[string]any{"bcs": base64.StdEncoding.EncodeToString([]byte{1})}}
	}
	if fn == "get_quote_quantity_out" || fn == "get_base_quantity_out" || fn == "get_quantity_out" {
		firstRet = []any{
			map[string]any{"bcs": encodeU64(100)},
			map[string]any{"bcs": encodeU64(200)},
			map[string]any{"bcs": encodeU64(300)},
		}
	}
	if fn == "get_pool_referral_balances" {
		firstRet = []any{
			map[string]any{"bcs": encodeU64(100)},
			map[string]any{"bcs": encodeU64(200)},
			map[string]any{"bcs": encodeU64(300)},
		}
	}
	if fn == "pool_referral_multiplier" || fn == "quorum" || fn == "lowest_trigger_above_price" || fn == "highest_trigger_below_price" {
		firstRet = []any{map[string]any{"bcs": encodeU64(2_000_000_000)}}
	}
	if fn == "id" || fn == "balance_manager_referral_owner" || fn == "balance_manager_referral_pool_id" {
		firstRet = []any{map[string]any{"bcs": encodeAddressHex("0x123")}}
	}
	if fn == "get_balance_manager_referral_id" {
		firstRet = []any{map[string]any{"bcs": encodeOptionAddress("0xabc")}}
	}
	if fn == "get_balance_manager_ids" || fn == "get_margin_manager_ids" {
		firstRet = []any{map[string]any{"bcs": encodeVecAddress([]string{"0x111", "0x222"})}}
	}
	if fn == "conditional_order_ids" {
		firstRet = []any{map[string]any{"bcs": encodeVecU64([]uint64{1, 2, 3})}}
	}
	if fn == "get_level2_range" || fn == "get_level2_ticks_from_mid" {
		firstRet = []any{map[string]any{"bcs": base64.StdEncoding.EncodeToString([]byte{0x0a, 0x0b})}}
	}

	if p, ok := out.(*map[string]any); ok {
		*p = map[string]any{
			"commandResults": []any{
				map[string]any{"returnValues": firstRet},
				map[string]any{"returnValues": []any{map[string]any{"bcs": encodeU64(777)}}},
			},
		}
	}
	return nil
}

func newMethodTestClient() *Client {
	return NewClient(ClientOptions{
		Client:  deepbookMethodMockClient{},
		Network: "testnet",
		Options: Options{
			Address: "0x1",
			BalanceManagers: map[string]types.BalanceManager{
				"m1": {Address: "0x2"},
			},
			MarginManagers: map[string]types.MarginManager{
				"mm1": {Address: "0x3", PoolKey: "DEEP_SUI"},
			},
		},
	})
}

func TestDeepBookClientQuantityAndPriceMethods(t *testing.T) {
	c := newMethodTestClient()
	ctx := context.Background()

	q1, err := c.GetQuoteQuantityOut(ctx, "DEEP_SUI", 1)
	if err != nil || q1["deepRequired"] == nil {
		t.Fatalf("GetQuoteQuantityOut failed: %v", err)
	}
	q2, err := c.GetBaseQuantityOut(ctx, "DEEP_SUI", 1)
	if err != nil || q2["deepRequired"] == nil {
		t.Fatalf("GetBaseQuantityOut failed: %v", err)
	}
	q3, err := c.GetQuantityOut(ctx, "DEEP_SUI", 1, 0)
	if err != nil || q3["deepRequired"] == nil {
		t.Fatalf("GetQuantityOut failed: %v", err)
	}

	mid, err := c.MidPrice(ctx, "DEEP_SUI")
	if err != nil || mid <= 0 {
		t.Fatalf("MidPrice failed: %v, value=%v", err, mid)
	}
}

func TestDeepBookClientInputFeeAndInMethods(t *testing.T) {
	c := newMethodTestClient()
	ctx := context.Background()

	if _, err := c.GetQuoteQuantityOutInputFee(ctx, "DEEP_SUI", 1); err != nil {
		t.Fatalf("GetQuoteQuantityOutInputFee failed: %v", err)
	}
	if _, err := c.GetBaseQuantityOutInputFee(ctx, "DEEP_SUI", 1); err != nil {
		t.Fatalf("GetBaseQuantityOutInputFee failed: %v", err)
	}
	if _, err := c.GetQuantityOutInputFee(ctx, "DEEP_SUI", 1, 0); err != nil {
		t.Fatalf("GetQuantityOutInputFee failed: %v", err)
	}
	if _, err := c.GetBaseQuantityIn(ctx, "DEEP_SUI", 1, true); err != nil {
		t.Fatalf("GetBaseQuantityIn failed: %v", err)
	}
	if _, err := c.GetQuoteQuantityIn(ctx, "DEEP_SUI", 1, false); err != nil {
		t.Fatalf("GetQuoteQuantityIn failed: %v", err)
	}
	if _, err := c.GetOrderDeepRequired(ctx, "DEEP_SUI", 1, 1); err != nil {
		t.Fatalf("GetOrderDeepRequired failed: %v", err)
	}
}

func TestDeepBookClientRawBCSMethods(t *testing.T) {
	c := newMethodTestClient()
	ctx := context.Background()

	cases := []struct {
		name string
		call func() (string, error)
	}{
		{"GetOrder", func() (string, error) { return c.GetOrder(ctx, "DEEP_SUI", "1") }},
		{"GetOrders", func() (string, error) { return c.GetOrders(ctx, "DEEP_SUI", []string{"1"}) }},
		{"AccountOpenOrders", func() (string, error) { return c.AccountOpenOrders(ctx, "DEEP_SUI", "m1") }},
		{"VaultBalances", func() (string, error) { return c.VaultBalances(ctx, "DEEP_SUI") }},
		{"GetPoolIDByAssets", func() (string, error) { return c.GetPoolIDByAssets(ctx, "0x2::sui::SUI", "0x3::coin::C") }},
		{"PoolTradeParams", func() (string, error) { return c.PoolTradeParams(ctx, "DEEP_SUI") }},
		{"PoolBookParams", func() (string, error) { return c.PoolBookParams(ctx, "DEEP_SUI") }},
		{"Account", func() (string, error) { return c.Account(ctx, "DEEP_SUI", "m1") }},
		{"LockedBalance", func() (string, error) { return c.LockedBalance(ctx, "DEEP_SUI", "m1") }},
		{"GetPoolDeepPrice", func() (string, error) { return c.GetPoolDeepPrice(ctx, "DEEP_SUI") }},
		{"BalanceManagerReferralOwner", func() (string, error) { return c.BalanceManagerReferralOwner(ctx, "0xaaa") }},
		{"GetMarginAccountOrderDetails", func() (string, error) { return c.GetMarginAccountOrderDetails(ctx, "mm1") }},
		{"GetAccountOrderDetails", func() (string, error) { return c.GetAccountOrderDetails(ctx, "DEEP_SUI", "m1") }},
		{"PoolTradeParamsNext", func() (string, error) { return c.PoolTradeParamsNext(ctx, "DEEP_SUI") }},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			v, err := tc.call()
			if err != nil {
				t.Fatalf("%s failed: %v", tc.name, err)
			}
			if v == "" {
				t.Fatalf("%s returned empty bcs", tc.name)
			}
		})
	}
}

func TestDeepBookClientWhitelistedAndPriceInfoAge(t *testing.T) {
	c := newMethodTestClient()
	ctx := context.Background()

	w, err := c.Whitelisted(ctx, "DEEP_SUI")
	if err != nil || !w {
		t.Fatalf("Whitelisted failed: %v, value=%v", err, w)
	}

	age, err := c.GetPriceInfoObjectAge(ctx, "SUI")
	if err != nil {
		t.Fatalf("GetPriceInfoObjectAge failed: %v", err)
	}
	if age != -1 {
		t.Fatalf("expected -1 for coin without price info object, got %d", age)
	}

	custom := NewClient(ClientOptions{
		Client:  deepbookMethodMockClient{},
		Network: "testnet",
		Options: Options{
			Address: "0x1",
			Coins: utils.CoinMap{
				"SUI": {Address: "0x2", Type: "0x2::sui::SUI", Scalar: 1_000_000_000, PriceInfoObjectID: "0xabc"},
			},
			Pools: utils.PoolMap{
				"DEEP_SUI": {Address: "0x11", BaseCoin: "SUI", QuoteCoin: "SUI"},
			},
			BalanceManagers: map[string]types.BalanceManager{
				"m1": {Address: "0x2"},
			},
		},
	})
	age2, err := custom.GetPriceInfoObjectAge(ctx, "SUI")
	if err != nil {
		t.Fatalf("GetPriceInfoObjectAge(custom) failed: %v", err)
	}
	if age2 <= 0 {
		t.Fatalf("expected positive timestamp for price info object age, got %d", age2)
	}
}

func TestDeepBookClientNewReadMethods(t *testing.T) {
	c := newMethodTestClient()
	ctx := context.Background()

	bmIDs, err := c.GetBalanceManagerIDs(ctx, "0x1")
	if err != nil || len(bmIDs) != 2 {
		t.Fatalf("GetBalanceManagerIDs failed: %v ids=%v", err, bmIDs)
	}

	if _, err := c.BalanceManagerReferralPoolID(ctx, "0xaaa"); err != nil {
		t.Fatalf("BalanceManagerReferralPoolID failed: %v", err)
	}
	if _, err := c.GetBalanceManagerReferralID(ctx, "m1", "DEEP_SUI"); err != nil {
		t.Fatalf("GetBalanceManagerReferralID failed: %v", err)
	}

	rb, err := c.GetPoolReferralBalances(ctx, "DEEP_SUI", "0xaaa")
	if err != nil {
		t.Fatalf("GetPoolReferralBalances failed: %v", err)
	}
	if fmt.Sprintf("%.6f", rb["deep"]) == "0.000000" {
		t.Fatalf("expected non-zero deep referral balance")
	}

	if _, err := c.PoolReferralMultiplier(ctx, "DEEP_SUI", "0xaaa"); err != nil {
		t.Fatalf("PoolReferralMultiplier failed: %v", err)
	}
	if v, err := c.StablePool(ctx, "DEEP_SUI"); err != nil || !v {
		t.Fatalf("StablePool failed: %v val=%v", err, v)
	}
	if v, err := c.RegisteredPool(ctx, "DEEP_SUI"); err != nil || !v {
		t.Fatalf("RegisteredPool failed: %v val=%v", err, v)
	}
	if v, err := c.AccountExists(ctx, "DEEP_SUI", "m1"); err != nil || !v {
		t.Fatalf("AccountExists failed: %v val=%v", err, v)
	}
	if _, err := c.Quorum(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("Quorum failed: %v", err)
	}
	if _, err := c.PoolID(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("PoolID failed: %v", err)
	}
	if ok, err := c.CanPlaceLimitOrder(ctx, types.CanPlaceLimitOrderParams{
		PoolKey: "DEEP_SUI", BalanceManagerKey: "m1", Price: 1.0, Quantity: 1.0,
		IsBid: true, PayWithDeep: false, ExpireTimestamp: 1,
	}); err != nil || !ok {
		t.Fatalf("CanPlaceLimitOrder failed: %v val=%v", err, ok)
	}
	if ok, err := c.CanPlaceMarketOrder(ctx, types.CanPlaceMarketOrderParams{
		PoolKey: "DEEP_SUI", BalanceManagerKey: "m1", Quantity: 1.0, IsBid: true, PayWithDeep: false,
	}); err != nil || !ok {
		t.Fatalf("CanPlaceMarketOrder failed: %v val=%v", err, ok)
	}
	if ok, err := c.CheckMarketOrderParams(ctx, "DEEP_SUI", 1.0); err != nil || !ok {
		t.Fatalf("CheckMarketOrderParams failed: %v val=%v", err, ok)
	}
	if ok, err := c.CheckLimitOrderParams(ctx, "DEEP_SUI", 1.0, 1.0, 1); err != nil || !ok {
		t.Fatalf("CheckLimitOrderParams failed: %v val=%v", err, ok)
	}
	if _, err := c.GetMarginPoolID(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolID failed: %v", err)
	}
	if ok, err := c.IsPoolEnabledForMargin(ctx, "DEEP_SUI"); err != nil || !ok {
		t.Fatalf("IsPoolEnabledForMargin failed: %v val=%v", err, ok)
	}
	if ids, err := c.GetMarginManagerIDsForOwner(ctx, "0x1"); err != nil || len(ids) != 2 {
		t.Fatalf("GetMarginManagerIDsForOwner failed: %v ids=%v", err, ids)
	}
	if ids, err := c.GetConditionalOrderIDs(ctx, "mm1"); err != nil || len(ids) != 3 {
		t.Fatalf("GetConditionalOrderIDs failed: %v ids=%v", err, ids)
	}
	if _, err := c.GetLowestTriggerAbovePrice(ctx, "mm1"); err != nil {
		t.Fatalf("GetLowestTriggerAbovePrice failed: %v", err)
	}
	if _, err := c.GetHighestTriggerBelowPrice(ctx, "mm1"); err != nil {
		t.Fatalf("GetHighestTriggerBelowPrice failed: %v", err)
	}
	if raw, err := c.GetLevel2Range(ctx, "DEEP_SUI", 0.9, 1.1, true); err != nil || raw == "" {
		t.Fatalf("GetLevel2Range failed: %v raw=%q", err, raw)
	}
	if raw, err := c.GetLevel2TicksFromMid(ctx, "DEEP_SUI", 10); err != nil || raw == "" {
		t.Fatalf("GetLevel2TicksFromMid failed: %v raw=%q", err, raw)
	}
}
