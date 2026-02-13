package com.suisdks.sui.deepbook_v3

import com.suisdks.sui.transactions.Transaction
import com.suisdks.sui.transactions.pure
import com.suisdks.sui.transactions.object_

class GovernanceContract(
    private val config: DeepBookConfig
) {
    private val packageId = config.packageIds.deepbookPackageId

    fun stake(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String,
        stakeAmount: Double
    ) {
        val pool = config.getPool(poolKey)
        val balanceManager = config.getBalanceManager(balanceManagerKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val stakeInput = (stakeAmount * DEEP_SCALAR).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::stake",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(balanceManager.address),
                pure(generateProof(tx, balanceManagerKey)),
                pure(stakeInput)
            )
        )
    }

    fun unstake(tx: Transaction, poolKey: String, balanceManagerKey: String) {
        val pool = config.getPool(poolKey)
        val balanceManager = config.getBalanceManager(balanceManagerKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::unstake",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(balanceManager.address),
                pure(generateProof(tx, balanceManagerKey))
            )
        )
    }

    fun submitProposal(tx: Transaction, params: ProposalParams) {
        val pool = config.getPool(params.poolKey)
        val balanceManager = config.getBalanceManager(params.balanceManagerKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::submit_proposal",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(balanceManager.address),
                pure(generateProof(tx, params.balanceManagerKey)),
                pure((params.takerFee * FLOAT_SCALAR).toULong()),
                pure((params.makerFee * FLOAT_SCALAR).toULong()),
                pure((params.stakeRequired * DEEP_SCALAR).toULong())
            )
        )
    }

    fun vote(
        tx: Transaction,
        poolKey: String,
        balanceManagerKey: String,
        proposalId: String
    ) {
        val pool = config.getPool(poolKey)
        val balanceManager = config.getBalanceManager(balanceManagerKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool::vote",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(balanceManager.address),
                pure(generateProof(tx, balanceManagerKey)),
                pure(proposalId)
            )
        )
    }

    private fun generateProof(tx: Transaction, managerKey: String): ByteArray {
        val manager = config.getBalanceManager(managerKey)
        return manager.address.toByteArray()
    }
}

data class ProposalParams(
    val poolKey: String,
    val balanceManagerKey: String,
    val takerFee: Double,
    val makerFee: Double,
    val stakeRequired: Double
)
