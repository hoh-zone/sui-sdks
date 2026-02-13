package com.suisdks.sui.deepbook_v3

import com.suisdks.sui.transactions.Transaction
import com.suisdks.sui.transactions.pure
import com.suisdks.sui.transactions.object_

class DeepBookAdminContract(
    private val config: DeepBookConfig
) {
    private val packageId = config.packageIds.deepbookPackageId

    fun createPoolAdmin(tx: Transaction) {
        tx.moveCall(
            target = "$packageId::pool_admin::new",
            arguments = listOf(object_(config.packageIds.registryId))
        )
    }

    fun unregisterPoolAdmin(tx: Transaction, poolAdminCap: String) {
        tx.moveCall(
            target = "$packageId::pool_admin::unregister",
            arguments = listOf(object_(poolAdminCap))
        )
    }

    fun createPermissionlessPool(tx: Transaction, params: CreatePermissionlessPoolParams) {
        val baseCoin = config.getCoin(params.baseCoinKey)
        val quoteCoin = config.getCoin(params.quoteCoinKey)
        val deepCoinType = config.getCoin("DEEP").type
        
        val baseScalar = baseCoin.scalar
        val quoteScalar = quoteCoin.scalar
        
        val adjustedTickSize = ((params.tickSize * FLOAT_SCALAR * quoteScalar) / baseScalar).toULong()
        val adjustedLotSize = (params.lotSize * baseScalar).toULong()
        val adjustedMinSize = (params.minSize * baseScalar).toULong()
        
        tx.moveCall(
            target = "$packageId::pool::create_permissionless_pool",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(config.packageIds.registryId),
                pure(adjustedTickSize),
                pure(adjustedLotSize),
                pure(adjustedMinSize),
                object_(params.deepCoin ?: deepCoinType)
            )
        )
    }

    fun registerDeepbookPool(tx: Transaction, poolKey: String, poolAdminCap: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool_admin::register_pool",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(config.packageIds.registryId),
                object_(pool.address),
                object_(poolAdminCap)
            )
        )
    }

    fun enableDeepbookPool(tx: Transaction, poolKey: String, poolAdminCap: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool_admin::enable_pool",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(poolAdminCap)
            )
        )
    }

    fun disableDeepbookPool(tx: Transaction, poolKey: String, poolAdminCap: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool_admin::disable_pool",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(poolAdminCap)
            )
        )
    }

    fun enableDeepbookPoolForLoan(tx: Transaction, poolKey: String, poolAdminCap: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool_admin::enable_pool_for_loan",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(poolAdminCap)
            )
        )
    }

    fun disableDeepbookPoolForLoan(tx: Transaction, poolKey: String, poolAdminCap: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool_admin::disable_pool_for_loan",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(poolAdminCap)
            )
        )
    }

    fun allowedPauseCaps(tx: Transaction, adminCap: String) {
        tx.moveCall(
            target = "$packageId::pool_admin::allowed_pause_caps",
            arguments = listOf(object_(adminCap))
        )
    }

    fun mintPauseCap(tx: Transaction, poolKey: String, adminCap: String) {
        val pool = config.getPool(poolKey)
        val baseCoin = config.getCoin(pool.baseCoin)
        val quoteCoin = config.getCoin(pool.quoteCoin)
        
        tx.moveCall(
            target = "$packageId::pool_admin::mint_pause_cap",
            typeArguments = listOf(baseCoin.type, quoteCoin.type),
            arguments = listOf(
                object_(pool.address),
                object_(adminCap)
            )
        )
    }

    fun revokePauseCap(tx: Transaction, pauseCap: String, adminCap: String) {
        tx.moveCall(
            target = "$packageId::pool_admin::revoke_pause_cap",
            arguments = listOf(
                object_(pauseCap),
                object_(adminCap)
            )
        )
    }

    fun disableVersion(tx: Transaction, version: Long, adminCap: String) {
        tx.moveCall(
            target = "$packageId::registry::disable_version",
            arguments = listOf(
                object_(config.packageIds.registryId),
                pure(version.toULong()),
                object_(adminCap)
            )
        )
    }

    fun enableVersion(tx: Transaction, version: Long, adminCap: String) {
        tx.moveCall(
            target = "$packageId::registry::enable_version",
            arguments = listOf(
                object_(config.packageIds.registryId),
                pure(version.toULong()),
                object_(adminCap)
            )
        )
    }
}

data class CreatePermissionlessPoolParams(
    val baseCoinKey: String,
    val quoteCoinKey: String,
    val tickSize: Double,
    val lotSize: Double,
    val minSize: Double,
    val deepCoin: String? = null
)

const val POOL_CREATION_FEE_DEEP = 100_000_000L
