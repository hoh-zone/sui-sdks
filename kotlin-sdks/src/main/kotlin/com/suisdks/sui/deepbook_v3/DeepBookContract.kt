package com.suisdks.sui.deepbook_v3

import com.suisdks.sui.transactions.Transaction
import com.suisdks.sui.transactions.pure
import com.suisdks.sui.transactions.object_

class DeepBookContract(
    private val config: DeepBookConfig
) {
    private val packageId = config.packageIds.deepbookPackageId

    fun placeLimitOrder(tx: Transaction, params: LimitOrderParams) {
        val pool = config.getPool(params.poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(params.balanceManagerKey)
        
        val price = (params.price * FLOAT_SCALAR * quoteCoin.scalar / baseCoin.scalar).toULong()
        val quantity = (params.quantity * baseCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::place_limit_order",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, params.balanceManagerKey)),
                pure(params.clientOrderId.toULong()),
                pure(params.orderType.toByte()),
                pure(params.selfMatchingOption.toByte()),
                pure(price),
                pure(quantity),
                pure(params.isBid),
                pure(params.payWithDeep),
                pure(params.expiration),
                object_("0x6")
            )
        )
    }

    fun placeMarketOrder(tx: Transaction, params: MarketOrderParams) {
        val pool = config.getPool(params.poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(params.balanceManagerKey)
        
        val quantity = (params.quantity * baseCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::place_market_order",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, params.balanceManagerKey)),
                pure(params.clientOrderId.toULong()),
                pure(params.selfMatchingOption.toByte()),
                pure(quantity),
                pure(params.isBid),
                pure(params.payWithDeep),
                object_("0x6")
            )
        )
    }

    fun modifyOrder(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String,
        orderId: String,
        newQuantity: Double
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(balanceManagerKey)
        val inputQuantity = (newQuantity * baseCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::modify_order",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, balanceManagerKey)),
                pure(orderId),
                pure(inputQuantity),
                object_("0x6")
            )
        )
    }

    fun cancelOrder(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String,
        orderId: String
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(balanceManagerKey)
        
        tx.moveCall(
            target = "$packageId::pool::cancel_order",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, balanceManagerKey)),
                pure(orderId),
                object_("0x6")
            )
        )
    }

    fun cancelOrders(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String,
        orderIds: List<String>
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(balanceManagerKey)
        
        tx.moveCall(
            target = "$packageId::pool::cancel_orders",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, balanceManagerKey)),
                pure(orderIds),
                object_("0x6")
            )
        )
    }

    fun cancelAllOrders(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(balanceManagerKey)
        
        tx.moveCall(
            target = "$packageId::pool::cancel_all_orders",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, balanceManagerKey)),
                object_("0x6")
            )
        )
    }

    fun withdrawSettledAmounts(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(balanceManagerKey)
        
        tx.moveCall(
            target = "$packageId::pool::withdraw_settled_amounts",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, balanceManagerKey))
            )
        )
    }

    fun withdrawSettledAmountsPermissionless(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(balanceManagerKey)
        
        tx.moveCall(
            target = "$packageId::pool::withdraw_settled_amounts_permissionless",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address)
            )
        )
    }

    fun getOrder(tx: Transaction, poolKey: String, orderId: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_order",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(orderId)
            )
        )
    }

    fun getOrders(tx: Transaction, poolKey: String, orderIds: List<String>) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_orders",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(orderIds)
            )
        )
    }

    fun accountOpenOrders(tx: Transaction, poolKey: String, managerKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(managerKey)
        
        tx.moveCall(
            target = "$packageId::pool::account_open_orders",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address)
            )
        )
    }

    fun account(tx: Transaction, poolKey: String, managerKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(managerKey)
        
        tx.moveCall(
            target = "$packageId::pool::account",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address)
            )
        )
    }

    fun accountExists(tx: Transaction, poolKey: String, managerKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(managerKey)
        
        tx.moveCall(
            target = "$packageId::pool::account_exists",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address)
            )
        )
    }

    fun vaultBalances(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::vault_balances",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun midPrice(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::mid_price",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_("0x6")
            )
        )
    }

    fun whitelisted(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::whitelisted",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun getQuoteQuantityOut(tx: Transaction, poolKey: String, baseQuantity: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_quote_quantity_out",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((baseQuantity * baseCoin.scalar).toULong()),
                object_("0x6")
            )
        )
    }

    fun getBaseQuantityOut(tx: Transaction, poolKey: String, quoteQuantity: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_base_quantity_out",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((quoteQuantity * quoteCoin.scalar).toULong()),
                object_("0x6")
            )
        )
    }

    fun getQuantityOut(tx: Transaction, poolKey: String, baseQuantity: Double, quoteQuantity: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_quantity_out",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((baseQuantity * baseCoin.scalar).toULong()),
                pure((quoteQuantity * quoteCoin.scalar).toULong()),
                object_("0x6")
            )
        )
    }

    fun getBaseQuantityIn(tx: Transaction, poolKey: String, targetQuoteQuantity: Double, payWithDeep: Boolean) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_base_quantity_in",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((targetQuoteQuantity * quoteCoin.scalar).toULong()),
                pure(payWithDeep),
                object_("0x6")
            )
        )
    }

    fun getQuoteQuantityIn(tx: Transaction, poolKey: String, targetBaseQuantity: Double, payWithDeep: Boolean) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_quote_quantity_in",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((targetBaseQuantity * baseCoin.scalar).toULong()),
                pure(payWithDeep),
                object_("0x6")
            )
        )
    }

    fun getQuoteQuantityOutInputFee(tx: Transaction, poolKey: String, baseQuantity: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_quote_quantity_out_input_fee",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((baseQuantity * baseCoin.scalar).toULong()),
                object_("0x6")
            )
        )
    }

    fun getBaseQuantityOutInputFee(tx: Transaction, poolKey: String, quoteQuantity: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_base_quantity_out_input_fee",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((quoteQuantity * quoteCoin.scalar).toULong()),
                object_("0x6")
            )
        )
    }

    fun getQuantityOutInputFee(tx: Transaction, poolKey: String, baseQuantity: Double, quoteQuantity: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_quantity_out_input_fee",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((baseQuantity * baseCoin.scalar).toULong()),
                pure((quoteQuantity * quoteCoin.scalar).toULong()),
                object_("0x6")
            )
        )
    }

    fun getLevel2Range(
        tx: Transaction,
        poolKey: String,
        priceLow: Double,
        priceHigh: Double,
        isBid: Boolean
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_level2_range",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(((priceLow * FLOAT_SCALAR * quoteCoin.scalar) / baseCoin.scalar).toULong()),
                pure(((priceHigh * FLOAT_SCALAR * quoteCoin.scalar) / baseCoin.scalar).toULong()),
                pure(isBid),
                object_("0x6")
            )
        )
    }

    fun getLevel2TicksFromMid(tx: Transaction, poolKey: String, tickFromMid: Long) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_level2_ticks_from_mid",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(tickFromMid.toULong()),
                object_("0x6")
            )
        )
    }

    fun burnDeep(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::burn_deep",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(config.packageIds.deepTreasuryId)
            )
        )
    }

    fun poolTradeParams(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::pool_trade_params",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun poolBookParams(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::pool_book_params",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun claimRebates(tx: Transaction, poolKey: String, balanceManagerKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(balanceManagerKey)
        
        tx.moveCall(
            target = "$packageId::pool::claim_rebates",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address),
                pure(generateProof(tx, balanceManagerKey))
            )
        )
    }

    fun addDeepPricePoint(tx: Transaction, targetPoolKey: String, referencePoolKey: String) {
        val targetPool = config.getPool(targetPoolKey)
        val referencePool = config.getPool(referencePoolKey)
        val targetBaseCoin = config.getCoin(targetPool.baseCoin)
        val targetQuoteCoin = config.getCoin(targetPool.quoteCoin)
        val referenceBaseCoin = config.getCoin(referencePool.baseCoin)
        val referenceQuoteCoin = config.getCoin(referencePool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::add_deep_price_point",
            typeArguments = listOf(
                targetBaseCoin.type,
                targetQuoteCoin.type,
                referenceBaseCoin.type,
                referenceQuoteCoin.type
            ),
            arguments = listOf(
                object_(targetPool.address),
                object_(referencePool.address),
                object_("0x6")
            )
        )
    }

    fun mintReferral(tx: Transaction, poolKey: String, multiplier: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val adjustedNumber = (multiplier * FLOAT_SCALAR).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::mint_referral",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(adjustedNumber)
            )
        )
    }

    fun updatePoolReferralMultiplier(tx: Transaction, poolKey: String, referral: String, multiplier: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val adjustedNumber = (multiplier * FLOAT_SCALAR).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::update_pool_referral_multiplier",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(referral),
                pure(adjustedNumber)
            )
        )
    }

    fun claimPoolReferralRewards(tx: Transaction, poolKey: String, referral: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::claim_pool_referral_rewards",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(referral)
            )
        )
    }

    fun updatePoolAllowedVersions(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::update_pool_allowed_versions",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(config.packageIds.registryId)
            )
        )
    }

    fun getPoolIdByAssets(tx: Transaction, baseType: String, quoteType: String) {
        tx.moveCall(
            target = "$packageId::pool::get_pool_id_by_asset",
            typeArguments = listOf(baseType, quoteType),
            arguments = listOf(object_(config.packageIds.registryId))
        )
    }

    fun getBalanceManagerIds(tx: Transaction, owner: String) {
        tx.moveCall(
            target = "$packageId::registry::get_balance_manager_ids",
            arguments = listOf(
                object_(config.packageIds.registryId),
                pure(owner)
            )
        )
    }

    fun getPoolReferralBalances(tx: Transaction, poolKey: String, referral: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_pool_referral_balances",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(referral)
            )
        )
    }

    fun poolReferralMultiplier(tx: Transaction, poolKey: String, referral: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::pool_referral_multiplier",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(referral)
            )
        )
    }

    fun stablePool(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::stable_pool",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun registeredPool(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::registered_pool",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun lockedBalance(tx: Transaction, poolKey: String, managerKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(managerKey)
        
        tx.moveCall(
            target = "$packageId::pool::locked_balance",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address)
            )
        )
    }

    fun getAccountOrderDetails(tx: Transaction, poolKey: String, managerKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val manager = config.getBalanceManager(managerKey)
        
        tx.moveCall(
            target = "$packageId::pool::get_account_order_details",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(manager.address)
            )
        )
    }

    fun getOrderDeepRequired(tx: Transaction, poolKey: String, baseQuantity: Double, price: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val inputPrice = ((price * FLOAT_SCALAR * quoteCoin.scalar) / baseCoin.scalar).toULong()
        val inputQuantity = (baseQuantity * baseCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::get_order_deep_required",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(inputQuantity),
                pure(inputPrice)
            )
        )
    }

    fun getPoolDeepPrice(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::get_order_deep_price",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun poolTradeParamsNext(tx: Transaction, poolKey: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::pool_trade_params_next",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(object_(pool.address))
        )
    }

    fun swapExactBaseForQuote(tx: Transaction, params: SwapParams) {
        val pool = config.getPool(params.poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val deepCoinType = config.getCoin("DEEP")
        
        val minQuoteInput = (params.minOut * quoteCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::swap_exact_base_for_quote",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((params.amount * baseCoin.scalar).toULong()),
                pure((params.deepAmount * DEEP_SCALAR).toULong()),
                pure(minQuoteInput),
                object_("0x6")
            )
        )
    }

    fun swapExactQuoteForBase(tx: Transaction, params: SwapParams) {
        val pool = config.getPool(params.poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val deepCoinType = config.getCoin("DEEP")
        
        val minBaseInput = (params.minOut * baseCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::swap_exact_quote_for_base",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure((params.amount * quoteCoin.scalar).toULong()),
                pure((params.deepAmount * DEEP_SCALAR).toULong()),
                pure(minBaseInput),
                object_("0x6")
            )
        )
    }

    fun swapExactQuantity(tx: Transaction, params: SwapParams, isBaseToCoin: Boolean) {
        val pool = config.getPool(params.poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        val minOutInput = if (isBaseToCoin) {
            (params.minOut * quoteCoin.scalar).toULong()
        } else {
            (params.minOut * baseCoin.scalar).toULong()
        }
        
        tx.moveCall(
            target = "$packageId::pool::swap_exact_quantity",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(if (isBaseToCoin) (params.amount * baseCoin.scalar).toULong() else 0uL),
                pure(if (!isBaseToCoin) (params.amount * quoteCoin.scalar).toULong() else 0uL),
                pure((params.deepAmount * DEEP_SCALAR).toULong()),
                pure(minOutInput),
                object_("0x6")
            )
        )
    }

    fun swapExactBaseForQuoteWithManager(tx: Transaction, params: SwapWithManagerParams) {
        val pool = config.getPool(params.poolKey)
        val balanceManager = config.getBalanceManager(params.balanceManagerKey)
        val baseCoinType = config.getCoin(pool.baseCoin)
        val quoteCoinType = config.getCoin(pool.quoteCoin)
        
        val minQuoteInput = (params.minOut * quoteCoinType.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::swap_exact_base_for_quote_with_manager",
            typeArguments = listOf(baseCoinType.type, quoteCoinType.type),
            arguments = listOf(
                object_(pool.address),
                object_(balanceManager.address),
                object_(params.tradeCap),
                object_(params.depositCap),
                object_(params.withdrawCap),
                pure((params.amount * baseCoinType.scalar).toULong()),
                pure(minQuoteInput),
                object_("0x6")
            )
        )
    }

    fun swapExactQuoteForBaseWithManager(tx: Transaction, params: SwapWithManagerParams) {
        val pool = config.getPool(params.poolKey)
        val balanceManager = config.getBalanceManager(params.balanceManagerKey)
        val baseCoinType = config.getCoin(pool.baseCoin)
        val quoteCoinType = config.getCoin(pool.quoteCoin)
        
        val minBaseInput = (params.minOut * baseCoinType.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::swap_exact_quote_for_base_with_manager",
            typeArguments = listOf(baseCoinType.type, quoteCoinType.type),
            arguments = listOf(
                object_(pool.address),
                object_(balanceManager.address),
                object_(params.tradeCap),
                object_(params.depositCap),
                object_(params.withdrawCap),
                pure((params.amount * quoteCoinType.scalar).toULong()),
                pure(minBaseInput),
                object_("0x6")
            )
        )
    }

    fun swapExactQuantityWithManager(tx: Transaction, params: SwapWithManagerParams, isBaseToCoin: Boolean) {
        val pool = config.getPool(params.poolKey)
        val balanceManager = config.getBalanceManager(params.balanceManagerKey)
        val baseCoinType = config.getCoin(pool.baseCoin)
        val quoteCoinType = config.getCoin(pool.quoteCoin)
        
        val minOutInput = if (isBaseToCoin) {
            (params.minOut * quoteCoinType.scalar).toULong()
        } else {
            (params.minOut * baseCoinType.scalar).toULong()
        }
        
        tx.moveCall(
            target = "$packageId::pool::swap_exact_quantity_with_manager",
            typeArguments = listOf(baseCoinType.type, quoteCoinType.type),
            arguments = listOf(
                object_(pool.address),
                object_(balanceManager.address),
                object_(params.tradeCap),
                object_(params.depositCap),
                object_(params.withdrawCap),
                pure(if (isBaseToCoin) (params.amount * baseCoinType.scalar).toULong() else 0uL),
                pure(if (!isBaseToCoin) (params.amount * quoteCoinType.scalar).toULong() else 0uL),
                pure(minOutInput),
                object_("0x6")
            )
        )
    }

    private fun generateProof(tx: Transaction, managerKey: String): ByteArray {
        val manager = config.getBalanceManager(managerKey)
        return manager.address.toByteArray()
    }

    companion object {
        const val GAS_BUDGET = 10_000_000L
    }
}

data class LimitOrderParams(
    val poolKey: String,
    val balanceManagerKey: String,
    val clientOrderId: String,
    val price: Double,
    val quantity: Double,
    val isBid: Boolean,
    val payWithDeep: Boolean = false,
    val orderType: Byte = 0,
    val selfMatchingOption: Byte = 0,
    val expiration: ULong = ULong.MAX_VALUE
)

data class MarketOrderParams(
    val poolKey: String,
    val balanceManagerKey: String,
    val clientOrderId: String,
    val quantity: Double,
    val isBid: Boolean,
    val payWithDeep: Boolean = false,
    val selfMatchingOption: Byte = 0
)

data class SwapParams(
    val poolKey: String,
    val amount: Double,
    val deepAmount: Double = 0.0,
    val minOut: Double = 0.0,
    val baseCoin: Any? = null,
    val quoteCoin: Any? = null,
    val deepCoin: Any? = null
)

data class SwapWithManagerParams(
    val poolKey: String,
    val balanceManagerKey: String,
    val tradeCap: String,
    val depositCap: String,
    val withdrawCap: String,
    val amount: Double,
    val minOut: Double,
    val baseCoin: Any? = null,
    val quoteCoin: Any? = null
)
