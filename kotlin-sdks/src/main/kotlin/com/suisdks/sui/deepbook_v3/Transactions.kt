package com.suisdks.sui.deepbook_v3

import com.suisdks.sui.transactions.Transaction

@Deprecated(
    message = "Use DeepBookContract, BalanceManagerContract, FlashLoanContract, GovernanceContract instead",
    replaceWith = ReplaceWith("DeepBookContract(config)")
)
class DeepBookTransactionBuilder(
    private val config: DeepBookConfig
) {
    private val deepBookContract = DeepBookContract(config)
    private val balanceManagerContract = BalanceManagerContract(config)

    fun placeLimitOrder(tx: Transaction, params: LimitOrderParams) {
        deepBookContract.placeLimitOrder(tx, params)
    }

    fun placeMarketOrder(tx: Transaction, params: MarketOrderParams) {
        deepBookContract.placeMarketOrder(tx, params)
    }

    fun cancelOrder(tx: Transaction, poolKey: String, balanceManagerKey: String, orderId: String) {
        deepBookContract.cancelOrder(tx, poolKey, balanceManagerKey, orderId)
    }

    fun depositIntoManager(tx: Transaction, managerKey: String, coinKey: String, amount: Double) {
        balanceManagerContract.depositIntoManager(tx, managerKey, coinKey, amount)
    }

    fun withdrawFromManager(
        tx: Transaction,
        managerKey: String,
        coinKey: String,
        amount: Double,
        recipient: String = config.address
    ) {
        balanceManagerContract.withdrawFromManager(tx, managerKey, coinKey, amount, recipient)
    }

    companion object {
        const val FLOAT_SCALAR = 1_000_000_000.0
        const val GAS_BUDGET = 10_000_000L
    }
}

@Deprecated(
    message = "Use MarginContract from margin package instead",
    replaceWith = ReplaceWith("MarginContract")
)
class MarginTransactionBuilder(
    private val config: DeepBookConfig
) {
    fun openMarginPosition(
        tx: Transaction,
        poolKey: String,
        managerKey: String,
        collateralAmount: Double,
        borrowAmount: Double
    ) {
    }

    fun closeMarginPosition(tx: Transaction, poolKey: String, managerKey: String) {
    }

    fun addCollateral(tx: Transaction, poolKey: String, managerKey: String, amount: Double) {
    }
}
