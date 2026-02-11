package com.suisdks.sui.client

import com.suisdks.sui.jsonrpc.JsonRpcClient
import com.suisdks.sui.pagination.iterPaginatedItems
import com.suisdks.sui.transactions.Transaction
import java.util.Base64
import java.util.concurrent.CancellationException

private const val DEFAULT_COIN_TYPE = "0x2::sui::SUI"

class SuiClient(private val rpc: JsonRpcClient) {
    companion object {
        fun fromNetwork(
            network: String = "testnet",
            timeoutMs: Int = 30_000,
            headers: Map<String, String> = emptyMap(),
        ): SuiClient = SuiClient(JsonRpcClient.fromNetwork(network, timeoutMs, headers))

        fun fromEndpoint(
            endpoint: String,
            timeoutMs: Int = 30_000,
            headers: Map<String, String> = emptyMap(),
        ): SuiClient = SuiClient(JsonRpcClient.fromEndpoint(endpoint, timeoutMs, headers))
    }

    fun execute(method: String, params: List<Any?> = emptyList()): Map<String, Any?> = rpc.call(method, params)

    fun rpc(): JsonRpcClient = rpc

    fun discoverRpcApi(): Map<String, Any?> = mapResult("rpc.discover")

    fun getRpcApiVersion(): String? {
        val discover = discoverRpcApi()
        return discover["info"]?.let { info ->
            @Suppress("UNCHECKED_CAST")
            (info as? Map<String, Any?>)?.get("version")?.toString()
        }
    }

    fun dryRun(txBytesB64: String): Map<String, Any?> = mapResult("sui_dryRunTransactionBlock", txBytesB64)

    fun dryRunTransactionBlock(txBytesB64: String): Map<String, Any?> = dryRun(txBytesB64)

    fun devInspectTransactionBlock(sender: String, txBytesB64: String): Map<String, Any?> =
        mapResult("sui_devInspectTransactionBlock", sender, txBytesB64)

    fun getObject(objectId: String, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        mapResult("sui_getObject", objectId, options)

    fun getObjects(objectIds: List<String>, options: Map<String, Any?> = emptyMap()): List<Map<String, Any?>> =
        objectIds.map { getObject(it, options) }

    fun multiGetObjects(objectIds: List<String>, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        mapResult("sui_multiGetObjects", requireNoDuplicates(objectIds, "objectIds"), options)

    fun getEvents(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int? = null,
        descendingOrder: Boolean = false,
    ): Map<String, Any?> = mapResult("suix_queryEvents", query, cursor, limit, descendingOrder)

    fun iterEvents(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int = 100,
        descendingOrder: Boolean = false,
        maxItems: Int? = null,
    ): Sequence<Map<String, Any?>> = iterPaginatedItems(
        fetchPage = { c -> getEvents(query = query, cursor = c, limit = limit, descendingOrder = descendingOrder) },
        startCursor = cursor,
        maxItems = maxItems,
    )

    fun getPackage(packageId: String): Map<String, Any?> = getObject(
        packageId,
        mapOf(
            "showType" to true,
            "showOwner" to true,
            "showPreviousTransaction" to true,
            "showDisplay" to false,
            "showContent" to true,
            "showBcs" to true,
            "showStorageRebate" to true,
        ),
    )

    fun getGas(owner: String, coinType: String = DEFAULT_COIN_TYPE, cursor: String? = null, limit: Int? = null): Map<String, Any?> =
        mapResult("suix_getCoins", owner, coinType, cursor, limit)

    fun getAllCoins(owner: String, cursor: String? = null, limit: Int? = null): Map<String, Any?> =
        mapResult("suix_getAllCoins", owner, cursor, limit)

    fun iterAllCoins(
        owner: String,
        cursor: String? = null,
        limit: Int = 100,
        maxItems: Int? = null,
    ): Sequence<Map<String, Any?>> = iterPaginatedItems(
        fetchPage = { c -> getAllCoins(owner = owner, cursor = c, limit = limit) },
        startCursor = cursor,
        maxItems = maxItems,
    )

    fun getBalance(owner: String, coinType: String = DEFAULT_COIN_TYPE): Map<String, Any?> =
        mapResult("suix_getBalance", owner, coinType)

    fun getAllBalances(owner: String): Map<String, Any?> = mapResult("suix_getAllBalances", owner)

    fun getCoinMetadata(coinType: String): Map<String, Any?> = mapResult("suix_getCoinMetadata", coinType)

    fun getTotalSupply(coinType: String): Map<String, Any?> = mapResult("suix_getTotalSupply", coinType)

    fun getOwnedObjects(
        owner: String,
        query: Map<String, Any?> = emptyMap(),
        cursor: String? = null,
        limit: Int? = null,
    ): Map<String, Any?> = mapResult("suix_getOwnedObjects", owner, query, cursor, limit)

    fun getOwnedObjectsLegacy(
        owner: String,
        query: Map<String, Any?> = emptyMap(),
        cursor: String? = null,
        limit: Int? = null,
    ): Map<String, Any?> = mapResult("sui_getOwnedObjects", owner, query, cursor, limit)

    fun iterOwnedObjects(
        owner: String,
        query: Map<String, Any?> = emptyMap(),
        cursor: String? = null,
        limit: Int = 100,
        maxItems: Int? = null,
    ): Sequence<Map<String, Any?>> = iterPaginatedItems(
        fetchPage = { c -> getOwnedObjects(owner = owner, query = query, cursor = c, limit = limit) },
        startCursor = cursor,
        maxItems = maxItems,
    )

    fun getDynamicFields(parentObjectId: String, cursor: String? = null, limit: Int? = null): Map<String, Any?> =
        mapResult("suix_getDynamicFields", parentObjectId, cursor, limit)

    fun iterDynamicFields(
        parentObjectId: String,
        cursor: String? = null,
        limit: Int = 100,
        maxItems: Int? = null,
    ): Sequence<Map<String, Any?>> = iterPaginatedItems(
        fetchPage = { c -> getDynamicFields(parentObjectId = parentObjectId, cursor = c, limit = limit) },
        startCursor = cursor,
        maxItems = maxItems,
    )

    fun getDynamicFieldObject(parentObjectId: String, name: Map<String, Any?>): Map<String, Any?> =
        mapResult("suix_getDynamicFieldObject", parentObjectId, name)

    fun getLatestSuiSystemState(): Map<String, Any?> = mapResult("suix_getLatestSuiSystemState")

    fun getReferenceGasPrice(): Map<String, Any?> = mapResult("suix_getReferenceGasPrice")

    fun verifyZkLoginSignature(
        bytes: String,
        signature: String,
        intentScope: Int,
        author: String,
    ): Map<String, Any?> = mapResult("sui_verifyZkLoginSignature", bytes, signature, intentScope, author)

    fun getLatestCheckpointSequenceNumber(): Any? = result("sui_getLatestCheckpointSequenceNumber")

    fun queryTransactionBlocks(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int? = null,
        descendingOrder: Boolean = false,
    ): Map<String, Any?> = mapResult("suix_queryTransactionBlocks", query, cursor, limit, descendingOrder)

    fun iterTransactionBlocks(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int = 100,
        descendingOrder: Boolean = false,
        maxItems: Int? = null,
    ): Sequence<Map<String, Any?>> = iterPaginatedItems(
        fetchPage = { c -> queryTransactionBlocks(query = query, cursor = c, limit = limit, descendingOrder = descendingOrder) },
        startCursor = cursor,
        maxItems = maxItems,
    )

    fun getTransactionBlock(digest: String, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        mapResult("sui_getTransactionBlock", digest, options)

    fun getTotalTransactionBlocks(): Any? = result("sui_getTotalTransactionBlocks")

    fun multiGetTransactionBlocks(digests: List<String>, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        mapResult("sui_multiGetTransactionBlocks", requireNoDuplicates(digests, "digests"), options)

    fun getEventsByTransaction(transactionDigest: String): Map<String, Any?> =
        mapResult("sui_getEvents", transactionDigest)

    fun getCheckpoint(checkpointId: String): Map<String, Any?> = mapResult("sui_getCheckpoint", checkpointId)

    fun getCheckpoints(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false): Map<String, Any?> =
        mapResult("sui_getCheckpoints", cursor, limit, descendingOrder)

    fun iterCheckpoints(
        cursor: String? = null,
        limit: Int = 100,
        descendingOrder: Boolean = false,
        maxItems: Int? = null,
    ): Sequence<Map<String, Any?>> = iterPaginatedItems(
        fetchPage = { c -> getCheckpoints(cursor = c, limit = limit, descendingOrder = descendingOrder) },
        startCursor = cursor,
        maxItems = maxItems,
    )

    fun getCommitteeInfo(epoch: String? = null): Map<String, Any?> = mapResult("suix_getCommitteeInfo", epoch)

    fun getProtocolConfig(version: String? = null): Map<String, Any?> = mapResult("sui_getProtocolConfig", version)

    fun getNetworkMetrics(): Map<String, Any?> = mapResult("suix_getNetworkMetrics")

    fun getAddressMetrics(): Map<String, Any?> = mapResult("suix_getLatestAddressMetrics")

    fun getEpochMetrics(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false): Map<String, Any?> =
        mapResult("suix_getEpochMetrics", cursor, limit, descendingOrder)

    fun getAllEpochAddressMetrics(descendingOrder: Boolean = false): Map<String, Any?> =
        mapResult("suix_getAllEpochAddressMetrics", descendingOrder)

    fun getEpochs(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false): Map<String, Any?> =
        mapResult("suix_getEpochs", cursor, limit, descendingOrder)

    fun getMoveCallMetrics(): Map<String, Any?> = mapResult("suix_getMoveCallMetrics")

    fun getCurrentEpoch(): Map<String, Any?> = mapResult("suix_getCurrentEpoch")

    fun getChainIdentifier(): Any? = result("sui_getChainIdentifier")

    fun resolveNameServiceAddress(name: String): Map<String, Any?> = mapResult("suix_resolveNameServiceAddress", name)

    fun resolveNameServiceNames(address: String, cursor: String? = null, limit: Int? = null): Map<String, Any?> =
        mapResult("suix_resolveNameServiceNames", address, cursor, limit)

    fun getValidatorsApy(): Map<String, Any?> = mapResult("suix_getValidatorsApy")

    fun getStakes(owner: String): Map<String, Any?> = mapResult("suix_getStakes", owner)

    fun getStakesByIds(stakedSuiIds: List<String>): Map<String, Any?> = mapResult("suix_getStakesByIds", stakedSuiIds)

    fun tryGetPastObject(objectId: String, version: Int, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        mapResult("sui_tryGetPastObject", objectId, version, options)

    fun tryMultiGetPastObjects(pastObjects: List<Map<String, Any?>>, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        mapResult("sui_tryMultiGetPastObjects", pastObjects, options)

    fun getNormalizedMoveModulesByPackage(packageId: String): Map<String, Any?> =
        mapResult("sui_getNormalizedMoveModulesByPackage", packageId)

    fun getNormalizedMoveModule(packageId: String, moduleName: String): Map<String, Any?> =
        mapResult("sui_getNormalizedMoveModule", packageId, moduleName)

    fun getNormalizedMoveFunction(packageId: String, moduleName: String, functionName: String): Map<String, Any?> =
        mapResult("sui_getNormalizedMoveFunction", packageId, moduleName, functionName)

    fun getMoveFunctionArgTypes(packageId: String, moduleName: String, functionName: String): Map<String, Any?> =
        mapResult("sui_getMoveFunctionArgTypes", packageId, moduleName, functionName)

    fun getNormalizedMoveStruct(packageId: String, moduleName: String, structName: String): Map<String, Any?> =
        mapResult("sui_getNormalizedMoveStruct", packageId, moduleName, structName)

    fun executeTransactionBlock(
        transactionBlock: String,
        signature: String,
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> = mapResult("sui_executeTransactionBlock", transactionBlock, listOf(signature), options)

    fun executeTransactionBlock(
        transactionBlock: ByteArray,
        signature: String,
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> = executeTransactionBlock(
        transactionBlock = Base64.getEncoder().encodeToString(transactionBlock),
        signature = signature,
        options = options,
    )

    fun executeTransactionBlock(
        transactionBlock: String,
        signatures: List<String>,
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> = mapResult("sui_executeTransactionBlock", transactionBlock, signatures, options)

    fun executeTransactionBlock(
        transactionBlock: ByteArray,
        signatures: List<String>,
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> = executeTransactionBlock(
        transactionBlock = Base64.getEncoder().encodeToString(transactionBlock),
        signatures = signatures,
        options = options,
    )

    fun signAndExecuteTransaction(
        transaction: Transaction,
        signer: TransactionSigner,
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> {
        transaction.setSenderIfNotSet(signer.toSuiAddress())
        val txBytes = transaction.build()
        val signed = signer.signTransaction(txBytes)
        return executeTransactionBlock(
            transactionBlock = Base64.getEncoder().encodeToString(signed.bytes),
            signature = signed.signature,
            options = options,
        )
    }

    fun signAndExecuteTransaction(
        transactionBytes: ByteArray,
        signer: TransactionSigner,
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> {
        val signed = signer.signTransaction(transactionBytes)
        return executeTransactionBlock(
            transactionBlock = signed.bytes,
            signature = signed.signature,
            options = options,
        )
    }

    fun waitForTransaction(
        digest: String,
        options: Map<String, Any?> = emptyMap(),
        timeoutMs: Long = 60_000,
        pollIntervalMs: Long = 2_000,
        shouldCancel: (() -> Boolean)? = null,
    ): Map<String, Any?> {
        require(timeoutMs > 0) { "timeoutMs must be positive" }
        require(pollIntervalMs > 0) { "pollIntervalMs must be positive" }

        val deadline = System.currentTimeMillis() + timeoutMs
        var lastError: Throwable? = null
        while (System.currentTimeMillis() < deadline) {
            if (shouldCancel?.invoke() == true) {
                throw CancellationException("waitForTransaction cancelled for digest=$digest")
            }
            try {
                return getTransactionBlock(digest, options)
            } catch (e: Throwable) {
                lastError = e
                Thread.sleep(pollIntervalMs)
            }
        }
        throw IllegalStateException("waitForTransaction timed out for digest=$digest", lastError)
    }

    fun close() {
        // JsonRpcClient uses per-request connection and keeps no persistent session.
    }

    private fun result(method: String, vararg params: Any?): Any? {
        return execute(method, params.toList())["result"]
    }

    private fun mapResult(method: String, vararg params: Any?): Map<String, Any?> {
        val result = result(method, *params)
        @Suppress("UNCHECKED_CAST")
        return result as? Map<String, Any?> ?: mapOf("result" to result)
    }

    private fun <T> requireNoDuplicates(values: List<T>, field: String): List<T> {
        require(values.size == values.toSet().size) { "Duplicate values in $field: $values" }
        return values
    }
}

data class SignedTransaction(
    val signature: String,
    val bytes: ByteArray,
)

fun interface TransactionSigner {
    fun signTransaction(bytes: ByteArray): SignedTransaction
    fun toSuiAddress(): String = ""
}
