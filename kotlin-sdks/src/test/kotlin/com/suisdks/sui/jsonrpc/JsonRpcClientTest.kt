package com.suisdks.sui.jsonrpc

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class JsonRpcClientTest {
    @Test
    fun fromNetworkSupportsLocalnet() {
        // Verifies network mapping exists; request path is covered by transport-specific tests.
        JsonRpcClient.fromNetwork("localnet")
    }

    @Test
    fun callReturnsResultEnvelope() {
        var lastMethod = ""
        var lastParams: List<Any?> = emptyList()
        val client = JsonRpcClient(object : JsonRpcTransport {
            override fun request(method: String, params: List<Any?>): Map<String, Any?> {
                lastMethod = method
                lastParams = params
                return mapOf("jsonrpc" to "2.0", "id" to 1, "result" to mapOf("ok" to true))
            }
        })

        val out = client.call("sui_getObject", listOf("0x1", emptyMap<String, Any?>()))
        val result = out["result"] as Map<*, *>
        assertEquals(true, result["ok"])
        assertEquals("sui_getObject", lastMethod)
        assertEquals("0x1", lastParams.first())

        val raw = client.callRaw("sui_getChainIdentifier")
        assertEquals("2.0", raw["jsonrpc"])

        val batch = client.batchCall(
            listOf(
                "sui_getChainIdentifier" to emptyList(),
                "sui_getLatestCheckpointSequenceNumber" to emptyList(),
            ),
        )
        assertEquals(2, batch.size)
        client.close()
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

    @Test
    fun fromEndpointFactoryExists() {
        JsonRpcClient.fromEndpoint("http://127.0.0.1:9000", timeoutMs = 1000)
    }
}
