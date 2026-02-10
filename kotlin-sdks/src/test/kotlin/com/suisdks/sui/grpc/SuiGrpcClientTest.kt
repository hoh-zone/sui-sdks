package com.suisdks.sui.grpc

import kotlin.test.Test
import kotlin.test.assertEquals

class SuiGrpcClientTest {
    @Test
    fun callPassesMetadataToTransport() {
        var captured: GrpcRequest? = null
        val transport = object : GrpcTransport {
            override fun unary(request: GrpcRequest): GrpcResponse {
                captured = request
                return GrpcResponse(result = mapOf("ok" to true))
            }
        }

        val client = SuiGrpcClient(transport)
        val out = client.call(
            method = "sui_getObject",
            params = listOf("0x1", emptyMap<String, Any?>()),
            metadata = mapOf("x-api-key" to "k", "authorization" to "Bearer t"),
        )

        assertEquals("sui_getObject", captured!!.method)
        assertEquals("k", captured!!.metadata["x-api-key"])
        assertEquals("Bearer t", captured!!.metadata["authorization"])
        assertEquals(true, (out as Map<*, *>)["ok"])
    }
}
