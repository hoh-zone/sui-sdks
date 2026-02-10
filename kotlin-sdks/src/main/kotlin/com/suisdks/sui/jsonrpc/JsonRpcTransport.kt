package com.suisdks.sui.jsonrpc

interface JsonRpcTransport {
    fun request(method: String, params: List<Any?> = emptyList()): Map<String, Any?>
}
