package com.suisdks.sui.grpc

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

    fun getObject(objectId: String, options: Map<String, Any?> = emptyMap()): Any? =
        call("sui_getObject", listOf(objectId, options))

    fun executeTransactionBlock(
        txBytesBase64: String,
        signatures: List<String> = emptyList(),
        options: Map<String, Any?> = emptyMap(),
    ): Any? = call("sui_executeTransactionBlock", listOf(txBytesBase64, signatures, options))

    override fun close() {
        transport.close()
    }
}
