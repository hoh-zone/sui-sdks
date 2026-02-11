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

func encodeOrderForTest() string {
	w := bcs.NewWriter(nil)
	_ = w.WriteBytes(make([]byte, 32))
	var orderID [16]byte
	orderID[0] = 1
	_ = w.Write128(orderID)
	_ = w.Write64(1)
	_ = w.Write64(100)
	_ = w.Write64(10)
	_ = w.Write8(1)
	_ = w.Write8(1)
	_ = w.Write64(2_000_000)
	_ = w.Write64(1)
	_ = w.Write8(1)
	_ = w.Write64(1)
	return base64.StdEncoding.EncodeToString(w.ToBytes())
}

func encodeVecOrderForTest(n int) string {
	orderBytes, _ := base64.StdEncoding.DecodeString(encodeOrderForTest())
	w := bcs.NewWriter(nil)
	_ = w.WriteULEB(uint64(n))
	for i := 0; i < n; i++ {
		_ = w.WriteBytes(orderBytes)
	}
	return base64.StdEncoding.EncodeToString(w.ToBytes())
}

func encodeVecU128ForTest(vals []uint64) string {
	w := bcs.NewWriter(nil)
	_ = w.WriteULEB(uint64(len(vals)))
	for _, v := range vals {
		var le [16]byte
		le[0] = byte(v)
		le[1] = byte(v >> 8)
		le[2] = byte(v >> 16)
		le[3] = byte(v >> 24)
		le[4] = byte(v >> 32)
		le[5] = byte(v >> 40)
		le[6] = byte(v >> 48)
		le[7] = byte(v >> 56)
		_ = w.WriteBytes(le[:])
	}
	return base64.StdEncoding.EncodeToString(w.ToBytes())
}

func writeU128LEFromU64(w *bcs.Writer, v uint64) {
	var le [16]byte
	le[0] = byte(v)
	le[1] = byte(v >> 8)
	le[2] = byte(v >> 16)
	le[3] = byte(v >> 24)
	le[4] = byte(v >> 32)
	le[5] = byte(v >> 40)
	le[6] = byte(v >> 48)
	le[7] = byte(v >> 56)
	_ = w.WriteBytes(le[:])
}

func encodeAccountForTest() string {
	w := bcs.NewWriter(nil)
	_ = w.Write64(1)           // epoch
	_ = w.WriteULEB(2)         // open_orders length
	writeU128LEFromU64(w, 1)   // open order 1
	writeU128LEFromU64(w, 2)   // open order 2
	writeU128LEFromU64(w, 100) // taker_volume
	writeU128LEFromU64(w, 200) // maker_volume
	_ = w.Write64(3000000)     // active_stake
	_ = w.Write64(4000000)     // inactive_stake
	_ = w.Write8(1)            // created_proposal
	_ = w.Write8(1)            // voted_proposal Some
	addrBytes, _ := base64.StdEncoding.DecodeString(encodeAddressHex("0x123"))
	_ = w.WriteBytes(addrBytes) // voted_proposal address
	_ = w.Write64(100)          // unclaimed base
	_ = w.Write64(200)          // unclaimed quote
	_ = w.Write64(300)          // unclaimed deep
	_ = w.Write64(110)          // settled base
	_ = w.Write64(210)          // settled quote
	_ = w.Write64(310)          // settled deep
	_ = w.Write64(120)          // owed base
	_ = w.Write64(220)          // owed quote
	_ = w.Write64(320)          // owed deep
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
	if fn == "pool_trade_params" || fn == "pool_trade_params_next" || fn == "pool_book_params" || fn == "locked_balance" || fn == "get_quote_quantity_out_input_fee" || fn == "get_base_quantity_out_input_fee" || fn == "get_quantity_out_input_fee" || fn == "get_base_quantity_in" || fn == "get_quote_quantity_in" {
		firstRet = []any{
			map[string]any{"bcs": encodeU64(100)},
			map[string]any{"bcs": encodeU64(200)},
			map[string]any{"bcs": encodeU64(300)},
		}
	}
	if fn == "get_order_deep_required" {
		firstRet = []any{
			map[string]any{"bcs": encodeU64(100)},
			map[string]any{"bcs": encodeU64(200)},
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
	if fn == "owner" || fn == "deepbook_pool" || fn == "balance_manager" || fn == "base_margin_pool_id" || fn == "quote_margin_pool_id" {
		firstRet = []any{map[string]any{"bcs": encodeAddressHex("0x234")}}
	}
	if fn == "get_balance_manager_referral_id" {
		firstRet = []any{map[string]any{"bcs": encodeOptionAddress("0xabc")}}
	}
	if fn == "margin_pool_id" {
		firstRet = []any{map[string]any{"bcs": encodeOptionAddress("0x345")}}
	}
	if fn == "get_balance_manager_ids" || fn == "get_margin_manager_ids" {
		firstRet = []any{map[string]any{"bcs": encodeVecAddress([]string{"0x111", "0x222"})}}
	}
	if fn == "allowed_maintainers" || fn == "allowed_pause_caps" {
		firstRet = []any{map[string]any{"bcs": encodeVecAddress([]string{"0xaaa", "0xbbb"})}}
	}
	if fn == "conditional_order_ids" {
		firstRet = []any{map[string]any{"bcs": encodeVecU64([]uint64{1, 2, 3})}}
	}
	if fn == "borrowed_shares" || fn == "calculate_assets" || fn == "calculate_debts" {
		firstRet = []any{
			map[string]any{"bcs": encodeU64(100)},
			map[string]any{"bcs": encodeU64(200)},
		}
	}
	if fn == "borrowed_base_shares" || fn == "borrowed_quote_shares" || fn == "base_balance" || fn == "quote_balance" || fn == "deep_balance" || fn == "last_update_timestamp" || fn == "supply_cap" || fn == "max_utilization_rate" || fn == "protocol_spread" || fn == "min_borrow" || fn == "interest_rate" || fn == "user_supply_shares" || fn == "user_supply_amount" || fn == "min_withdraw_risk_ratio" || fn == "min_borrow_risk_ratio" || fn == "liquidation_risk_ratio" || fn == "target_liquidation_risk_ratio" || fn == "user_liquidation_reward" || fn == "pool_liquidation_reward" {
		firstRet = []any{map[string]any{"bcs": encodeU64(2_000_000_000)}}
	}
	if fn == "manager_state" {
		firstRet = []any{
			map[string]any{"bcs": encodeAddressHex("0xaaa")},
			map[string]any{"bcs": encodeAddressHex("0xbbb")},
			map[string]any{"bcs": encodeU64(1_500_000_000)},
			map[string]any{"bcs": encodeU64(100)},
			map[string]any{"bcs": encodeU64(200)},
			map[string]any{"bcs": encodeU64(10)},
			map[string]any{"bcs": encodeU64(20)},
			map[string]any{"bcs": encodeU64(12345)},
			map[string]any{"bcs": base64.StdEncoding.EncodeToString([]byte{8})},
			map[string]any{"bcs": encodeU64(67890)},
			map[string]any{"bcs": base64.StdEncoding.EncodeToString([]byte{8})},
			map[string]any{"bcs": encodeU64(1000)},
			map[string]any{"bcs": encodeU64(900)},
			map[string]any{"bcs": encodeU64(800)},
		}
	}
	if fn == "has_base_debt" || fn == "deepbook_pool_allowed" {
		firstRet = []any{map[string]any{"bcs": base64.StdEncoding.EncodeToString([]byte{1})}}
	}
	if fn == "get_order" {
		firstRet = []any{map[string]any{"bcs": encodeOrderForTest()}}
	}
	if fn == "account" {
		firstRet = []any{map[string]any{"bcs": encodeAccountForTest()}}
	}
	if fn == "get_orders" || fn == "get_account_order_details" {
		firstRet = []any{map[string]any{"bcs": encodeVecOrderForTest(2)}}
	}
	if fn == "account_open_orders" {
		firstRet = []any{map[string]any{"bcs": encodeVecU128ForTest([]uint64{1, 2, 3})}}
	}
	if fn == "get_margin_account_order_details" {
		firstRet = []any{map[string]any{"bcs": encodeU64(111)}}
	}
	if fn == "vault_balances" {
		firstRet = []any{
			map[string]any{"bcs": encodeU64(100)},
			map[string]any{"bcs": encodeU64(200)},
			map[string]any{"bcs": encodeU64(300)},
		}
	}
	if fn == "get_pool_id_by_asset" {
		firstRet = []any{map[string]any{"bcs": encodeAddressHex("0x456")}}
	}
	if fn == "get_level2_range" {
		firstRet = []any{
			map[string]any{"bcs": encodeVecU64([]uint64{100, 110})},
			map[string]any{"bcs": encodeVecU64([]uint64{200, 210})},
		}
	}
	if fn == "get_level2_ticks_from_mid" {
		firstRet = []any{
			map[string]any{"bcs": encodeVecU64([]uint64{100})},
			map[string]any{"bcs": encodeVecU64([]uint64{200})},
			map[string]any{"bcs": encodeVecU64([]uint64{120})},
			map[string]any{"bcs": encodeVecU64([]uint64{220})},
		}
	}
	if fn == "get_order_deep_price" {
		firstRet = []any{map[string]any{"bcs": base64.StdEncoding.EncodeToString([]byte{1, 64, 66, 15, 0, 0, 0, 0, 0})}}
	}

	if p, ok := out.(*map[string]any); ok {
		secondRet := []any{map[string]any{"bcs": encodeU64(777)}}
		if fn == "get_margin_account_order_details" || fn == "balance_manager" {
			secondRet = []any{map[string]any{"bcs": encodeVecOrderForTest(2)}}
		}
		*p = map[string]any{
			"commandResults": []any{
				map[string]any{"returnValues": firstRet},
				map[string]any{"returnValues": secondRet},
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

	if v, err := c.GetQuoteQuantityOutInputFee(ctx, "DEEP_SUI", 1); err != nil || v["deepRequired"] == nil {
		t.Fatalf("GetQuoteQuantityOutInputFee failed: %v", err)
	}
	if v, err := c.GetBaseQuantityOutInputFee(ctx, "DEEP_SUI", 1); err != nil || v["deepRequired"] == nil {
		t.Fatalf("GetBaseQuantityOutInputFee failed: %v", err)
	}
	if v, err := c.GetQuantityOutInputFee(ctx, "DEEP_SUI", 1, 0); err != nil || v["deepRequired"] == nil {
		t.Fatalf("GetQuantityOutInputFee failed: %v", err)
	}
	if v, err := c.GetBaseQuantityIn(ctx, "DEEP_SUI", 1, true); err != nil || v["deepRequired"] == nil {
		t.Fatalf("GetBaseQuantityIn failed: %v", err)
	}
	if v, err := c.GetQuoteQuantityIn(ctx, "DEEP_SUI", 1, false); err != nil || v["deepRequired"] == nil {
		t.Fatalf("GetQuoteQuantityIn failed: %v", err)
	}
	if v, err := c.GetOrderDeepRequired(ctx, "DEEP_SUI", 1, 1); err != nil || v["deepRequiredMaker"] == nil {
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
		{"GetOrderRaw", func() (string, error) { return c.GetOrderRaw(ctx, "DEEP_SUI", "1") }},
		{"BalanceManagerReferralOwner", func() (string, error) { return c.BalanceManagerReferralOwner(ctx, "0xaaa") }},
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

func TestDeepBookClientStructuredOrderAndVaultMethods(t *testing.T) {
	c := newMethodTestClient()
	ctx := context.Background()

	if order, err := c.GetOrder(ctx, "DEEP_SUI", "1"); err != nil || order["order_id"] == nil {
		t.Fatalf("GetOrder failed: %v order=%v", err, order)
	}
	if orders, err := c.GetOrders(ctx, "DEEP_SUI", []string{"1", "2"}); err != nil || len(orders) != 2 {
		t.Fatalf("GetOrders failed: %v orders=%v", err, orders)
	}
	if account, err := c.Account(ctx, "DEEP_SUI", "m1"); err != nil || account["open_orders"] == nil {
		t.Fatalf("Account failed: %v account=%v", err, account)
	}
	if openOrders, err := c.AccountOpenOrders(ctx, "DEEP_SUI", "m1"); err != nil || len(openOrders) != 3 {
		t.Fatalf("AccountOpenOrders failed: %v openOrders=%v", err, openOrders)
	}
	if details, err := c.GetAccountOrderDetails(ctx, "DEEP_SUI", "m1"); err != nil || len(details) != 2 {
		t.Fatalf("GetAccountOrderDetails failed: %v details=%v", err, details)
	}
	if v, err := c.VaultBalances(ctx, "DEEP_SUI"); err != nil || v["deep"] == nil {
		t.Fatalf("VaultBalances failed: %v v=%v", err, v)
	}
	if poolID, err := c.GetPoolIDByAssets(ctx, "0x2::sui::SUI", "0x3::coin::C"); err != nil || poolID == "" {
		t.Fatalf("GetPoolIDByAssets failed: %v id=%q", err, poolID)
	}
	if details, err := c.GetMarginAccountOrderDetails(ctx, "mm1"); err != nil || len(details) != 2 {
		t.Fatalf("GetMarginAccountOrderDetails failed: %v details=%v", err, details)
	}
	if v, err := c.GetPoolDeepPrice(ctx, "DEEP_SUI"); err != nil || v["asset_is_base"] == nil {
		t.Fatalf("GetPoolDeepPrice failed: %v v=%v", err, v)
	}
}

func TestDeepBookClientStructuredPoolAndBalanceMethods(t *testing.T) {
	c := newMethodTestClient()
	ctx := context.Background()

	if v, err := c.PoolTradeParams(ctx, "DEEP_SUI"); err != nil || v["stakeRequired"] == nil {
		t.Fatalf("PoolTradeParams failed: %v v=%v", err, v)
	}
	if v, err := c.PoolBookParams(ctx, "DEEP_SUI"); err != nil || v["tickSize"] == nil {
		t.Fatalf("PoolBookParams failed: %v v=%v", err, v)
	}
	if v, err := c.LockedBalance(ctx, "DEEP_SUI", "m1"); err != nil || v["deep"] == nil {
		t.Fatalf("LockedBalance failed: %v v=%v", err, v)
	}
	if v, err := c.PoolTradeParamsNext(ctx, "DEEP_SUI"); err != nil || v["makerFee"] == nil {
		t.Fatalf("PoolTradeParamsNext failed: %v v=%v", err, v)
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
	if v, err := c.GetLevel2Range(ctx, "DEEP_SUI", 0.9, 1.1, true); err != nil || v["prices"] == nil {
		t.Fatalf("GetLevel2Range failed: %v v=%v", err, v)
	}
	if v, err := c.GetLevel2TicksFromMid(ctx, "DEEP_SUI", 10); err != nil || v["bid_prices"] == nil {
		t.Fatalf("GetLevel2TicksFromMid failed: %v v=%v", err, v)
	}

	if _, err := c.GetOrderNormalized(ctx, "DEEP_SUI", "1"); err != nil {
		t.Fatalf("GetOrderNormalized failed: %v", err)
	}
	customPriceClient := NewClient(ClientOptions{
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
	if _, err := customPriceClient.GetPriceInfoObject(ctx, "SUI"); err != nil {
		t.Fatalf("GetPriceInfoObject failed: %v", err)
	}
	if _, err := customPriceClient.GetPriceInfoObjects(ctx, []string{"SUI"}); err != nil {
		t.Fatalf("GetPriceInfoObjects failed: %v", err)
	}
	if _, err := c.IsDeepbookPoolAllowed(ctx, "SUI", "0x123"); err != nil {
		t.Fatalf("IsDeepbookPoolAllowed failed: %v", err)
	}
	if _, err := c.GetMarginPoolTotalSupply(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolTotalSupply failed: %v", err)
	}
	if _, err := c.GetMarginPoolSupplyShares(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolSupplyShares failed: %v", err)
	}
	if _, err := c.GetMarginPoolTotalBorrow(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolTotalBorrow failed: %v", err)
	}
	if _, err := c.GetMarginPoolBorrowShares(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolBorrowShares failed: %v", err)
	}
	if _, err := c.GetMarginPoolLastUpdateTimestamp(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolLastUpdateTimestamp failed: %v", err)
	}
	if _, err := c.GetMarginPoolSupplyCap(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolSupplyCap failed: %v", err)
	}
	if _, err := c.GetMarginPoolMaxUtilizationRate(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolMaxUtilizationRate failed: %v", err)
	}
	if _, err := c.GetMarginPoolProtocolSpread(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolProtocolSpread failed: %v", err)
	}
	if _, err := c.GetMarginPoolMinBorrow(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolMinBorrow failed: %v", err)
	}
	if _, err := c.GetMarginPoolInterestRate(ctx, "SUI"); err != nil {
		t.Fatalf("GetMarginPoolInterestRate failed: %v", err)
	}
	if _, err := c.GetUserSupplyShares(ctx, "SUI", "0x123"); err != nil {
		t.Fatalf("GetUserSupplyShares failed: %v", err)
	}
	if _, err := c.GetUserSupplyAmount(ctx, "SUI", "0x123"); err != nil {
		t.Fatalf("GetUserSupplyAmount failed: %v", err)
	}
	if _, err := c.GetBaseMarginPoolID(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetBaseMarginPoolID failed: %v", err)
	}
	if _, err := c.GetQuoteMarginPoolID(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetQuoteMarginPoolID failed: %v", err)
	}
	if _, err := c.GetMinWithdrawRiskRatio(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetMinWithdrawRiskRatio failed: %v", err)
	}
	if _, err := c.GetMinBorrowRiskRatio(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetMinBorrowRiskRatio failed: %v", err)
	}
	if _, err := c.GetLiquidationRiskRatio(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetLiquidationRiskRatio failed: %v", err)
	}
	if _, err := c.GetTargetLiquidationRiskRatio(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetTargetLiquidationRiskRatio failed: %v", err)
	}
	if _, err := c.GetUserLiquidationReward(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetUserLiquidationReward failed: %v", err)
	}
	if _, err := c.GetPoolLiquidationReward(ctx, "DEEP_SUI"); err != nil {
		t.Fatalf("GetPoolLiquidationReward failed: %v", err)
	}
	if vals, err := c.GetAllowedMaintainers(ctx); err != nil || len(vals) == 0 {
		t.Fatalf("GetAllowedMaintainers failed: %v vals=%v", err, vals)
	}
	if vals, err := c.GetAllowedPauseCaps(ctx); err != nil || len(vals) == 0 {
		t.Fatalf("GetAllowedPauseCaps failed: %v vals=%v", err, vals)
	}
	if _, err := c.GetMarginManagerOwner(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerOwner failed: %v", err)
	}
	if _, err := c.GetMarginManagerDeepbookPool(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerDeepbookPool failed: %v", err)
	}
	if _, err := c.GetMarginManagerMarginPoolID(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerMarginPoolID failed: %v", err)
	}
	if _, err := c.GetMarginManagerBorrowedShares(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerBorrowedShares failed: %v", err)
	}
	if _, err := c.GetMarginManagerBorrowedBaseShares(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerBorrowedBaseShares failed: %v", err)
	}
	if _, err := c.GetMarginManagerBorrowedQuoteShares(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerBorrowedQuoteShares failed: %v", err)
	}
	if _, err := c.GetMarginManagerHasBaseDebt(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerHasBaseDebt failed: %v", err)
	}
	if _, err := c.GetMarginManagerBalanceManagerID(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerBalanceManagerID failed: %v", err)
	}
	if _, err := c.GetMarginManagerAssets(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerAssets failed: %v", err)
	}
	if _, err := c.GetMarginManagerDebts(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerDebts failed: %v", err)
	}
	if _, err := c.GetMarginManagerBaseBalance(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerBaseBalance failed: %v", err)
	}
	if _, err := c.GetMarginManagerQuoteBalance(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerQuoteBalance failed: %v", err)
	}
	if _, err := c.GetMarginManagerDeepBalance(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerDeepBalance failed: %v", err)
	}
	if _, err := c.GetMarginManagerState(ctx, "mm1"); err != nil {
		t.Fatalf("GetMarginManagerState failed: %v", err)
	}
	if _, err := c.GetMarginManagerStates(ctx, []string{"mm1"}); err != nil {
		t.Fatalf("GetMarginManagerStates failed: %v", err)
	}
}
