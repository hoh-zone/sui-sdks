package com.suisdks.sui.deepbook_v3

import com.suisdks.sui.transactions.Transaction
import com.suisdks.sui.transactions.pure
import com.suisdks.sui.transactions.object_

class BalanceManagerContract(
    private val config: DeepBookConfig
) {
    private val packageId = config.packageIds.deepbookPackageId

    fun createAndShareBalanceManager(tx: Transaction) {
        val manager = tx.moveCall(
            target = "$packageId::balance_manager::new"
        )
        
        tx.moveCall(
            target = "0x2::transfer::public_share_object",
            typeArguments = listOf("$packageId::balance_manager::BalanceManager"),
            arguments = listOf(manager)
        )
    }

    fun createBalanceManagerWithOwner(tx: Transaction, ownerAddress: String) {
        tx.moveCall(
            target = "$packageId::balance_manager::new_with_custom_owner",
            arguments = listOf(pure(ownerAddress))
        )
    }

    fun shareBalanceManager(tx: Transaction, manager: Any) {
        tx.moveCall(
            target = "0x2::transfer::public_share_object",
            typeArguments = listOf("$packageId::balance_manager::BalanceManager"),
            arguments = listOf(manager)
        )
    }

    fun depositIntoManager(
        tx: Transaction,
        managerKey: String,
        coinKey: String,
        amountToDeposit: Double
    ) {
        val managerId = config.getBalanceManager(managerKey).address
        val coin = config.getCoin(coinKey)
        val depositInput = (amountToDeposit * coin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::balance_manager::deposit",
            typeArguments = listOf(coin.type),
            arguments = listOf(
                object_(managerId),
                pure(depositInput)
            )
        )
    }

    fun withdrawFromManager(
        tx: Transaction,
        managerKey: String,
        coinKey: String,
        amountToWithdraw: Double,
        recipient: String
    ) {
        val managerId = config.getBalanceManager(managerKey).address
        val coin = config.getCoin(coinKey)
        val withdrawInput = (amountToWithdraw * coin.scalar).toULong()
        
        val coinObject = tx.moveCall(
            target = "$packageId::balance_manager::withdraw",
            typeArguments = listOf(coin.type),
            arguments = listOf(
                object_(managerId),
                pure(withdrawInput)
            )
        )
        
        tx.transferObjects(listOf(coinObject), recipient)
    }

    fun withdrawAllFromManager(
        tx: Transaction,
        managerKey: String,
        coinKey: String,
        recipient: String
    ) {
        val managerId = config.getBalanceManager(managerKey).address
        val coin = config.getCoin(coinKey)
        
        val withdrawalCoin = tx.moveCall(
            target = "$packageId::balance_manager::withdraw_all",
            typeArguments = listOf(coin.type),
            arguments = listOf(object_(managerId))
        )
        
        tx.transferObjects(listOf(withdrawalCoin), recipient)
    }

    fun checkManagerBalance(tx: Transaction, managerKey: String, coinKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        val coin = config.getCoin(coinKey)
        
        tx.moveCall(
            target = "$packageId::balance_manager::balance",
            typeArguments = listOf(coin.type),
            arguments = listOf(object_(managerId))
        )
    }

    fun generateProof(tx: Transaction, managerKey: String): ByteArray {
        val balanceManager = config.getBalanceManager(managerKey)
        return if (balanceManager.tradeCap != null) {
            generateProofAsTrader(tx, balanceManager.address, balanceManager.tradeCap)
        } else {
            generateProofAsOwner(tx, balanceManager.address)
        }
    }

    fun generateProofAsOwner(tx: Transaction, managerId: String): ByteArray {
        tx.moveCall(
            target = "$packageId::balance_manager::generate_proof_as_owner",
            arguments = listOf(object_(managerId))
        )
        return managerId.toByteArray()
    }

    fun generateProofAsTrader(tx: Transaction, managerId: String, tradeCapId: String): ByteArray {
        tx.moveCall(
            target = "$packageId::balance_manager::generate_proof_as_trader",
            arguments = listOf(
                object_(managerId),
                object_(tradeCapId)
            )
        )
        return managerId.toByteArray()
    }

    fun mintTradeCap(tx: Transaction, managerKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::mint_trade_cap",
            arguments = listOf(object_(managerId))
        )
    }

    fun mintDepositCap(tx: Transaction, managerKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::mint_deposit_cap",
            arguments = listOf(object_(managerId))
        )
    }

    fun mintWithdrawalCap(tx: Transaction, managerKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::mint_withdraw_cap",
            arguments = listOf(object_(managerId))
        )
    }

    fun depositWithCap(
        tx: Transaction,
        managerKey: String,
        coinKey: String,
        amountToDeposit: Double
    ) {
        val manager = config.getBalanceManager(managerKey)
        val managerId = manager.address
        val depositCapId = manager.depositCap 
            ?: throw NoSuchElementException("DepositCap not set for $managerKey")
        val coin = config.getCoin(coinKey)
        val depositInput = (amountToDeposit * coin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::balance_manager::deposit_with_cap",
            typeArguments = listOf(coin.type),
            arguments = listOf(
                object_(managerId),
                object_(depositCapId),
                pure(depositInput)
            )
        )
    }

    fun withdrawWithCap(
        tx: Transaction,
        managerKey: String,
        coinKey: String,
        amountToWithdraw: Double
    ) {
        val manager = config.getBalanceManager(managerKey)
        val managerId = manager.address
        val withdrawCapId = manager.withdrawCap 
            ?: throw NoSuchElementException("WithdrawCap not set for $managerKey")
        val coin = config.getCoin(coinKey)
        val withdrawAmount = (amountToWithdraw * coin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::balance_manager::withdraw_with_cap",
            typeArguments = listOf(coin.type),
            arguments = listOf(
                object_(managerId),
                object_(withdrawCapId),
                pure(withdrawAmount)
            )
        )
    }

    fun setBalanceManagerReferral(
        tx: Transaction,
        managerKey: String,
        referral: String,
        tradeCap: Any
    ) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::set_balance_manager_referral",
            arguments = listOf(
                object_(managerId),
                object_(referral),
                tradeCap
            )
        )
    }

    fun unsetBalanceManagerReferral(
        tx: Transaction,
        managerKey: String,
        poolKey: String,
        tradeCap: Any
    ) {
        val managerId = config.getBalanceManager(managerKey).address
        val poolId = config.getPool(poolKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::unset_balance_manager_referral",
            arguments = listOf(
                object_(managerId),
                pure(poolId),
                tradeCap
            )
        )
    }

    fun registerBalanceManager(tx: Transaction, managerKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::register_balance_manager",
            arguments = listOf(
                object_(managerId),
                object_(config.packageIds.registryId)
            )
        )
    }

    fun owner(tx: Transaction, managerKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::owner",
            arguments = listOf(object_(managerId))
        )
    }

    fun id(tx: Transaction, managerKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::id",
            arguments = listOf(object_(managerId))
        )
    }

    fun balanceManagerReferralOwner(tx: Transaction, referralId: String) {
        tx.moveCall(
            target = "$packageId::balance_manager::balance_manager_referral_owner",
            arguments = listOf(object_(referralId))
        )
    }

    fun balanceManagerReferralPoolId(tx: Transaction, referralId: String) {
        tx.moveCall(
            target = "$packageId::balance_manager::balance_manager_referral_pool_id",
            arguments = listOf(object_(referralId))
        )
    }

    fun getBalanceManagerReferralId(tx: Transaction, managerKey: String, poolKey: String) {
        val managerId = config.getBalanceManager(managerKey).address
        val poolId = config.getPool(poolKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::get_balance_manager_referral_id",
            arguments = listOf(
                object_(managerId),
                pure(poolId)
            )
        )
    }

    fun revokeTradeCap(tx: Transaction, managerKey: String, tradeCapId: String) {
        val managerId = config.getBalanceManager(managerKey).address
        
        tx.moveCall(
            target = "$packageId::balance_manager::revoke_trade_cap",
            arguments = listOf(
                object_(managerId),
                pure(tradeCapId)
            )
        )
    }
}
