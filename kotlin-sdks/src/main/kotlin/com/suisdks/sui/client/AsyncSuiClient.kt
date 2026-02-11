package com.suisdks.sui.client

import com.suisdks.sui.jsonrpc.JsonRpcClient
import com.suisdks.sui.transactions.Transaction
import java.util.concurrent.CompletableFuture
import java.util.concurrent.Executor
import java.util.concurrent.ForkJoinPool

class AsyncSuiClient private constructor(
    private val client: SuiClient,
    private val executor: Executor,
) {
    companion object {
        fun fromNetwork(
            network: String = "testnet",
            timeoutMs: Int = 30_000,
            headers: Map<String, String> = emptyMap(),
            executor: Executor = ForkJoinPool.commonPool(),
        ): AsyncSuiClient {
            val rpc = JsonRpcClient.fromNetwork(network, timeoutMs, headers)
            return AsyncSuiClient(SuiClient(rpc), executor)
        }

        fun fromEndpoint(
            endpoint: String,
            timeoutMs: Int = 30_000,
            headers: Map<String, String> = emptyMap(),
            executor: Executor = ForkJoinPool.commonPool(),
        ): AsyncSuiClient {
            val rpc = JsonRpcClient.fromEndpoint(endpoint, timeoutMs, headers)
            return AsyncSuiClient(SuiClient(rpc), executor)
        }

        fun fromClient(client: SuiClient, executor: Executor = ForkJoinPool.commonPool()): AsyncSuiClient {
            return AsyncSuiClient(client, executor)
        }
    }

    fun execute(method: String, params: List<Any?> = emptyList()): CompletableFuture<Map<String, Any?>> =
        CompletableFuture.supplyAsync({ client.execute(method, params) }, executor)

    fun rpc() = client.rpc()

    fun discoverRpcApi() = async { client.discoverRpcApi() }

    fun getRpcApiVersion() = async { client.getRpcApiVersion() }

    fun dryRun(txBytesB64: String) = async { client.dryRun(txBytesB64) }

    fun dryRunTransactionBlock(txBytesB64: String) = async { client.dryRunTransactionBlock(txBytesB64) }

    fun devInspectTransactionBlock(sender: String, txBytesB64: String) = async {
        client.devInspectTransactionBlock(sender, txBytesB64)
    }

    fun getObject(objectId: String, options: Map<String, Any?> = emptyMap()) = async { client.getObject(objectId, options) }

    fun getObjects(objectIds: List<String>, options: Map<String, Any?> = emptyMap()) = async {
        client.getObjects(objectIds, options)
    }

    fun multiGetObjects(objectIds: List<String>, options: Map<String, Any?> = emptyMap()) = async {
        client.multiGetObjects(objectIds, options)
    }

    fun getEvents(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int? = null,
        descendingOrder: Boolean = false,
    ) = async { client.getEvents(query, cursor, limit, descendingOrder) }

    fun iterEvents(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int = 100,
        descendingOrder: Boolean = false,
        maxItems: Int? = null,
    ) = async { client.iterEvents(query, cursor, limit, descendingOrder, maxItems).toList() }

    fun getPackage(packageId: String) = async { client.getPackage(packageId) }

    fun getGas(owner: String, coinType: String = "0x2::sui::SUI", cursor: String? = null, limit: Int? = null) = async {
        client.getGas(owner, coinType, cursor, limit)
    }

    fun getAllCoins(owner: String, cursor: String? = null, limit: Int? = null) = async {
        client.getAllCoins(owner, cursor, limit)
    }

    fun iterAllCoins(owner: String, cursor: String? = null, limit: Int = 100, maxItems: Int? = null) = async {
        client.iterAllCoins(owner, cursor, limit, maxItems).toList()
    }

    fun getBalance(owner: String, coinType: String = "0x2::sui::SUI") = async { client.getBalance(owner, coinType) }

    fun getAllBalances(owner: String) = async { client.getAllBalances(owner) }

    fun getCoinMetadata(coinType: String) = async { client.getCoinMetadata(coinType) }

    fun getTotalSupply(coinType: String) = async { client.getTotalSupply(coinType) }

    fun getOwnedObjects(
        owner: String,
        query: Map<String, Any?> = emptyMap(),
        cursor: String? = null,
        limit: Int? = null,
    ) = async { client.getOwnedObjects(owner, query, cursor, limit) }

    fun getOwnedObjectsLegacy(
        owner: String,
        query: Map<String, Any?> = emptyMap(),
        cursor: String? = null,
        limit: Int? = null,
    ) = async { client.getOwnedObjectsLegacy(owner, query, cursor, limit) }

    fun iterOwnedObjects(
        owner: String,
        query: Map<String, Any?> = emptyMap(),
        cursor: String? = null,
        limit: Int = 100,
        maxItems: Int? = null,
    ) = async { client.iterOwnedObjects(owner, query, cursor, limit, maxItems).toList() }

    fun getDynamicFields(parentObjectId: String, cursor: String? = null, limit: Int? = null) = async {
        client.getDynamicFields(parentObjectId, cursor, limit)
    }

    fun iterDynamicFields(parentObjectId: String, cursor: String? = null, limit: Int = 100, maxItems: Int? = null) = async {
        client.iterDynamicFields(parentObjectId, cursor, limit, maxItems).toList()
    }

    fun getDynamicFieldObject(parentObjectId: String, name: Map<String, Any?>) = async {
        client.getDynamicFieldObject(parentObjectId, name)
    }

    fun getLatestSuiSystemState() = async { client.getLatestSuiSystemState() }

    fun getReferenceGasPrice() = async { client.getReferenceGasPrice() }

    fun verifyZkLoginSignature(
        bytes: String,
        signature: String,
        intentScope: Int,
        author: String,
    ) = async { client.verifyZkLoginSignature(bytes, signature, intentScope, author) }

    fun getLatestCheckpointSequenceNumber() = async { client.getLatestCheckpointSequenceNumber() }

    fun queryTransactionBlocks(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int? = null,
        descendingOrder: Boolean = false,
    ) = async { client.queryTransactionBlocks(query, cursor, limit, descendingOrder) }

    fun iterTransactionBlocks(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int = 100,
        descendingOrder: Boolean = false,
        maxItems: Int? = null,
    ) = async { client.iterTransactionBlocks(query, cursor, limit, descendingOrder, maxItems).toList() }

    fun getTransactionBlock(digest: String, options: Map<String, Any?> = emptyMap()) = async {
        client.getTransactionBlock(digest, options)
    }

    fun getTotalTransactionBlocks() = async { client.getTotalTransactionBlocks() }

    fun multiGetTransactionBlocks(digests: List<String>, options: Map<String, Any?> = emptyMap()) = async {
        client.multiGetTransactionBlocks(digests, options)
    }

    fun getEventsByTransaction(transactionDigest: String) = async { client.getEventsByTransaction(transactionDigest) }

    fun getCheckpoint(checkpointId: String) = async { client.getCheckpoint(checkpointId) }

    fun getCheckpoints(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false) = async {
        client.getCheckpoints(cursor, limit, descendingOrder)
    }

    fun iterCheckpoints(
        cursor: String? = null,
        limit: Int = 100,
        descendingOrder: Boolean = false,
        maxItems: Int? = null,
    ) = async { client.iterCheckpoints(cursor, limit, descendingOrder, maxItems).toList() }

    fun getCommitteeInfo(epoch: String? = null) = async { client.getCommitteeInfo(epoch) }

    fun getProtocolConfig(version: String? = null) = async { client.getProtocolConfig(version) }

    fun getNetworkMetrics() = async { client.getNetworkMetrics() }

    fun getAddressMetrics() = async { client.getAddressMetrics() }

    fun getEpochMetrics(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false) = async {
        client.getEpochMetrics(cursor, limit, descendingOrder)
    }

    fun getAllEpochAddressMetrics(descendingOrder: Boolean = false) = async {
        client.getAllEpochAddressMetrics(descendingOrder)
    }

    fun getEpochs(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false) = async {
        client.getEpochs(cursor, limit, descendingOrder)
    }

    fun getMoveCallMetrics() = async { client.getMoveCallMetrics() }

    fun getCurrentEpoch() = async { client.getCurrentEpoch() }

    fun getChainIdentifier() = async { client.getChainIdentifier() }

    fun resolveNameServiceAddress(name: String) = async { client.resolveNameServiceAddress(name) }

    fun resolveNameServiceNames(address: String, cursor: String? = null, limit: Int? = null) = async {
        client.resolveNameServiceNames(address, cursor, limit)
    }

    fun getValidatorsApy() = async { client.getValidatorsApy() }

    fun getStakes(owner: String) = async { client.getStakes(owner) }

    fun getStakesByIds(stakedSuiIds: List<String>) = async { client.getStakesByIds(stakedSuiIds) }

    fun tryGetPastObject(objectId: String, version: Int, options: Map<String, Any?> = emptyMap()) = async {
        client.tryGetPastObject(objectId, version, options)
    }

    fun tryMultiGetPastObjects(pastObjects: List<Map<String, Any?>>, options: Map<String, Any?> = emptyMap()) = async {
        client.tryMultiGetPastObjects(pastObjects, options)
    }

    fun getNormalizedMoveModulesByPackage(packageId: String) = async { client.getNormalizedMoveModulesByPackage(packageId) }

    fun getNormalizedMoveModule(packageId: String, moduleName: String) = async {
        client.getNormalizedMoveModule(packageId, moduleName)
    }

    fun getNormalizedMoveFunction(packageId: String, moduleName: String, functionName: String) = async {
        client.getNormalizedMoveFunction(packageId, moduleName, functionName)
    }

    fun getMoveFunctionArgTypes(packageId: String, moduleName: String, functionName: String) = async {
        client.getMoveFunctionArgTypes(packageId, moduleName, functionName)
    }

    fun getNormalizedMoveStruct(packageId: String, moduleName: String, structName: String) = async {
        client.getNormalizedMoveStruct(packageId, moduleName, structName)
    }

    fun executeTransactionBlock(
        transactionBlock: String,
        signature: String,
        options: Map<String, Any?> = emptyMap(),
    ) = async { client.executeTransactionBlock(transactionBlock, signature, options) }

    fun executeTransactionBlock(
        transactionBlock: ByteArray,
        signature: String,
        options: Map<String, Any?> = emptyMap(),
    ) = async { client.executeTransactionBlock(transactionBlock, signature, options) }

    fun executeTransactionBlock(
        transactionBlock: String,
        signatures: List<String>,
        options: Map<String, Any?> = emptyMap(),
    ) = async { client.executeTransactionBlock(transactionBlock, signatures, options) }

    fun executeTransactionBlock(
        transactionBlock: ByteArray,
        signatures: List<String>,
        options: Map<String, Any?> = emptyMap(),
    ) = async { client.executeTransactionBlock(transactionBlock, signatures, options) }

    fun signAndExecuteTransaction(
        transaction: Transaction,
        signer: TransactionSigner,
        options: Map<String, Any?> = emptyMap(),
    ) = async { client.signAndExecuteTransaction(transaction, signer, options) }

    fun signAndExecuteTransaction(
        transactionBytes: ByteArray,
        signer: TransactionSigner,
        options: Map<String, Any?> = emptyMap(),
    ) = async { client.signAndExecuteTransaction(transactionBytes, signer, options) }

    fun waitForTransaction(
        digest: String,
        options: Map<String, Any?> = emptyMap(),
        timeoutMs: Long = 60_000,
        pollIntervalMs: Long = 2_000,
        shouldCancel: (() -> Boolean)? = null,
    ) = async { client.waitForTransaction(digest, options, timeoutMs, pollIntervalMs, shouldCancel) }

    fun close(): CompletableFuture<Unit> = async {
        client.close()
    }

    private fun <T> async(fn: () -> T): CompletableFuture<T> = CompletableFuture.supplyAsync(fn, executor)
}
