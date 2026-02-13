package com.suisdks.sui.deepbook_v3

import com.suisdks.sui.transactions.Transaction
import com.suisdks.sui.transactions.pure
import com.suisdks.sui.transactions.object_

class FlashLoanContract(
    private val config: DeepBookConfig
) {
    private val packageId = config.packageIds.deepbookPackageId

    fun borrowBaseAsset(tx: Transaction, poolKey: String, borrowAmount: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val inputQuantity = (borrowAmount * baseCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::borrow_flashloan_base",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(inputQuantity)
            )
        )
    }

    fun returnBaseAsset(
        tx: Transaction,
        poolKey: String,
        borrowAmount: Double,
        baseCoinInput: Any,
        flashLoan: Any
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val borrowScalar = baseCoin.scalar
        
        val baseCoinReturn = tx.splitCoins(baseCoinInput, listOf(
            pure((borrowAmount * borrowScalar).toULong())
        ))
        
        tx.moveCall(
            target = "$packageId::pool::return_flashloan_base",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                baseCoinReturn,
                flashLoan
            )
        )
    }

    fun borrowQuoteAsset(tx: Transaction, poolKey: String, borrowAmount: Double) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val inputQuantity = (borrowAmount * quoteCoin.scalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::borrow_flashloan_quote",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                pure(inputQuantity)
            )
        )
    }

    fun returnQuoteAsset(
        tx: Transaction,
        poolKey: String,
        borrowAmount: Double,
        quoteCoinInput: Any,
        flashLoan: Any
    ) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        val borrowScalar = quoteCoin.scalar
        
        val quoteCoinReturn = tx.splitCoins(quoteCoinInput, listOf(
            pure((borrowAmount * borrowScalar).toULong())
        ))
        
        tx.moveCall(
            target = "$packageId::pool::return_flashloan_quote",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                quoteCoinReturn,
                flashLoan
            )
        )
    }
}
