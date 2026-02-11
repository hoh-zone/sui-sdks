package com.suisdks.sui.grpc

import java.util.Base64

class SuiGrpcClient(private val transport: GrpcTransport) : AutoCloseable {
    companion object {
        fun fromOfficialGrpc(
            target: String,
            plaintext: Boolean = false,
            security: GrpcSecurityOptions = GrpcSecurityOptions(),
            tls: GrpcTlsOptions = GrpcTlsOptions(),
        ): SuiGrpcClient {
            return SuiGrpcClient(OfficialGrpcTransport(target, plaintext, security, tls))
        }
    }

    fun call(method: String, params: List<Any?> = emptyList(), metadata: Map<String, String> = emptyMap()): Any? {
        val response = transport.unary(GrpcRequest(method = method, params = params, metadata = metadata))
        if (response.hasError()) {
            throw IllegalStateException("gRPC error: ${response.error}")
        }
        return response.result
    }

    fun unary(request: GrpcRequest): GrpcResponse {
        val result = call(request.method, request.params, request.metadata)
        return GrpcResponse(result = result)
    }

    fun batch(requests: List<GrpcRequest>): List<GrpcResponse> = requests.map { request ->
        val result = call(request.method, request.params, request.metadata)
        GrpcResponse(result = result)
    }

    fun discoverRpcApi(): Any? = call("rpc.discover")

    fun getLatestCheckpointSequenceNumber(): Any? = call("sui_getLatestCheckpointSequenceNumber")

    fun getLatestSuiSystemState(): Any? = call("suix_getLatestSuiSystemState")

    fun getCoins(owner: String, coinType: String = "0x2::sui::SUI", cursor: String? = null, limit: Int? = null): Any? =
        call("suix_getCoins", listOf(owner, coinType, cursor, limit))

    fun getAllCoins(owner: String, cursor: String? = null, limit: Int? = null): Any? =
        call("suix_getAllCoins", listOf(owner, cursor, limit))

    fun getBalance(owner: String, coinType: String = "0x2::sui::SUI"): Any? =
        call("suix_getBalance", listOf(owner, coinType))

    fun getAllBalances(owner: String): Any? = call("suix_getAllBalances", listOf(owner))

    fun getCoinMetadata(coinType: String): Any? = call("suix_getCoinMetadata", listOf(coinType))

    fun getTotalSupply(coinType: String): Any? = call("suix_getTotalSupply", listOf(coinType))

    fun getOwnedObjects(owner: String, query: Map<String, Any?> = emptyMap(), cursor: String? = null, limit: Int? = null): Any? =
        call("suix_getOwnedObjects", listOf(owner, query, cursor, limit))

    fun getDynamicFields(parentObjectId: String, cursor: String? = null, limit: Int? = null): Any? =
        call("suix_getDynamicFields", listOf(parentObjectId, cursor, limit))

    fun getDynamicFieldObject(parentObjectId: String, name: Map<String, Any?>): Any? =
        call("suix_getDynamicFieldObject", listOf(parentObjectId, name))

    fun queryEvents(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int? = null,
        descendingOrder: Boolean = false,
    ): Any? = call("suix_queryEvents", listOf(query, cursor, limit, descendingOrder))

    fun queryTransactionBlocks(
        query: Map<String, Any?>,
        cursor: String? = null,
        limit: Int? = null,
        descendingOrder: Boolean = false,
    ): Any? = call("suix_queryTransactionBlocks", listOf(query, cursor, limit, descendingOrder))

    fun getObject(objectId: String, options: Map<String, Any?> = emptyMap()): Any? =
        call("sui_getObject", listOf(objectId, options))

    fun multiGetObjects(objectIds: List<String>, options: Map<String, Any?> = emptyMap()): Any? =
        call("sui_multiGetObjects", listOf(objectIds, options))

    fun getObjects(objectIds: List<String>, options: Map<String, Any?> = emptyMap()): List<GrpcResponse> =
        batch(objectIds.map { GrpcRequest(method = "sui_getObject", params = listOf(it, options)) })

    fun dryRunTransactionBlock(txBytesBase64: String): Any? =
        call("sui_dryRunTransactionBlock", listOf(txBytesBase64))

    fun devInspectTransactionBlock(sender: String, txBytesBase64: String): Any? =
        call("sui_devInspectTransactionBlock", listOf(sender, txBytesBase64))

    fun getEventsByTransaction(transactionDigest: String): Any? =
        call("sui_getEvents", listOf(transactionDigest))

    fun getTransactionBlock(digest: String, options: Map<String, Any?> = emptyMap()): Any? =
        call("sui_getTransactionBlock", listOf(digest, options))

    fun getTotalTransactionBlocks(): Any? = call("sui_getTotalTransactionBlocks")

    fun multiGetTransactionBlocks(digests: List<String>, options: Map<String, Any?> = emptyMap()): Any? =
        call("sui_multiGetTransactionBlocks", listOf(digests, options))

    fun getCheckpoints(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false): Any? =
        call("sui_getCheckpoints", listOf(cursor, limit, descendingOrder))

    fun getCheckpoint(checkpointId: String): Any? = call("sui_getCheckpoint", listOf(checkpointId))

    fun getChainIdentifier(): Any? = call("sui_getChainIdentifier")

    fun getCommitteeInfo(epoch: String? = null): Any? = call("suix_getCommitteeInfo", listOf(epoch))

    fun getProtocolConfig(version: String? = null): Any? = call("sui_getProtocolConfig", listOf(version))

    fun getNetworkMetrics(): Any? = call("suix_getNetworkMetrics")

    fun getAddressMetrics(): Any? = call("suix_getLatestAddressMetrics")

    fun getEpochMetrics(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false): Any? =
        call("suix_getEpochMetrics", listOf(cursor, limit, descendingOrder))

    fun getAllEpochAddressMetrics(descendingOrder: Boolean = false): Any? =
        call("suix_getAllEpochAddressMetrics", listOf(descendingOrder))

    fun getEpochs(cursor: String? = null, limit: Int? = null, descendingOrder: Boolean = false): Any? =
        call("suix_getEpochs", listOf(cursor, limit, descendingOrder))

    fun getMoveCallMetrics(): Any? = call("suix_getMoveCallMetrics")

    fun getCurrentEpoch(): Any? = call("suix_getCurrentEpoch")

    fun resolveNameServiceAddress(name: String): Any? = call("suix_resolveNameServiceAddress", listOf(name))

    fun resolveNameServiceNames(address: String, cursor: String? = null, limit: Int? = null): Any? =
        call("suix_resolveNameServiceNames", listOf(address, cursor, limit))

    fun getValidatorsApy(): Any? = call("suix_getValidatorsApy")

    fun getStakes(owner: String): Any? = call("suix_getStakes", listOf(owner))

    fun getStakesByIds(stakedSuiIds: List<String>): Any? = call("suix_getStakesByIds", listOf(stakedSuiIds))

    fun tryGetPastObject(objectId: String, version: Int, options: Map<String, Any?> = emptyMap()): Any? =
        call("sui_tryGetPastObject", listOf(objectId, version, options))

    fun tryMultiGetPastObjects(pastObjects: List<Map<String, Any?>>, options: Map<String, Any?> = emptyMap()): Any? =
        call("sui_tryMultiGetPastObjects", listOf(pastObjects, options))

    fun getNormalizedMoveModulesByPackage(packageId: String): Any? =
        call("sui_getNormalizedMoveModulesByPackage", listOf(packageId))

    fun getNormalizedMoveModule(packageId: String, moduleName: String): Any? =
        call("sui_getNormalizedMoveModule", listOf(packageId, moduleName))

    fun getNormalizedMoveFunction(packageId: String, moduleName: String, functionName: String): Any? =
        call("sui_getNormalizedMoveFunction", listOf(packageId, moduleName, functionName))

    fun getMoveFunctionArgTypes(packageId: String, moduleName: String, functionName: String): Any? =
        call("sui_getMoveFunctionArgTypes", listOf(packageId, moduleName, functionName))

    fun getNormalizedMoveStruct(packageId: String, moduleName: String, structName: String): Any? =
        call("sui_getNormalizedMoveStruct", listOf(packageId, moduleName, structName))

    fun verifyZkLoginSignature(bytes: String, signature: String, intentScope: Int, author: String): Any? =
        call("sui_verifyZkLoginSignature", listOf(bytes, signature, intentScope, author))

    fun getReferenceGasPrice(): Any? = call("suix_getReferenceGasPrice")

    fun executeTransactionBlock(
        txBytesBase64: String,
        signatures: List<String> = emptyList(),
        options: Map<String, Any?> = emptyMap(),
    ): Any? = call("sui_executeTransactionBlock", listOf(txBytesBase64, signatures, options))

    fun executeTransactionBlock(
        txBytesBase64: String,
        signature: String,
        options: Map<String, Any?> = emptyMap(),
    ): Any? = executeTransactionBlock(txBytesBase64, listOf(signature), options)

    fun executeTransactionBlock(
        txBytes: ByteArray,
        signatures: List<String> = emptyList(),
        options: Map<String, Any?> = emptyMap(),
    ): Any? = executeTransactionBlock(Base64.getEncoder().encodeToString(txBytes), signatures, options)

    fun executeTransactionBlock(
        txBytes: ByteArray,
        signature: String,
        options: Map<String, Any?> = emptyMap(),
    ): Any? = executeTransactionBlock(txBytes, listOf(signature), options)

    override fun close() {
        transport.close()
    }
}

typealias GrpcCoreClient = SuiGrpcClient
