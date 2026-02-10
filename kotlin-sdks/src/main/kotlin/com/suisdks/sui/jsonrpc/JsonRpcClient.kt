package com.suisdks.sui.jsonrpc

class JsonRpcClient(private val transport: JsonRpcTransport) {
    companion object {
        private val endpoints = mapOf(
            "mainnet" to "https://fullnode.mainnet.sui.io:443",
            "testnet" to "https://fullnode.testnet.sui.io:443",
            "devnet" to "https://fullnode.devnet.sui.io:443",
        )

        fun fromNetwork(network: String = "testnet"): JsonRpcClient {
            val endpoint = endpoints[network] ?: throw IllegalArgumentException("unsupported network: $network")
            return JsonRpcClient(HttpJsonRpcTransport(endpoint))
        }
    }

    fun call(method: String, params: List<Any?> = emptyList()): Map<String, Any?> {
        val payload = transport.request(method, params)
        if (payload.containsKey("error") && payload["error"] != null) {
            throw IllegalStateException("jsonrpc error: ${payload["error"]}")
        }
        if (payload.containsKey("result")) {
            return mapOf("result" to payload["result"])
        }
        return payload
    }

    fun getObject(objectId: String, options: Map<String, Any?> = emptyMap()): Map<String, Any?> =
        call("sui_getObject", listOf(objectId, options))

    fun executeTransactionBlock(
        txBytesBase64: String,
        signatures: List<String> = emptyList(),
        options: Map<String, Any?> = emptyMap(),
    ): Map<String, Any?> = call("sui_executeTransactionBlock", listOf(txBytesBase64, signatures, options))
}
