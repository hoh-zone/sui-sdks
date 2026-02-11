package com.suisdks.sui.jsonrpc

class JsonRpcClient(private val transport: JsonRpcTransport) {
    companion object {
        fun fromNetwork(
            network: String = "testnet",
            timeoutMs: Int = 30_000,
            headers: Map<String, String> = emptyMap(),
        ): JsonRpcClient {
            val endpoint = getJsonRpcFullnodeUrl(network)
            return JsonRpcClient(HttpJsonRpcTransport(endpoint, timeoutMs, headers))
        }

        fun fromEndpoint(
            endpoint: String,
            timeoutMs: Int = 30_000,
            headers: Map<String, String> = emptyMap(),
        ): JsonRpcClient {
            return JsonRpcClient(HttpJsonRpcTransport(endpoint, timeoutMs, headers))
        }
    }

    fun callRaw(method: String, params: List<Any?> = emptyList()): Map<String, Any?> {
        return transport.request(method, params)
    }

    fun call(method: String, params: List<Any?> = emptyList()): Map<String, Any?> {
        val payload = callRaw(method, params)
        if (payload.containsKey("error") && payload["error"] != null) {
            throw IllegalStateException("jsonrpc error: ${payload["error"]}")
        }
        if (payload.containsKey("result")) {
            return mapOf("result" to payload["result"])
        }
        return payload
    }

    fun batchCall(requests: List<Pair<String, List<Any?>>>): List<Map<String, Any?>> {
        return requests.map { (method, params) -> call(method, params) }
    }

    fun getObject(objectId: String, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        call("sui_getObject", listOf(objectId, options))

    fun executeTransactionBlock(
        txBytesBase64: String,
        signatures: List<String> = emptyList(),
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> = call("sui_executeTransactionBlock", listOf(txBytesBase64, signatures, options))

    fun close() {
        // HttpJsonRpcTransport has no persistent resources to close.
    }
}
