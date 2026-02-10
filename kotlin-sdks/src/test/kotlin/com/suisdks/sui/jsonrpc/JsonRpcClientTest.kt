package com.suisdks.sui.jsonrpc

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class JsonRpcClientTest {
    @Test
    fun callReturnsResultEnvelope() {
        val client = JsonRpcClient(object : JsonRpcTransport {
            override fun request(method: String, params: List<Any?>): Map<String, Any?> {
                return mapOf("jsonrpc" to "2.0", "id" to 1, "result" to mapOf("ok" to true))
            }
        })

        val out = client.call("sui_getObject", listOf("0x1", emptyMap<String, Any?>()))
        val result = out["result"] as Map<*, *>
        assertEquals(true, result["ok"])
    }

    @Test
    fun callThrowsOnError() {
        val client = JsonRpcClient(object : JsonRpcTransport {
            override fun request(method: String, params: List<Any?>): Map<String, Any?> {
                return mapOf("error" to mapOf("message" to "boom"))
            }
        })

        assertFailsWith<IllegalStateException> {
            client.call("sui_getObject")
        }
    }
}
