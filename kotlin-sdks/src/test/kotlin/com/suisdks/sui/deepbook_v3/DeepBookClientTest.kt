package com.suisdks.sui.deepbook_v3

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import com.suisdks.sui.bcs.BcsWriter
import java.util.Base64
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class DeepBookClientTest {
    @Test
    fun testnetFactory() {
        val cfg = DeepBookConfig.testnet("0x1")
        assertEquals("testnet", cfg.network)
        assertTrue(cfg.coins.containsKey("DEEP"))
        assertTrue(cfg.pools.containsKey("DEEP_SUI"))
    }

    @Test
    fun deepbookViewHelpers() {
        val testCoins = TESTNET_COINS.toMutableMap()
        val deep = testCoins["DEEP"]!!
        testCoins["DEEP"] = deep.copy(priceInfoObjectId = "0x99")

        val config = DeepBookConfig(
            address = "0x1",
            balanceManagers = mapOf("m1" to BalanceManager(address = "0x2")),
            marginManagers = mapOf("mm1" to MarginManager(address = "0x3", poolKey = "DEEP_SUI")),
            coins = testCoins,
        )

        val client = DeepBookClient(
            callFn = { method, params ->
                if (method == "sui_getObject") {
                    mapOf(
                        "result" to mapOf(
                            "data" to mapOf(
                                "content" to mapOf(
                                    "fields" to mapOf(
                                        "price_info" to mapOf(
                                            "fields" to mapOf(
                                                "arrival_time" to "1710000000",
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    )
                } else {
                    assertEquals("sui_dryRunTransactionBlock", method)
                    val txB64 = params.first().toString()
                    val moveFns = extractMoveFns(txB64)
                    val moveFn = moveFns.first()
                    val secondMoveFn = moveFns.getOrNull(1)
                    when {
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::deepbook_pool_allowed" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(bool(true)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::total_supply" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::supply_shares" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::total_borrow" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::borrow_shares" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::last_update_timestamp" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_700_000_000)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::supply_cap" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::max_utilization_rate" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(250_000_000)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::protocol_spread" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(250_000_000)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::min_borrow" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::interest_rate" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(250_000_000)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::user_supply_shares" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_pool::user_supply_amount" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_234_567)))
                        moveFn == "margin_registry::get_margin_pool_id" && secondMoveFn == "margin_manager::calculate_debts" ->
                            dryRunResultTwoCommands(listOf(addr(0x56)), listOf(u64(1_000_000), u64(1_500_000_000)))
                        moveFn == "margin_manager::get_margin_account_order_details" && secondMoveFn == "margin_manager::balance_manager" ->
                            dryRunResultTwoCommands(listOf(u64(100)), listOf(u64(100)))
                        moveFns.all { it == "margin_manager::manager_state" } ->
                            dryRunResultManyCommands(moveFns.mapIndexed { idx, _ -> marginStateResult(idx) })
                        else -> when (moveFn) {
                    "balance_manager::balance" -> dryRunResult(listOf(u64(2_000_000)))
                    "pool::whitelisted" -> dryRunResult(listOf(bool(true)))
                    "pool::get_quote_quantity_out" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::get_base_quantity_out" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::get_quote_quantity_out_input_fee" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::get_base_quantity_out_input_fee" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::get_quantity_out" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::get_quantity_out_input_fee" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::get_base_quantity_in" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::get_quote_quantity_in" -> dryRunResult(listOf(u64(1_000_000), u64(2_000_000_000), u64(300_000)))
                    "pool::mid_price" -> dryRunResult(listOf(u64(500_000_000)))
                    "registry::get_balance_manager_ids" -> dryRunResult(listOf(vecAddress(0xAA, 0xBB)))
                    "pool::get_pool_id_by_asset" -> dryRunResult(listOf(addr(0xCC)))
                    "pool::account_open_orders" -> dryRunResult(listOf(vecU128("11", "22")))
                    "pool::get_order" -> dryRunResult(listOf(orderBytes(orderId = "22772505558994441469959731", clientOrderId = 42L)))
                    "pool::get_orders" -> dryRunResult(listOf(vecOrder(orderBytes(orderId = "22772505558994441469959731", clientOrderId = 42L))))
                    "pool::can_place_limit_order" -> dryRunResult(listOf(bool(true)))
                    "pool::can_place_market_order" -> dryRunResult(listOf(bool(true)))
                    "pool::check_market_order_params" -> dryRunResult(listOf(bool(true)))
                    "pool::check_limit_order_params" -> dryRunResult(listOf(bool(true)))
                    "pool::id" -> dryRunResult(listOf(addr(0x11)))
                    "pool::quorum" -> dryRunResult(listOf(u64(1_500_000)))
                    "pool::stable_pool" -> dryRunResult(listOf(bool(true)))
                    "pool::registered_pool" -> dryRunResult(listOf(bool(true)))
                    "pool::get_order_deep_required" -> dryRunResult(listOf(u64(1_200_000), u64(800_000)))
                    "pool::get_level2_range" -> dryRunResult(listOf(vecU64(1_000_000_000_000), vecU64(2_000_000)))
                    "pool::get_level2_ticks_from_mid" ->
                        dryRunResult(listOf(vecU64(1_000_000_000_000), vecU64(3_000_000), vecU64(2_000_000_000_000), vecU64(4_000_000)))
                    "pool::get_pool_deep_price" -> dryRunResult(listOf(poolDeepPrice(assetIsBase = true, deepPerAsset = 1_000_000_000)))
                    "pool::account" -> dryRunResult(listOf(u64(100)))
                    "pool::vault_balances" -> dryRunResult(listOf(u64(100), u64(200), u64(300)))
                    "pool::pool_trade_params" -> dryRunResult(listOf(u64(250_000_000), u64(100_000_000), u64(500_000)))
                    "pool::pool_trade_params_next" -> dryRunResult(listOf(u64(260_000_000), u64(110_000_000), u64(600_000)))
                    "pool::pool_book_params" -> dryRunResult(listOf(u64(1_000_000_000), u64(2_000_000), u64(1_000_000)))
                    "pool::locked_balance" -> dryRunResult(listOf(u64(100), u64(200), u64(300)))
                    "pool::account_exists" -> dryRunResult(listOf(bool(true)))
                    "margin_registry::allowed_maintainers" -> dryRunResult(listOf(vecAddress(0x12, 0x34)))
                    "margin_registry::allowed_pause_caps" -> dryRunResult(listOf(vecAddress(0x56, 0x78)))
                    "margin_registry::pool_enabled" -> dryRunResult(listOf(bool(true)))
                    "margin_registry::get_margin_pool_id" -> dryRunResult(listOf(addr(0x9A)))
                    "margin_registry::get_deepbook_pool_margin_pool_ids" -> dryRunResult(listOf(twoAddr(0xA1, 0xB2)))
                    "margin_registry::get_margin_manager_ids" -> dryRunResult(listOf(vecAddress(0x21, 0x43)))
                    "margin_registry::base_margin_pool_id" -> dryRunResult(listOf(addr(0xA1)))
                    "margin_registry::quote_margin_pool_id" -> dryRunResult(listOf(addr(0xB2)))
                    "margin_registry::min_withdraw_risk_ratio" -> dryRunResult(listOf(u64(250_000_000)))
                    "margin_registry::min_borrow_risk_ratio" -> dryRunResult(listOf(u64(250_000_000)))
                    "margin_registry::liquidation_risk_ratio" -> dryRunResult(listOf(u64(250_000_000)))
                    "margin_registry::target_liquidation_risk_ratio" -> dryRunResult(listOf(u64(250_000_000)))
                    "margin_registry::user_liquidation_reward" -> dryRunResult(listOf(u64(250_000_000)))
                    "margin_registry::pool_liquidation_reward" -> dryRunResult(listOf(u64(250_000_000)))
                    "margin_manager::owner" -> dryRunResult(listOf(addr(0xAB)))
                    "margin_manager::deepbook_pool" -> dryRunResult(listOf(addr(0xBC)))
                    "margin_manager::margin_pool_id" -> dryRunResult(listOf(optionAddr(0xCD)))
                    "margin_manager::borrowed_shares" -> dryRunResult(listOf(u64(777), u64(888)))
                    "margin_manager::borrowed_base_shares" -> dryRunResult(listOf(u64(999)))
                    "margin_manager::borrowed_quote_shares" -> dryRunResult(listOf(u64(1001)))
                    "margin_manager::has_base_debt" -> dryRunResult(listOf(bool(true)))
                    "margin_manager::balance_manager" -> dryRunResult(listOf(addr(0xDE)))
                    "margin_manager::base_balance" -> dryRunResult(listOf(u64(2_000_000)))
                    "margin_manager::quote_balance" -> dryRunResult(listOf(u64(3_000_000_000)))
                    "margin_manager::deep_balance" -> dryRunResult(listOf(u64(4_000_000)))
                    "margin_manager::calculate_assets" -> dryRunResult(listOf(u64(2_000_000), u64(3_000_000_000)))
                    "margin_tpsl::conditional_order_ids" -> dryRunResult(listOf(vecU64(11, 22)))
                    "margin_tpsl::lowest_trigger_above_price" -> dryRunResult(listOf(u64(4444)))
                    "margin_tpsl::highest_trigger_below_price" -> dryRunResult(listOf(u64(5555)))
                    "balance_manager::balance_manager_referral_owner" -> dryRunResult(listOf(addr(0xDD)))
                    "balance_manager::balance_manager_referral_pool_id" -> dryRunResult(listOf(addr(0xEE)))
                    "balance_manager::get_balance_manager_referral_id" -> dryRunResult(listOf(optionAddr(0xFF)))
                    "pool::get_account_order_details" -> dryRunResult(listOf(vecOrder(orderBytes(orderId = "22772505558994441469959731", clientOrderId = 42L))))
                    "pool::get_pool_referral_balances" -> dryRunResult(listOf(u64(100), u64(200), u64(300)))
                    "pool::pool_referral_multiplier" -> dryRunResult(listOf(u64(250_000_000)))
                    else -> error("unexpected move fn: $moveFn")
                        }
                    }
                }
            },
            config = config,
        )

        val balance = client.checkManagerBalance("m1", "DEEP")
        assertEquals("0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP", balance["coinType"])
        assertEquals(2.0, balance["balance"])
        assertTrue(client.whitelisted("DEEP_SUI"))

        val q = client.getQuoteQuantityOut("DEEP_SUI", 1.0)
        assertEquals(1.0, q["baseOut"])
        assertEquals(2.0, q["quoteOut"])
        assertEquals(0.3, q["deepRequired"])
        assertEquals(1.0, client.getBaseQuantityOut("DEEP_SUI", 1.0)["baseOut"])
        assertEquals(1.0, client.getQuoteQuantityOutInputFee("DEEP_SUI", 1.0)["baseOut"])
        assertEquals(1.0, client.getBaseQuantityOutInputFee("DEEP_SUI", 1.0)["baseOut"])
        assertEquals(1.0, client.getQuantityOut("DEEP_SUI", 1.0, 2.0)["baseOut"])
        assertEquals(1.0, client.getQuantityOutInputFee("DEEP_SUI", 1.0, 2.0)["baseOut"])
        assertEquals(1.0, client.getBaseQuantityIn("DEEP_SUI", 2.0, true)["baseIn"])
        assertEquals(1.0, client.getQuoteQuantityIn("DEEP_SUI", 1.0, true)["baseOut"])

        assertEquals(0.5, client.midPrice("DEEP_SUI"))
        assertEquals(
            listOf("0x" + "aa".repeat(32), "0x" + "bb".repeat(32)),
            client.getBalanceManagerIds("0x1"),
        )
        assertEquals(listOf("11", "22"), client.accountOpenOrders("DEEP_SUI", "m1"))
        assertEquals("0x" + "cc".repeat(32), client.getPoolIdByAssets("0x2", "0x2"))
        val order = client.getOrder("DEEP_SUI", "22772505558994441469959731")
        assertEquals("42", order?.get("client_order_id"))
        assertEquals(1, (client.getOrders("DEEP_SUI", listOf("22772505558994441469959731"))).size)
        assertTrue(client.getOrderNormalized("DEEP_SUI", "22772505558994441469959731")?.containsKey("normalized_price") == true)
        assertTrue(client.canPlaceLimitOrder("DEEP_SUI", "m1", 0.2, 1.0, true, true, 123))
        assertTrue(client.canPlaceMarketOrder("DEEP_SUI", "m1", 1.0, true, true))
        assertTrue(client.checkMarketOrderParams("DEEP_SUI", 1.0))
        assertTrue(client.checkLimitOrderParams("DEEP_SUI", 0.2, 1.0, 123))
        assertEquals("0x" + "11".repeat(32), client.poolId("DEEP_SUI"))
        assertEquals(1.5, client.quorum("DEEP_SUI"))
        assertTrue(client.stablePool("DEEP_SUI"))
        assertTrue(client.registeredPool("DEEP_SUI"))
        val deepReq = client.getOrderDeepRequired("DEEP_SUI", 1.0, 0.2)
        assertEquals(1.2, deepReq["deepRequiredTaker"]!!, 1e-9)
        assertEquals(0.8, deepReq["deepRequiredMaker"]!!, 1e-9)
        val l2 = client.getLevel2Range("DEEP_SUI", 0.1, 0.2, true)
        assertEquals(1, l2["prices"]?.size)
        assertEquals(1, l2["quantities"]?.size)
        val ticks = client.getLevel2TicksFromMid("DEEP_SUI", 5)
        assertEquals(1, ticks["bid_prices"]?.size)
        assertEquals(1, ticks["ask_prices"]?.size)
        val deepPrice = client.getPoolDeepPrice("DEEP_SUI")
        assertEquals(true, deepPrice["asset_is_base"])
        assertEquals(true, deepPrice.containsKey("deep_per_base"))
        assertTrue(client.account("DEEP_SUI", "m1").isNotBlank())
        val vault = client.vaultBalances("DEEP_SUI")
        assertEquals(0.0001, vault["base"]!!, 1e-12)
        assertEquals(0.0000002, vault["quote"]!!, 1e-12)
        assertEquals(0.0003, vault["deep"]!!, 1e-12)
        val trade = client.poolTradeParams("DEEP_SUI")
        assertEquals(0.25, trade["takerFee"]!!, 1e-12)
        assertEquals(0.1, trade["makerFee"]!!, 1e-12)
        assertEquals(0.5, trade["stakeRequired"]!!, 1e-12)
        val tradeNext = client.poolTradeParamsNext("DEEP_SUI")
        assertEquals(0.26, tradeNext["takerFee"]!!, 1e-12)
        assertEquals(0.11, tradeNext["makerFee"]!!, 1e-12)
        assertEquals(0.6, tradeNext["stakeRequired"]!!, 1e-12)
        val book = client.poolBookParams("DEEP_SUI")
        assertEquals(1.0, book["tickSize"]!!, 1e-12)
        assertEquals(2.0, book["lotSize"]!!, 1e-12)
        assertEquals(1.0, book["minSize"]!!, 1e-12)
        val locked = client.lockedBalance("DEEP_SUI", "m1")
        assertEquals(0.0001, locked["base"]!!, 1e-12)
        assertEquals(0.0000002, locked["quote"]!!, 1e-12)
        assertEquals(0.0003, locked["deep"]!!, 1e-12)
        assertTrue(client.accountExists("DEEP_SUI", "m1"))
        assertEquals(
            listOf("0x" + "12".repeat(32), "0x" + "34".repeat(32)),
            client.getAllowedMaintainers(),
        )
        assertEquals(
            listOf("0x" + "56".repeat(32), "0x" + "78".repeat(32)),
            client.getAllowedPauseCaps(),
        )
        assertTrue(client.isPoolEnabledForMargin("DEEP_SUI"))
        assertEquals("0x" + "9a".repeat(32), client.getMarginPoolId("DEEP"))
        val marginIds = client.getDeepbookPoolMarginPoolIds("DEEP_SUI")
        assertEquals("0x" + "a1".repeat(32), marginIds["baseMarginPoolId"])
        assertEquals("0x" + "b2".repeat(32), marginIds["quoteMarginPoolId"])
        assertEquals(
            listOf("0x" + "21".repeat(32), "0x" + "43".repeat(32)),
            client.getMarginManagerIdsForOwner("0x1"),
        )
        assertEquals("0x" + "a1".repeat(32), client.getBaseMarginPoolId("DEEP_SUI"))
        assertEquals("0x" + "b2".repeat(32), client.getQuoteMarginPoolId("DEEP_SUI"))
        assertEquals(0.25, client.getMinWithdrawRiskRatio("DEEP_SUI"), 1e-12)
        assertEquals(0.25, client.getMinBorrowRiskRatio("DEEP_SUI"), 1e-12)
        assertEquals(0.25, client.getLiquidationRiskRatio("DEEP_SUI"), 1e-12)
        assertEquals(0.25, client.getTargetLiquidationRiskRatio("DEEP_SUI"), 1e-12)
        assertEquals(0.25, client.getUserLiquidationReward("DEEP_SUI"), 1e-12)
        assertEquals(0.25, client.getPoolLiquidationReward("DEEP_SUI"), 1e-12)
        assertEquals("0x" + "ab".repeat(32), client.getMarginManagerOwner("mm1"))
        assertEquals("0x" + "bc".repeat(32), client.getMarginManagerDeepbookPool("mm1"))
        assertEquals("0x" + "cd".repeat(32), client.getMarginManagerMarginPoolId("mm1"))
        val borrowed = client.getMarginManagerBorrowedShares("mm1")
        assertEquals("777", borrowed["baseShares"])
        assertEquals("888", borrowed["quoteShares"])
        assertEquals("999", client.getMarginManagerBorrowedBaseShares("mm1"))
        assertEquals("1001", client.getMarginManagerBorrowedQuoteShares("mm1"))
        assertTrue(client.getMarginManagerHasBaseDebt("mm1"))
        assertEquals("0x" + "de".repeat(32), client.getMarginManagerBalanceManagerId("mm1"))
        assertEquals(2.0, client.getMarginManagerBaseBalance("mm1"), 1e-12)
        assertEquals(3.0, client.getMarginManagerQuoteBalance("mm1"), 1e-12)
        assertEquals(4.0, client.getMarginManagerDeepBalance("mm1"), 1e-12)
        assertEquals(listOf("11", "22"), client.getConditionalOrderIds("mm1"))
        assertEquals(4444L, client.getLowestTriggerAbovePrice("mm1"))
        assertEquals(5555L, client.getHighestTriggerBelowPrice("mm1"))
        assertTrue(client.isDeepbookPoolAllowed("DEEP", "0x11"))
        assertEquals(1.234567, client.getMarginPoolTotalSupply("DEEP"), 1e-12)
        assertEquals(1.234567, client.getMarginPoolSupplyShares("DEEP"), 1e-12)
        assertEquals(1.234567, client.getMarginPoolTotalBorrow("DEEP"), 1e-12)
        assertEquals(1.234567, client.getMarginPoolBorrowShares("DEEP"), 1e-12)
        assertEquals(1_700_000_000L, client.getMarginPoolLastUpdateTimestamp("DEEP"))
        assertEquals(1.234567, client.getMarginPoolSupplyCap("DEEP"), 1e-12)
        assertEquals(0.25, client.getMarginPoolMaxUtilizationRate("DEEP"), 1e-12)
        assertEquals(0.25, client.getMarginPoolProtocolSpread("DEEP"), 1e-12)
        assertEquals(1.234567, client.getMarginPoolMinBorrow("DEEP"), 1e-12)
        assertEquals(0.25, client.getMarginPoolInterestRate("DEEP"), 1e-12)
        assertEquals(1.234567, client.getUserSupplyShares("DEEP", "0x1"), 1e-12)
        assertEquals(1.234567, client.getUserSupplyAmount("DEEP", "0x1"), 1e-12)
        assertEquals("1.234567", client.getMarginPoolTotalSupply("DEEP", 6))
        assertEquals("1.234567", client.getUserSupplyAmount("DEEP", "0x1", 6))
        assertEquals("0x" + "ff".repeat(32), client.getBalanceManagerReferralId("m1", "DEEP_SUI"))
        assertEquals(1, client.getAccountOrderDetails("DEEP_SUI", "m1").size)
        assertTrue(client.getMarginAccountOrderDetails("mm1").isNotBlank())
        assertEquals(1_710_000_000L, client.getPriceInfoObjectAge("DEEP"))
        assertEquals(10L, client.getPriceInfoObjectStaleness("DEEP", nowEpochSeconds = 1710000010))
        assertEquals("0x99", client.getPriceInfoObject("DEEP"))
        assertTrue(client.getPriceInfoObjectRaw("DEEP").containsKey("result"))
        assertEquals(1, client.getPriceInfoObjects().size)
        assertEquals(mapOf("DEEP" to "0x99"), client.getPriceInfoObjects(listOf("DEEP")))
        val state = client.getMarginManagerState("mm1")
        assertEquals(0.25, state["riskRatio"] as Double, 1e-12)
        assertEquals(2.0, state["baseAsset"] as Double, 1e-12)
        assertEquals(3.0, state["quoteAsset"] as Double, 1e-12)
        assertEquals(1.0, state["baseDebt"] as Double, 1e-12)
        assertEquals(1.5, state["quoteDebt"] as Double, 1e-12)
        assertEquals(8, state["basePythDecimals"])
        assertEquals(9, state["quotePythDecimals"])
        assertEquals(1111L, state["currentPrice"])
        assertEquals(2222L, state["lowestTriggerAbovePrice"])
        assertEquals(3333L, state["highestTriggerBelowPrice"])
        val states = client.getMarginManagerStates(listOf("mm1"))
        assertEquals(1, states.size)
        val statesById = client.getMarginManagerStates(mapOf("0x3" to "DEEP_SUI"))
        assertEquals(1, statesById.size)
        val assets = client.getMarginManagerAssets("mm1")
        assertEquals(2.0, assets["base"]!!, 1e-12)
        assertEquals(3.0, assets["quote"]!!, 1e-12)
        val assetsFmt = client.getMarginManagerAssets("mm1", 6)
        assertEquals("2", assetsFmt["baseAsset"])
        assertEquals("3", assetsFmt["quoteAsset"])
        val debts = client.getMarginManagerDebts("mm1")
        assertEquals(1.0, debts["base"]!!, 1e-12)
        assertEquals(1.5, debts["quote"]!!, 1e-12)
        val debtsFmt = client.getMarginManagerDebts("mm1", 6)
        assertEquals("1", debtsFmt["baseDebt"])
        assertEquals("1500", debtsFmt["quoteDebt"])
        assertEquals("2", client.getMarginManagerBaseBalance("mm1", 6))
        assertEquals("3", client.getMarginManagerQuoteBalance("mm1", 6))
        assertEquals("4", client.getMarginManagerDeepBalance("mm1", 6))
        assertEquals("0x" + "dd".repeat(32), client.balanceManagerReferralOwner("0xref"))
        assertEquals("0x" + "ee".repeat(32), client.balanceManagerReferralPoolId("0xref"))

        val ref = client.getPoolReferralBalances("DEEP_SUI", "0xref")
        assertEquals(0.0001, ref["base"])
        assertEquals(0.0000002, ref["quote"])
        assertEquals(0.0003, ref["deep"])
        assertEquals(0.25, client.poolReferralMultiplier("DEEP_SUI", "0xref"))
    }

    @Test
    fun decodeOrderId() {
        val encoded = (java.math.BigInteger("12345").shiftLeft(64)).add(java.math.BigInteger("77"))
        val client = DeepBookClient({ _, _ -> emptyMap() }, DeepBookConfig(address = "0x1"))
        val decoded = client.decodeOrderId(encoded.toString())
        assertEquals(true, decoded["isBid"])
        assertEquals("12345", decoded["price"])
        assertEquals("77", decoded["orderId"])
    }

    private fun extractMoveFns(txBytesB64: String): List<String> {
        val json = String(Base64.getDecoder().decode(txBytesB64), Charsets.UTF_8)
        val type = object : TypeToken<Map<String, Any?>>() {}.type
        val parsed = Gson().fromJson<Map<String, Any?>>(json, type)
        @Suppress("UNCHECKED_CAST")
        val commands = parsed["commands"] as List<Map<String, Any?>>
        return commands.mapNotNull { cmd ->
            @Suppress("UNCHECKED_CAST")
            val call = cmd["MoveCall"] as? Map<String, Any?>
            if (call != null) "${call["module"]}::${call["function"]}" else null
        }
    }

    private fun dryRunResult(returnValues: List<String>): Map<String, Any?> {
        return mapOf(
            "result" to mapOf(
                "commandResults" to listOf(
                    mapOf(
                        "returnValues" to returnValues.map { mapOf("bcs" to it) },
                    ),
                ),
            ),
        )
    }

    private fun dryRunResultTwoCommands(
        cmd0ReturnValues: List<String>,
        cmd1ReturnValues: List<String>,
    ): Map<String, Any?> {
        return mapOf(
            "result" to mapOf(
                "commandResults" to listOf(
                    mapOf(
                        "returnValues" to cmd0ReturnValues.map { mapOf("bcs" to it) },
                    ),
                    mapOf(
                        "returnValues" to cmd1ReturnValues.map { mapOf("bcs" to it) },
                    ),
                ),
            ),
        )
    }

    private fun dryRunResultManyCommands(commandReturnValues: List<List<String>>): Map<String, Any?> {
        return mapOf(
            "result" to mapOf(
                "commandResults" to commandReturnValues.map { returns ->
                    mapOf(
                        "returnValues" to returns.map { mapOf("bcs" to it) },
                    )
                },
            ),
        )
    }

    private fun u64(v: Long): String {
        val w = BcsWriter()
        w.writeU64(v)
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun bool(v: Boolean): String = Base64.getEncoder().encodeToString(byteArrayOf(if (v) 1 else 0))

    private fun addr(fill: Int): String = Base64.getEncoder().encodeToString(ByteArray(32) { fill.toByte() })

    private fun vecAddress(vararg fill: Int): String {
        val w = BcsWriter()
        w.writeUleb128(fill.size.toLong())
        fill.forEach { byte ->
            w.writeBytes(ByteArray(32) { byte.toByte() })
        }
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun u128(v: String): ByteArray {
        val n = java.math.BigInteger(v)
        val out = ByteArray(16)
        var cur = n
        repeat(16) { i ->
            out[i] = cur.and(java.math.BigInteger("ff", 16)).toByte()
            cur = cur.shiftRight(8)
        }
        return out
    }

    private fun vecU128(vararg values: String): String {
        val w = BcsWriter()
        w.writeUleb128(values.size.toLong())
        values.forEach { w.writeBytes(u128(it)) }
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun vecU64(vararg values: Long): String {
        val w = BcsWriter()
        w.writeUleb128(values.size.toLong())
        values.forEach { w.writeU64(it) }
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun orderBytes(orderId: String, clientOrderId: Long): String {
        val w = BcsWriter()
        w.writeBytes(ByteArray(32) { 0xAA.toByte() }) // manager_id
        w.writeBytes(u128(orderId))
        w.writeU64(clientOrderId)
        w.writeU64(2_000_000)
        w.writeU64(500_000)
        w.writeBool(true)
        w.writeBool(true)
        w.writeU64(1_000_000_000)
        w.writeU64(9)
        w.writeU8(1)
        w.writeU64(123456)
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun poolDeepPrice(assetIsBase: Boolean, deepPerAsset: Long): String {
        val w = BcsWriter()
        w.writeBool(assetIsBase)
        w.writeU64(deepPerAsset)
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun vecOrder(orderBase64: String): String {
        val raw = Base64.getDecoder().decode(orderBase64)
        val w = BcsWriter()
        w.writeUleb128(1)
        w.writeBytes(raw)
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun twoAddr(a: Int, b: Int): String {
        val w = BcsWriter()
        w.writeBytes(ByteArray(32) { a.toByte() })
        w.writeBytes(ByteArray(32) { b.toByte() })
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun optionAddr(fill: Int): String {
        val w = BcsWriter()
        w.writeU8(1)
        w.writeBytes(ByteArray(32) { fill.toByte() })
        return Base64.getEncoder().encodeToString(w.toByteArray())
    }

    private fun u8(v: Int): String = Base64.getEncoder().encodeToString(byteArrayOf((v and 0xFF).toByte()))

    private fun marginStateResult(offset: Int): List<String> {
        val managerByte = 0xA0 + offset
        val poolByte = 0xB0 + offset
        return listOf(
            addr(managerByte),
            addr(poolByte),
            u64(250_000_000),
            u64(2_000_000),
            u64(3_000_000_000),
            u64(1_000_000),
            u64(1_500_000_000),
            u64(123456),
            u8(8),
            u64(654321),
            u8(9),
            u64(1111),
            u64(2222),
            u64(3333),
        )
    }
}
