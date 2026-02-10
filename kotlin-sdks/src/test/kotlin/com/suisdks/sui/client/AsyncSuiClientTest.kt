package com.suisdks.sui.client

import com.suisdks.sui.jsonrpc.JsonRpcClient
import com.suisdks.sui.jsonrpc.JsonRpcTransport
import kotlin.test.Test
import kotlin.test.assertEquals

class AsyncSuiClientTest {
    @Test
    fun asyncMethodsReturnExpectedValues() {
        val transport = object : JsonRpcTransport {
            override fun request(method: String, params: List<Any?>): Map<String, Any?> {
                return when (method) {
                    "sui_getTotalTransactionBlocks" -> mapOf("result" to "9")
                    "suix_getAllCoins" -> mapOf(
                        "result" to mapOf(
                            "data" to listOf(mapOf("coinObjectId" to "0x1")),
                            "hasNextPage" to false,
                            "nextCursor" to null,
                        ),
                    )
                    else -> mapOf("result" to mapOf("ok" to true))
                }
            }
        }

        val client = SuiClient(JsonRpcClient(transport))
        val asyncClient = AsyncSuiClient.fromClient(client)

        assertEquals(true, asyncClient.discoverRpcApi().get()["ok"])
        val coins = asyncClient.iterAllCoins("0xabc").get()
        assertEquals("0x1", coins.first()["coinObjectId"])
        assertEquals("9", asyncClient.getTotalTransactionBlocks().get())
        asyncClient.close().get()
    }
}
