package com.suisdks.sui.deepbook_v3

import com.suisdks.sui.bcs.BcsReader
import com.suisdks.sui.bcs.BcsWriter
import com.suisdks.sui.client.SuiClient
import com.suisdks.sui.jsonrpc.JsonRpcClient
import com.suisdks.sui.transactions.Transaction
import java.math.BigInteger
import java.util.Base64
import java.util.Locale

class DeepBookClient(
    private val callFn: (String, List<Any?>) -> Map<String, Any?>,
    val config: DeepBookConfig,
) {
    companion object {
        fun fromJsonRpc(client: JsonRpcClient, config: DeepBookConfig): DeepBookClient =
            DeepBookClient({ method, params -> client.call(method, params) }, config)

        fun fromSuiClient(client: SuiClient, config: DeepBookConfig): DeepBookClient =
            DeepBookClient({ method, params -> client.execute(method, params) }, config)
    }

    fun checkManagerBalance(managerKey: String, coinKey: String): Map<String, Any?> {
        val manager = config.getBalanceManager(managerKey)
        val coin = config.getCoin(coinKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::balance_manager::balance",
                listOf(tx.`object`(manager.address)),
                listOf(coin.type),
            )
        }
        val balance = readU64(sim, 0, 0).toDouble() / coin.scalar
        return mapOf("coinType" to coin.type, "balance" to balance)
    }

    fun whitelisted(poolKey: String): Boolean {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::whitelisted",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun getQuoteQuantityOut(poolKey: String, baseQuantity: Double): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_quote_quantity_out",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(baseQuantity, base.scalar))),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "baseQuantity" to baseQuantity,
            "baseOut" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteOut" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun getBaseQuantityOut(poolKey: String, quoteQuantity: Double): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_base_quantity_out",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(quoteQuantity, quote.scalar))),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "quoteQuantity" to quoteQuantity,
            "baseOut" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteOut" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun getQuoteQuantityOutInputFee(poolKey: String, baseQuantity: Double): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_quote_quantity_out_input_fee",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(baseQuantity, base.scalar))),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "baseQuantity" to baseQuantity,
            "baseOut" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteOut" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun getBaseQuantityOutInputFee(poolKey: String, quoteQuantity: Double): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_base_quantity_out_input_fee",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(quoteQuantity, quote.scalar))),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "quoteQuantity" to quoteQuantity,
            "baseOut" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteOut" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun getQuantityOut(poolKey: String, baseQuantity: Double, quoteQuantity: Double): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_quantity_out",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(baseQuantity, base.scalar))),
                    tx.pure(u64(toRaw(quoteQuantity, quote.scalar))),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "baseQuantity" to baseQuantity,
            "quoteQuantity" to quoteQuantity,
            "baseOut" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteOut" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun getQuantityOutInputFee(poolKey: String, baseQuantity: Double, quoteQuantity: Double): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_quantity_out_input_fee",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(baseQuantity, base.scalar))),
                    tx.pure(u64(toRaw(quoteQuantity, quote.scalar))),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "baseQuantity" to baseQuantity,
            "quoteQuantity" to quoteQuantity,
            "baseOut" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteOut" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun getBaseQuantityIn(poolKey: String, targetQuoteQuantity: Double, payWithDeep: Boolean): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_base_quantity_in",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(targetQuoteQuantity, quote.scalar))),
                    tx.pure(bool(payWithDeep)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "baseIn" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteOut" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun getQuoteQuantityIn(poolKey: String, targetBaseQuantity: Double, payWithDeep: Boolean): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_quote_quantity_in",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(targetBaseQuantity, base.scalar))),
                    tx.pure(bool(payWithDeep)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "baseOut" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quoteIn" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deepRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun midPrice(poolKey: String): Double {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::mid_price",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        val value = readU64(sim, 0, 0).toDouble()
        return (value * base.scalar) / (FLOAT_SCALAR * quote.scalar)
    }

    fun getPoolIdByAssets(baseType: String, quoteType: String): String {
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_pool_id_by_asset",
                listOf(tx.pure(address(baseType)), tx.pure(address(quoteType))),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getBalanceManagerIds(owner: String): List<String> {
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::registry::get_balance_manager_ids",
                listOf(tx.pure(address(owner))),
            )
        }
        return readVecAddress(readBcs(sim, 0, 0))
    }

    fun balanceManagerReferralOwner(referral: String): String {
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::balance_manager::balance_manager_referral_owner",
                listOf(tx.pure(address(referral))),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun balanceManagerReferralPoolId(referral: String): String {
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::balance_manager::balance_manager_referral_pool_id",
                listOf(tx.pure(address(referral))),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getPoolReferralBalances(poolKey: String, referral: String): Map<String, Double> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_pool_referral_balances",
                listOf(tx.`object`(pool.address), tx.pure(address(referral))),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "base" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quote" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deep" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun poolReferralMultiplier(poolKey: String, referral: String): Double {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::pool_referral_multiplier",
                listOf(tx.`object`(pool.address), tx.pure(address(referral))),
                listOf(base.type, quote.type),
            )
        }
        return readU64(sim, 0, 0).toDouble() / FLOAT_SCALAR
    }

    fun accountOpenOrders(poolKey: String, managerKey: String): List<String> {
        val pool = config.getPool(poolKey)
        val manager = config.getBalanceManager(managerKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::account_open_orders",
                listOf(tx.`object`(pool.address), tx.`object`(manager.address)),
                listOf(base.type, quote.type),
            )
        }
        return readVecU128(readBcs(sim, 0, 0)).map { it.toString() }
    }

    fun getOrder(poolKey: String, orderId: String): Map<String, Any?>? {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_order",
                listOf(tx.`object`(pool.address), tx.pure(u128(orderId))),
                listOf(base.type, quote.type),
            )
        }
        return runCatching { readOrder(readBcs(sim, 0, 0)) }.getOrNull()
    }

    fun getOrders(poolKey: String, orderIds: List<String>): List<Map<String, Any?>> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            val ids = tx.pure(vecU128(orderIds))
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_orders",
                listOf(tx.`object`(pool.address), ids),
                listOf(base.type, quote.type),
            )
        }
        return runCatching { readVecOrder(readBcs(sim, 0, 0)) }.getOrDefault(emptyList())
    }

    fun canPlaceMarketOrder(
        poolKey: String,
        managerKey: String,
        quantity: Double,
        isBid: Boolean,
        payWithDeep: Boolean,
    ): Boolean {
        val pool = config.getPool(poolKey)
        val manager = config.getBalanceManager(managerKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::can_place_market_order",
                listOf(
                    tx.`object`(pool.address),
                    tx.`object`(manager.address),
                    tx.pure(u64(toRaw(quantity, if (isBid) quote.scalar else base.scalar))),
                    tx.pure(bool(isBid)),
                    tx.pure(bool(payWithDeep)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun canPlaceLimitOrder(
        poolKey: String,
        managerKey: String,
        price: Double,
        quantity: Double,
        isBid: Boolean,
        payWithDeep: Boolean,
        expireTimestamp: Long,
    ): Boolean {
        val pool = config.getPool(poolKey)
        val manager = config.getBalanceManager(managerKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val priceRaw = ((price * FLOAT_SCALAR * quote.scalar) / base.scalar).toLong()
        val quantityRaw = toRaw(quantity, base.scalar)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::can_place_limit_order",
                listOf(
                    tx.`object`(pool.address),
                    tx.`object`(manager.address),
                    tx.pure(u64(priceRaw)),
                    tx.pure(u64(quantityRaw)),
                    tx.pure(bool(isBid)),
                    tx.pure(bool(payWithDeep)),
                    tx.pure(u64(expireTimestamp)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun checkMarketOrderParams(poolKey: String, quantity: Double): Boolean {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::check_market_order_params",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(toRaw(quantity, base.scalar))),
                ),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun checkLimitOrderParams(poolKey: String, price: Double, quantity: Double, expireTimestamp: Long): Boolean {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val priceRaw = ((price * FLOAT_SCALAR * quote.scalar) / base.scalar).toLong()
        val quantityRaw = toRaw(quantity, base.scalar)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::check_limit_order_params",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(priceRaw)),
                    tx.pure(u64(quantityRaw)),
                    tx.pure(u64(expireTimestamp)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun poolId(poolKey: String): String {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::id",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun quorum(poolKey: String): Double {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::quorum",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return readU64(sim, 0, 0).toDouble() / DEEP_SCALAR
    }

    fun stablePool(poolKey: String): Boolean {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::stable_pool",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun registeredPool(poolKey: String): Boolean {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::registered_pool",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun getOrderDeepRequired(poolKey: String, baseQuantity: Double, price: Double): Map<String, Double> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val priceRaw = ((price * FLOAT_SCALAR * quote.scalar) / base.scalar).toLong()
        val quantityRaw = toRaw(baseQuantity, base.scalar)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_order_deep_required",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(quantityRaw)),
                    tx.pure(u64(priceRaw)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "deepRequiredTaker" to (readU64(sim, 0, 0).toDouble() / DEEP_SCALAR),
            "deepRequiredMaker" to (readU64(sim, 0, 1).toDouble() / DEEP_SCALAR),
        )
    }

    fun getLevel2Range(poolKey: String, priceLow: Double, priceHigh: Double, isBid: Boolean): Map<String, List<Double>> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val priceLowRaw = ((priceLow * FLOAT_SCALAR * quote.scalar) / base.scalar).toLong()
        val priceHighRaw = ((priceHigh * FLOAT_SCALAR * quote.scalar) / base.scalar).toLong()
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_level2_range",
                listOf(
                    tx.`object`(pool.address),
                    tx.pure(u64(priceLowRaw)),
                    tx.pure(u64(priceHighRaw)),
                    tx.pure(bool(isBid)),
                ),
                listOf(base.type, quote.type),
            )
        }
        val rawPrices = readVecU64(readBcs(sim, 0, 0))
        val rawQuantities = readVecU64(readBcs(sim, 0, 1))
        return mapOf(
            "prices" to rawPrices.map { ((it / FLOAT_SCALAR) / quote.scalar) * base.scalar },
            "quantities" to rawQuantities.map { it / base.scalar },
        )
    }

    fun getLevel2TicksFromMid(poolKey: String, ticks: Int): Map<String, List<Double>> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_level2_ticks_from_mid",
                listOf(tx.`object`(pool.address), tx.pure(u64(ticks.toLong()))),
                listOf(base.type, quote.type),
            )
        }
        val bidPrices = readVecU64(readBcs(sim, 0, 0))
        val bidQty = readVecU64(readBcs(sim, 0, 1))
        val askPrices = readVecU64(readBcs(sim, 0, 2))
        val askQty = readVecU64(readBcs(sim, 0, 3))
        return mapOf(
            "bid_prices" to bidPrices.map { ((it / FLOAT_SCALAR) / quote.scalar) * base.scalar },
            "bid_quantities" to bidQty.map { it / base.scalar },
            "ask_prices" to askPrices.map { ((it / FLOAT_SCALAR) / quote.scalar) * base.scalar },
            "ask_quantities" to askQty.map { it / base.scalar },
        )
    }

    fun getPoolDeepPrice(poolKey: String): Map<String, Any> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val deep = config.getCoin("DEEP")
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_pool_deep_price",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        val reader = BcsReader(readBcs(sim, 0, 0))
        val assetIsBase = reader.readBool()
        val deepPerAsset = reader.readU64().toDouble()
        return if (assetIsBase) {
            mapOf(
                "asset_is_base" to true,
                "deep_per_base" to ((deepPerAsset / FLOAT_SCALAR) * base.scalar) / deep.scalar,
            )
        } else {
            mapOf(
                "asset_is_base" to false,
                "deep_per_quote" to ((deepPerAsset / FLOAT_SCALAR) * quote.scalar) / deep.scalar,
            )
        }
    }

    fun account(poolKey: String, managerKey: String): String {
        val pool = config.getPool(poolKey)
        val manager = config.getBalanceManager(managerKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::account",
                listOf(tx.`object`(pool.address), tx.`object`(manager.address)),
                listOf(base.type, quote.type),
            )
        }
        return Base64.getEncoder().encodeToString(readBcs(sim, 0, 0))
    }

    fun getOrderNormalized(poolKey: String, orderId: String): Map<String, Any?>? {
        val order = getOrder(poolKey, orderId) ?: return null
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val decoded = decodeOrderId(order["order_id"].toString())
        @Suppress("UNCHECKED_CAST")
        val deepPrice = order["order_deep_price"] as Map<String, Any?>
        val normalized = order.toMutableMap()
        normalized["quantity"] = order["quantity"].toString().toDouble() / base.scalar
        normalized["filled_quantity"] = order["filled_quantity"].toString().toDouble() / base.scalar
        normalized["order_deep_price"] = mapOf(
            "asset_is_base" to (deepPrice["asset_is_base"] == true),
            "deep_per_asset" to (deepPrice["deep_per_asset"].toString().toDouble() / DEEP_SCALAR),
        )
        val normalizedPrice = (decoded["price"].toString().toDouble() * base.scalar) / (FLOAT_SCALAR * quote.scalar)
        normalized["is_bid"] = decoded["isBid"] == true
        normalized["normalized_price"] = normalizedPrice
        return normalized
    }

    fun vaultBalances(poolKey: String): Map<String, Double> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::vault_balances",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "base" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quote" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deep" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun poolTradeParams(poolKey: String): Map<String, Double> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::pool_trade_params",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "takerFee" to (readU64(sim, 0, 0).toDouble() / FLOAT_SCALAR),
            "makerFee" to (readU64(sim, 0, 1).toDouble() / FLOAT_SCALAR),
            "stakeRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun poolTradeParamsNext(poolKey: String): Map<String, Double> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::pool_trade_params_next",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "takerFee" to (readU64(sim, 0, 0).toDouble() / FLOAT_SCALAR),
            "makerFee" to (readU64(sim, 0, 1).toDouble() / FLOAT_SCALAR),
            "stakeRequired" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun poolBookParams(poolKey: String): Map<String, Double> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::pool_book_params",
                listOf(tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        val tickSize = readU64(sim, 0, 0).toDouble()
        val lotSize = readU64(sim, 0, 1).toDouble()
        val minSize = readU64(sim, 0, 2).toDouble()
        return mapOf(
            "tickSize" to ((tickSize * base.scalar) / (FLOAT_SCALAR * quote.scalar)),
            "lotSize" to (lotSize / base.scalar),
            "minSize" to (minSize / base.scalar),
        )
    }

    fun lockedBalance(poolKey: String, managerKey: String): Map<String, Double> {
        val pool = config.getPool(poolKey)
        val manager = config.getBalanceManager(managerKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::locked_balance",
                listOf(tx.`object`(pool.address), tx.`object`(manager.address)),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "base" to (readU64(sim, 0, 0).toDouble() / base.scalar),
            "quote" to (readU64(sim, 0, 1).toDouble() / quote.scalar),
            "deep" to (readU64(sim, 0, 2).toDouble() / DEEP_SCALAR),
        )
    }

    fun accountExists(poolKey: String, managerKey: String): Boolean {
        val pool = config.getPool(poolKey)
        val manager = config.getBalanceManager(managerKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::account_exists",
                listOf(tx.`object`(pool.address), tx.`object`(manager.address)),
                listOf(base.type, quote.type),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun getAllowedMaintainers(): List<String> {
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::allowed_maintainers",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
            )
        }
        return readVecAddress(readBcs(sim, 0, 0))
    }

    fun getAllowedPauseCaps(): List<String> {
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::allowed_pause_caps",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
            )
        }
        return readVecAddress(readBcs(sim, 0, 0))
    }

    fun isPoolEnabledForMargin(poolKey: String): Boolean {
        val pool = config.getPool(poolKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::pool_enabled",
                listOf(
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.pure(address(pool.address)),
                ),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun getMarginPoolId(coinKey: String): String {
        val coin = config.getCoin(coinKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_margin_pool_id",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
                listOf(coin.type),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getDeepbookPoolMarginPoolIds(poolKey: String): Map<String, String> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_deepbook_pool_margin_pool_ids",
                listOf(
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.pure(address(pool.address)),
                ),
                listOf(base.type, quote.type),
            )
        }
        val reader = BcsReader(readBcs(sim, 0, 0))
        return mapOf(
            "baseMarginPoolId" to toAddress(reader.readBytes(32)),
            "quoteMarginPoolId" to toAddress(reader.readBytes(32)),
        )
    }

    fun getMarginManagerIdsForOwner(owner: String): List<String> {
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_margin_manager_ids",
                listOf(
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.pure(address(owner)),
                ),
            )
        }
        return readVecAddress(readBcs(sim, 0, 0))
    }

    fun getBaseMarginPoolId(poolKey: String): String {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::base_margin_pool_id",
                listOf(
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.pure(address(pool.address)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getQuoteMarginPoolId(poolKey: String): String {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::quote_margin_pool_id",
                listOf(
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.pure(address(pool.address)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getMinWithdrawRiskRatio(poolKey: String): Double = readRiskRatio(poolKey, "min_withdraw_risk_ratio")

    fun getMinBorrowRiskRatio(poolKey: String): Double = readRiskRatio(poolKey, "min_borrow_risk_ratio")

    fun getLiquidationRiskRatio(poolKey: String): Double = readRiskRatio(poolKey, "liquidation_risk_ratio")

    fun getTargetLiquidationRiskRatio(poolKey: String): Double = readRiskRatio(poolKey, "target_liquidation_risk_ratio")

    fun getUserLiquidationReward(poolKey: String): Double = readRiskRatio(poolKey, "user_liquidation_reward")

    fun getPoolLiquidationReward(poolKey: String): Double = readRiskRatio(poolKey, "pool_liquidation_reward")

    fun getMarginManagerOwner(marginManagerKey: String): String {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::owner",
                listOf(tx.`object`(manager.address)),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getMarginManagerDeepbookPool(marginManagerKey: String): String {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::deepbook_pool",
                listOf(tx.`object`(manager.address)),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getMarginManagerMarginPoolId(marginManagerKey: String): String? {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::margin_pool_id",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readOptionAddress(readBcs(sim, 0, 0))
    }

    fun getMarginManagerBorrowedShares(marginManagerKey: String): Map<String, String> {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::borrowed_shares",
                listOf(tx.`object`(manager.address)),
            )
        }
        return mapOf(
            "baseShares" to readU64(sim, 0, 0).toString(),
            "quoteShares" to readU64(sim, 0, 1).toString(),
        )
    }

    fun getMarginManagerBorrowedBaseShares(marginManagerKey: String): String {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::borrowed_base_shares",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readU64(sim, 0, 0).toString()
    }

    fun getMarginManagerBorrowedQuoteShares(marginManagerKey: String): String {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::borrowed_quote_shares",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readU64(sim, 0, 0).toString()
    }

    fun getMarginManagerHasBaseDebt(marginManagerKey: String): Boolean {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::has_base_debt",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readBool(sim, 0, 0)
    }

    fun getMarginManagerBalanceManagerId(marginManagerKey: String): String {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::balance_manager",
                listOf(tx.`object`(manager.address)),
            )
        }
        return toAddress(readBcs(sim, 0, 0))
    }

    fun getMarginManagerBaseBalance(marginManagerKey: String): Double {
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val base = config.getCoin(pool.baseCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::base_balance",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readU64(sim, 0, 0).toDouble() / base.scalar
    }

    fun getMarginManagerBaseBalance(marginManagerKey: String, decimals: Int): String {
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val base = config.getCoin(pool.baseCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::base_balance",
                listOf(tx.`object`(manager.address)),
            )
        }
        return formatTokenAmount(readU64(sim, 0, 0), base.scalar, decimals)
    }

    fun getMarginManagerQuoteBalance(marginManagerKey: String): Double {
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::quote_balance",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readU64(sim, 0, 0).toDouble() / quote.scalar
    }

    fun getMarginManagerQuoteBalance(marginManagerKey: String, decimals: Int): String {
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::quote_balance",
                listOf(tx.`object`(manager.address)),
            )
        }
        return formatTokenAmount(readU64(sim, 0, 0), quote.scalar, decimals)
    }

    fun getMarginManagerDeepBalance(marginManagerKey: String): Double {
        val manager = config.getMarginManager(marginManagerKey)
        val deep = config.getCoin("DEEP")
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::deep_balance",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readU64(sim, 0, 0).toDouble() / deep.scalar
    }

    fun getMarginManagerDeepBalance(marginManagerKey: String, decimals: Int): String {
        val manager = config.getMarginManager(marginManagerKey)
        val deep = config.getCoin("DEEP")
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::deep_balance",
                listOf(tx.`object`(manager.address)),
            )
        }
        return formatTokenAmount(readU64(sim, 0, 0), deep.scalar, decimals)
    }

    fun getConditionalOrderIds(marginManagerKey: String): List<String> {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_tpsl::conditional_order_ids",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readVecU64(readBcs(sim, 0, 0)).map { it.toLong().toString() }
    }

    fun getLowestTriggerAbovePrice(marginManagerKey: String): Long {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_tpsl::lowest_trigger_above_price",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readU64(sim, 0, 0)
    }

    fun getHighestTriggerBelowPrice(marginManagerKey: String): Long {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_tpsl::highest_trigger_below_price",
                listOf(tx.`object`(manager.address)),
            )
        }
        return readU64(sim, 0, 0)
    }

    fun isDeepbookPoolAllowed(coinKey: String, deepbookPoolId: String): Boolean {
        val coin = config.getCoin(coinKey)
        val sim = simulate { tx ->
            val marginPoolId = tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_margin_pool_id",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
                listOf(coin.type),
            )
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_pool::deepbook_pool_allowed",
                listOf(marginPoolId, tx.pure(address(deepbookPoolId))),
                listOf(coin.type),
            )
        }
        return readBool(sim, 1, 0)
    }

    fun getMarginPoolTotalSupply(coinKey: String): Double = marginPoolAmountView(coinKey, "total_supply")
    fun getMarginPoolTotalSupply(coinKey: String, decimals: Int): String = marginPoolAmountView(coinKey, "total_supply", decimals)

    fun getMarginPoolSupplyShares(coinKey: String): Double = marginPoolAmountView(coinKey, "supply_shares")
    fun getMarginPoolSupplyShares(coinKey: String, decimals: Int): String = marginPoolAmountView(coinKey, "supply_shares", decimals)

    fun getMarginPoolTotalBorrow(coinKey: String): Double = marginPoolAmountView(coinKey, "total_borrow")
    fun getMarginPoolTotalBorrow(coinKey: String, decimals: Int): String = marginPoolAmountView(coinKey, "total_borrow", decimals)

    fun getMarginPoolBorrowShares(coinKey: String): Double = marginPoolAmountView(coinKey, "borrow_shares")
    fun getMarginPoolBorrowShares(coinKey: String, decimals: Int): String = marginPoolAmountView(coinKey, "borrow_shares", decimals)

    fun getMarginPoolLastUpdateTimestamp(coinKey: String): Long = marginPoolU64View(coinKey, "last_update_timestamp")

    fun getMarginPoolSupplyCap(coinKey: String): Double = marginPoolAmountView(coinKey, "supply_cap")
    fun getMarginPoolSupplyCap(coinKey: String, decimals: Int): String = marginPoolAmountView(coinKey, "supply_cap", decimals)

    fun getMarginPoolMaxUtilizationRate(coinKey: String): Double = marginPoolU64View(coinKey, "max_utilization_rate") / FLOAT_SCALAR

    fun getMarginPoolProtocolSpread(coinKey: String): Double = marginPoolU64View(coinKey, "protocol_spread") / FLOAT_SCALAR

    fun getMarginPoolMinBorrow(coinKey: String): Double = marginPoolAmountView(coinKey, "min_borrow")
    fun getMarginPoolMinBorrow(coinKey: String, decimals: Int): String = marginPoolAmountView(coinKey, "min_borrow", decimals)

    fun getMarginPoolInterestRate(coinKey: String): Double = marginPoolU64View(coinKey, "interest_rate") / FLOAT_SCALAR

    fun getUserSupplyShares(coinKey: String, supplierCapId: String): Double =
        marginPoolUserU64View(coinKey, "user_supply_shares", supplierCapId, includeClock = false)
    fun getUserSupplyShares(coinKey: String, supplierCapId: String, decimals: Int): String =
        marginPoolUserU64View(coinKey, "user_supply_shares", supplierCapId, decimals = decimals, includeClock = false)

    fun getUserSupplyAmount(coinKey: String, supplierCapId: String): Double =
        marginPoolUserU64View(coinKey, "user_supply_amount", supplierCapId, includeClock = true)
    fun getUserSupplyAmount(coinKey: String, supplierCapId: String, decimals: Int): String =
        marginPoolUserU64View(coinKey, "user_supply_amount", supplierCapId, decimals = decimals, includeClock = true)

    fun getBalanceManagerReferralId(managerKey: String, poolKey: String): String? {
        val manager = config.getBalanceManager(managerKey)
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::balance_manager::get_balance_manager_referral_id",
                listOf(tx.`object`(manager.address), tx.pure(address(pool.address))),
                listOf(base.type, quote.type),
            )
        }
        return readOptionAddress(readBcs(sim, 0, 0))
    }

    fun getAccountOrderDetails(poolKey: String, managerKey: String): List<Map<String, Any?>> {
        val pool = config.getPool(poolKey)
        val manager = config.getBalanceManager(managerKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.deepbookPackageId}::pool::get_account_order_details",
                listOf(tx.`object`(pool.address), tx.`object`(manager.address)),
                listOf(base.type, quote.type),
            )
        }
        return runCatching { readVecOrder(readBcs(sim, 0, 0)) }.getOrDefault(emptyList())
    }

    fun getMarginAccountOrderDetails(marginManagerKey: String): String {
        val manager = config.getMarginManager(marginManagerKey)
        val sim = simulate { tx ->
            val account = tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::get_margin_account_order_details",
                listOf(tx.`object`(manager.address)),
            )
            // Keep parity with py baseline: return second call return bytes as base64.
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::balance_manager",
                listOf(account),
            )
        }
        return Base64.getEncoder().encodeToString(readBcs(sim, 1, 0))
    }

    fun getPriceInfoObject(coinKey: String): String {
        val coin = config.getCoin(coinKey)
        val objectId = coin.priceInfoObjectId ?: throw IllegalArgumentException("price_info_object_id not configured for coin: $coinKey")
        // Keep py parity: this getter returns object id after validating read-path availability.
        getPriceInfoObjectAge(coinKey)
        return objectId
    }

    fun getPriceInfoObjectRaw(coinKey: String): Map<String, Any?> {
        val coin = config.getCoin(coinKey)
        val objectId = coin.priceInfoObjectId ?: throw IllegalArgumentException("price_info_object_id not configured for coin: $coinKey")
        return callFn("sui_getObject", listOf(objectId, mapOf("showContent" to true)))
    }

    fun getPriceInfoObjects(coinKeys: List<String>): Map<String, String> {
        val out = LinkedHashMap<String, String>()
        coinKeys.forEach { out[it] = getPriceInfoObject(it) }
        return out
    }

    fun getPriceInfoObjects(): Map<String, String> {
        val coinKeys = config.coins.filterValues { it.priceInfoObjectId != null }.keys.toList()
        return getPriceInfoObjects(coinKeys)
    }

    fun getPriceInfoObjectAge(coinKey: String): Long {
        val obj = getPriceInfoObjectRaw(coinKey)
        return extractArrivalTime(obj)
            ?: throw IllegalArgumentException("arrival_time not found in price info object for coin: $coinKey")
    }

    fun getPriceInfoObjectStaleness(coinKey: String, nowEpochSeconds: Long = System.currentTimeMillis() / 1000): Long {
        val arrival = getPriceInfoObjectAge(coinKey)
        return (nowEpochSeconds - arrival).coerceAtLeast(0)
    }

    fun getMarginManagerState(marginManagerKey: String): Map<String, Any?> {
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::manager_state",
                listOf(
                    tx.`object`(manager.address),
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.`object`(pool.address),
                    tx.`object`("0x6"),
                ),
                listOf(base.type, quote.type),
            )
        }
        return parseMarginManagerState(sim, commandIndex = 0, marginManagerKey = marginManagerKey, decimals = null)
    }

    fun getMarginManagerState(marginManagerKey: String, decimals: Int): Map<String, Any?> {
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::manager_state",
                listOf(
                    tx.`object`(manager.address),
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.`object`(pool.address),
                    tx.`object`("0x6"),
                ),
                listOf(base.type, quote.type),
            )
        }
        return parseMarginManagerState(sim, commandIndex = 0, marginManagerKey = marginManagerKey, decimals = decimals)
    }

    fun getMarginManagerStates(marginManagerKeys: List<String>): Map<String, Map<String, Any?>> {
        val sim = simulate { tx ->
            marginManagerKeys.forEach { key ->
                val manager = config.getMarginManager(key)
                val pool = config.getPool(manager.poolKey)
                val base = config.getCoin(pool.baseCoin)
                val quote = config.getCoin(pool.quoteCoin)
                tx.moveCall(
                    "${config.packageIds.marginPackageId}::margin_manager::manager_state",
                    listOf(
                        tx.`object`(manager.address),
                        tx.`object`(config.packageIds.marginRegistryId),
                        tx.`object`(pool.address),
                        tx.`object`("0x6"),
                    ),
                    listOf(base.type, quote.type),
                )
            }
        }
        return marginManagerKeys.withIndex().associate { (i, key) ->
            key to parseMarginManagerState(sim, commandIndex = i, marginManagerKey = key, decimals = null)
        }
    }

    fun getMarginManagerStates(marginManagers: Map<String, String>, decimals: Int = 6): Map<String, Map<String, Any?>> {
        val entries = marginManagers.entries.toList()
        if (entries.isEmpty()) return emptyMap()
        val sim = simulate { tx ->
            entries.forEach { (managerId, poolKey) ->
                val pool = config.getPool(poolKey)
                val base = config.getCoin(pool.baseCoin)
                val quote = config.getCoin(pool.quoteCoin)
                tx.moveCall(
                    "${config.packageIds.marginPackageId}::margin_manager::manager_state",
                    listOf(
                        tx.`object`(managerId),
                        tx.`object`(config.packageIds.marginRegistryId),
                        tx.`object`(pool.address),
                        tx.`object`("0x6"),
                    ),
                    listOf(base.type, quote.type),
                )
            }
        }
        val out = LinkedHashMap<String, Map<String, Any?>>()
        entries.withIndex().forEach { (i, entry) ->
            val parsed = parseMarginManagerStateByPoolKey(sim, i, entry.value, decimals)
            out[parsed["managerId"].toString()] = parsed
        }
        return out
    }

    fun getMarginManagerAssets(marginManagerKey: String): Map<String, Double> {
        val state = getMarginManagerState(marginManagerKey)
        return mapOf(
            "base" to (state["baseAsset"] as Double),
            "quote" to (state["quoteAsset"] as Double),
        )
    }

    fun getMarginManagerAssets(marginManagerKey: String, decimals: Int): Map<String, String> {
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::calculate_assets",
                listOf(tx.`object`(manager.address), tx.`object`(pool.address)),
                listOf(base.type, quote.type),
            )
        }
        return mapOf(
            "baseAsset" to formatTokenAmount(readU64(sim, 0, 0), base.scalar, decimals),
            "quoteAsset" to formatTokenAmount(readU64(sim, 0, 1), quote.scalar, decimals),
        )
    }

    fun getMarginManagerDebts(marginManagerKey: String): Map<String, Double> {
        val state = getMarginManagerState(marginManagerKey)
        return mapOf(
            "base" to (state["baseDebt"] as Double),
            "quote" to (state["quoteDebt"] as Double),
        )
    }

    fun getMarginManagerDebts(marginManagerKey: String, decimals: Int): Map<String, String> {
        val hasBaseDebt = getMarginManagerHasBaseDebt(marginManagerKey)
        val manager = config.getMarginManager(marginManagerKey)
        val pool = config.getPool(manager.poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val debtCoin = config.getCoin(if (hasBaseDebt) pool.baseCoin else pool.quoteCoin)
        val sim = simulate { tx ->
            val marginPoolId = tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_margin_pool_id",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
                listOf(debtCoin.type),
            )
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_manager::calculate_debts",
                listOf(tx.`object`(manager.address), marginPoolId, tx.`object`("0x6")),
                listOf(base.type, quote.type, debtCoin.type),
            )
        }
        return mapOf(
            "baseDebt" to formatTokenAmount(readU64(sim, 1, 0), debtCoin.scalar, decimals),
            "quoteDebt" to formatTokenAmount(readU64(sim, 1, 1), debtCoin.scalar, decimals),
        )
    }

    fun decodeOrderId(encodedOrderId: String): Map<String, Any> {
        val value = BigInteger(encodedOrderId)
        val isBid = value.shiftRight(127) == BigInteger.ZERO
        val price = value.shiftRight(64).and(BigInteger("7fffffffffffffff", 16))
        val orderId = value.and(BigInteger("ffffffffffffffff", 16))
        return mapOf(
            "isBid" to isBid,
            "price" to price.toString(),
            "orderId" to orderId.toString(),
        )
    }

    private fun simulate(build: (Transaction) -> Unit): Map<String, Any?> {
        val tx = Transaction()
        build(tx)
        val raw = callFn("sui_dryRunTransactionBlock", listOf(tx.buildBase64()))
        @Suppress("UNCHECKED_CAST")
        return (raw["result"] as? Map<String, Any?>) ?: raw
    }

    private fun readU64(sim: Map<String, Any?>, commandIndex: Int, returnIndex: Int): Long {
        val reader = BcsReader(readBcs(sim, commandIndex, returnIndex))
        return reader.readU64().toLong()
    }

    private fun readBool(sim: Map<String, Any?>, commandIndex: Int, returnIndex: Int): Boolean {
        val reader = BcsReader(readBcs(sim, commandIndex, returnIndex))
        return reader.readBool()
    }

    private fun readVecAddress(raw: ByteArray): List<String> {
        val reader = BcsReader(raw)
        val len = reader.readUleb128().toInt()
        val out = ArrayList<String>(len)
        repeat(len) {
            out.add(toAddress(reader.readBytes(32)))
        }
        return out
    }

    private fun readVecU128(raw: ByteArray): List<BigInteger> {
        val reader = BcsReader(raw)
        val len = reader.readUleb128().toInt()
        val out = ArrayList<BigInteger>(len)
        repeat(len) {
            out.add(leBytesToBigInt(reader.readBytes(16)))
        }
        return out
    }

    private fun readVecU64(raw: ByteArray): List<Double> {
        val reader = BcsReader(raw)
        val len = reader.readUleb128().toInt()
        val out = ArrayList<Double>(len)
        repeat(len) {
            out.add(reader.readU64().toDouble())
        }
        return out
    }

    private fun readRiskRatio(poolKey: String, fnName: String): Double {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val sim = simulate { tx ->
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::$fnName",
                listOf(
                    tx.`object`(config.packageIds.marginRegistryId),
                    tx.pure(address(pool.address)),
                ),
                listOf(base.type, quote.type),
            )
        }
        return readU64(sim, 0, 0).toDouble() / FLOAT_SCALAR
    }

    private fun marginPoolU64View(coinKey: String, fnName: String): Long {
        val coin = config.getCoin(coinKey)
        val sim = simulate { tx ->
            val marginPoolId = tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_margin_pool_id",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
                listOf(coin.type),
            )
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_pool::$fnName",
                listOf(marginPoolId),
                listOf(coin.type),
            )
        }
        return readU64(sim, 1, 0)
    }

    private fun marginPoolAmountView(coinKey: String, fnName: String): Double {
        val coin = config.getCoin(coinKey)
        return marginPoolU64View(coinKey, fnName).toDouble() / coin.scalar
    }

    private fun marginPoolAmountView(coinKey: String, fnName: String, decimals: Int): String {
        val coin = config.getCoin(coinKey)
        return formatTokenAmount(marginPoolU64View(coinKey, fnName), coin.scalar, decimals)
    }

    private fun marginPoolUserU64View(coinKey: String, fnName: String, supplierCapId: String, includeClock: Boolean): Double {
        val coin = config.getCoin(coinKey)
        val sim = simulate { tx ->
            val marginPoolId = tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_margin_pool_id",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
                listOf(coin.type),
            )
            val args = mutableListOf<Map<String, Any?>>(marginPoolId, tx.pure(address(supplierCapId)))
            if (includeClock) {
                args.add(tx.`object`("0x6"))
            }
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_pool::$fnName",
                args,
                listOf(coin.type),
            )
        }
        return readU64(sim, 1, 0).toDouble() / coin.scalar
    }

    private fun marginPoolUserU64View(
        coinKey: String,
        fnName: String,
        supplierCapId: String,
        decimals: Int,
        includeClock: Boolean,
    ): String {
        val coin = config.getCoin(coinKey)
        val sim = simulate { tx ->
            val marginPoolId = tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_registry::get_margin_pool_id",
                listOf(tx.`object`(config.packageIds.marginRegistryId)),
                listOf(coin.type),
            )
            val args = mutableListOf<Map<String, Any?>>(marginPoolId, tx.pure(address(supplierCapId)))
            if (includeClock) {
                args.add(tx.`object`("0x6"))
            }
            tx.moveCall(
                "${config.packageIds.marginPackageId}::margin_pool::$fnName",
                args,
                listOf(coin.type),
            )
        }
        return formatTokenAmount(readU64(sim, 1, 0), coin.scalar, decimals)
    }

    private fun parseMarginManagerState(
        sim: Map<String, Any?>,
        commandIndex: Int,
        marginManagerKey: String,
        decimals: Int?,
    ): Map<String, Any?> {
        val managerCfg = config.getMarginManager(marginManagerKey)
        return parseMarginManagerStateByPoolKey(sim, commandIndex, managerCfg.poolKey, decimals)
    }

    private fun parseMarginManagerStateByPoolKey(
        sim: Map<String, Any?>,
        commandIndex: Int,
        poolKey: String,
        decimals: Int?,
    ): Map<String, Any?> {
        val pool = config.getPool(poolKey)
        val base = config.getCoin(pool.baseCoin)
        val quote = config.getCoin(pool.quoteCoin)
        val managerId = toAddress(readBcs(sim, commandIndex, 0))
        val deepbookPoolId = toAddress(readBcs(sim, commandIndex, 1))
        val riskRatio = readU64(sim, commandIndex, 2).toDouble() / FLOAT_SCALAR
        val baseAssetRaw = readU64(sim, commandIndex, 3)
        val quoteAssetRaw = readU64(sim, commandIndex, 4)
        val baseDebtRaw = readU64(sim, commandIndex, 5)
        val quoteDebtRaw = readU64(sim, commandIndex, 6)
        val basePythPriceRaw = readU64(sim, commandIndex, 7)
        val basePythDecimals = readU8(sim, commandIndex, 8)
        val quotePythPriceRaw = readU64(sim, commandIndex, 9)
        val quotePythDecimals = readU8(sim, commandIndex, 10)
        val currentPrice = readU64(sim, commandIndex, 11)
        val lowestTriggerAbovePrice = readU64(sim, commandIndex, 12)
        val highestTriggerBelowPrice = readU64(sim, commandIndex, 13)
        val baseAsset: Any = if (decimals == null) baseAssetRaw.toDouble() / base.scalar else formatTokenAmount(baseAssetRaw, base.scalar, decimals)
        val quoteAsset: Any = if (decimals == null) quoteAssetRaw.toDouble() / quote.scalar else formatTokenAmount(quoteAssetRaw, quote.scalar, decimals)
        val baseDebt: Any = if (decimals == null) baseDebtRaw.toDouble() / base.scalar else formatTokenAmount(baseDebtRaw, base.scalar, decimals)
        val quoteDebt: Any = if (decimals == null) quoteDebtRaw.toDouble() / quote.scalar else formatTokenAmount(quoteDebtRaw, quote.scalar, decimals)
        val basePythPrice: Any = if (decimals == null) basePythPriceRaw else basePythPriceRaw.toString()
        val quotePythPrice: Any = if (decimals == null) quotePythPriceRaw else quotePythPriceRaw.toString()
        return mapOf(
            "managerId" to managerId,
            "deepbookPoolId" to deepbookPoolId,
            "deepbookPool" to deepbookPoolId,
            "riskRatio" to riskRatio,
            "baseAsset" to baseAsset,
            "quoteAsset" to quoteAsset,
            "baseDebt" to baseDebt,
            "quoteDebt" to quoteDebt,
            "basePythPrice" to basePythPrice,
            "basePythDecimals" to basePythDecimals,
            "quotePythPrice" to quotePythPrice,
            "quotePythDecimals" to quotePythDecimals,
            "currentPrice" to currentPrice,
            "lowestTriggerAbovePrice" to lowestTriggerAbovePrice,
            "highestTriggerBelowPrice" to highestTriggerBelowPrice,
        )
    }

    private fun formatTokenAmount(rawAmount: Long, scalar: Double, decimals: Int): String {
        require(scalar > 0.0) { "invalid scalar" }
        val scaled = rawAmount / scalar
        return "%.${decimals.coerceAtLeast(0)}f".format(Locale.US, scaled).trimEnd('0').trimEnd('.').ifBlank { "0" }
    }

    private fun readOptionAddress(raw: ByteArray): String? {
        val reader = BcsReader(raw)
        return when (reader.readU8()) {
            0 -> null
            1 -> toAddress(reader.readBytes(32))
            else -> throw IllegalArgumentException("invalid option discriminant for address")
        }
    }

    private fun readU8(sim: Map<String, Any?>, commandIndex: Int, returnIndex: Int): Int {
        val reader = BcsReader(readBcs(sim, commandIndex, returnIndex))
        return reader.readU8()
    }

    private fun readOrder(raw: ByteArray): Map<String, Any?> {
        val reader = BcsReader(raw)
        val managerId = toAddress(reader.readBytes(32))
        val orderId = leBytesToBigInt(reader.readBytes(16)).toString()
        val clientOrderId = reader.readU64().toString()
        val quantity = reader.readU64().toString()
        val filledQuantity = reader.readU64().toString()
        val feeIsDeep = reader.readBool()
        val assetIsBase = reader.readBool()
        val deepPerAsset = reader.readU64().toString()
        val orderDeepPrice = mapOf(
            "asset_is_base" to assetIsBase,
            "deep_per_asset" to deepPerAsset,
        )
        val epoch = reader.readU64().toString()
        val status = reader.readU8()
        val expireTimestamp = reader.readU64().toString()

        return mapOf(
            "manager_id" to managerId,
            "order_id" to orderId,
            "client_order_id" to clientOrderId,
            "quantity" to quantity,
            "filled_quantity" to filledQuantity,
            "fee_is_deep" to feeIsDeep,
            "order_deep_price" to orderDeepPrice,
            "epoch" to epoch,
            "status" to status,
            "expire_timestamp" to expireTimestamp,
        )
    }

    private fun readVecOrder(raw: ByteArray): List<Map<String, Any?>> {
        val reader = BcsReader(raw)
        val len = reader.readUleb128().toInt()
        val out = ArrayList<Map<String, Any?>>(len)
        repeat(len) {
            // Fixed-size order payload in DeepBook views.
            out.add(readOrder(reader.readBytes(32 + 16 + 8 + 8 + 8 + 1 + 1 + 8 + 8 + 1 + 8)))
        }
        return out
    }

    private fun readBcs(sim: Map<String, Any?>, commandIndex: Int, returnIndex: Int): ByteArray {
        @Suppress("UNCHECKED_CAST")
        val commandResults = (sim["commandResults"] as? List<Map<String, Any?>>)
            ?: (sim["results"] as? List<Map<String, Any?>>)
            ?: throw IllegalArgumentException("missing commandResults in dryRun response")
        val command = commandResults.getOrNull(commandIndex)
            ?: throw IllegalArgumentException("missing command result index: $commandIndex")
        @Suppress("UNCHECKED_CAST")
        val returnValues = command["returnValues"] as? List<Map<String, Any?>>
            ?: throw IllegalArgumentException("missing returnValues for command index: $commandIndex")
        val value = returnValues.getOrNull(returnIndex)
            ?: throw IllegalArgumentException("missing return value index: $returnIndex")
        val bcsBase64 = value["bcs"]?.toString() ?: throw IllegalArgumentException("missing bcs field")
        return Base64.getDecoder().decode(bcsBase64)
    }

    private fun toRaw(amount: Double, scalar: Double): Long = (amount * scalar).toLong()

    private fun u64(value: Long): ByteArray {
        val writer = BcsWriter()
        writer.writeU64(value)
        return writer.toByteArray()
    }

    private fun bool(value: Boolean): ByteArray = byteArrayOf(if (value) 1 else 0)

    private fun u128(value: String): ByteArray {
        val n = BigInteger(value)
        require(n >= BigInteger.ZERO) { "u128 cannot be negative: $value" }
        require(n.bitLength() <= 128) { "u128 out of range: $value" }
        val out = ByteArray(16)
        var cur = n
        for (i in 0 until 16) {
            out[i] = cur.and(BigInteger("ff", 16)).toByte()
            cur = cur.shiftRight(8)
        }
        return out
    }

    private fun vecU128(values: List<String>): ByteArray {
        val writer = BcsWriter()
        writer.writeUleb128(values.size.toLong())
        values.forEach { writer.writeBytes(u128(it)) }
        return writer.toByteArray()
    }

    private fun address(value: String): ByteArray {
        val normalized = value.removePrefix("0x")
        if (normalized.length == 64 && normalized.all { it in "0123456789abcdefABCDEF" }) {
            return hexToBytes(normalized)
        }
        return value.toByteArray(Charsets.UTF_8)
    }

    private fun toAddress(value: ByteArray): String = "0x" + value.joinToString("") { "%02x".format(it.toInt() and 0xFF) }

    private fun hexToBytes(hex: String): ByteArray {
        val evenHex = if (hex.length % 2 == 0) hex else "0$hex"
        val out = ByteArray(evenHex.length / 2)
        for (i in out.indices) {
            val idx = i * 2
            out[i] = evenHex.substring(idx, idx + 2).toInt(16).toByte()
        }
        return out
    }

    private fun leBytesToBigInt(bytes: ByteArray): BigInteger = BigInteger(1, bytes.reversedArray())

    private fun extractArrivalTime(result: Map<String, Any?>): Long? {
        @Suppress("UNCHECKED_CAST")
        val root = (result["result"] as? Map<String, Any?>) ?: result
        @Suppress("UNCHECKED_CAST")
        val data = root["data"] as? Map<String, Any?> ?: return null
        @Suppress("UNCHECKED_CAST")
        val content = data["content"] as? Map<String, Any?> ?: return null
        @Suppress("UNCHECKED_CAST")
        val fields = content["fields"] as? Map<String, Any?> ?: return null
        @Suppress("UNCHECKED_CAST")
        val priceInfo = fields["price_info"] as? Map<String, Any?> ?: return null
        @Suppress("UNCHECKED_CAST")
        val priceInfoFields = priceInfo["fields"] as? Map<String, Any?> ?: return null
        return priceInfoFields["arrival_time"]?.toString()?.toLongOrNull()
    }
}
