package com.suisdks.sui.client

import com.suisdks.sui.jsonrpc.JsonRpcClient
import com.suisdks.sui.jsonrpc.JsonRpcTransport
import kotlin.test.Test
import kotlin.test.assertEquals

class SuiClientTest {
    @Test
    fun executeAndIterHelpers() {
        val transport = object : JsonRpcTransport {
            override fun request(method: String, params: List<Any?>): Map<String, Any?> {
                return when (method) {
                    "sui_getLatestCheckpointSequenceNumber" -> mapOf("result" to "123")
                    "sui_getTotalTransactionBlocks" -> mapOf("result" to "456")
                    "sui_getOwnedObjects" -> mapOf(
                        "result" to mapOf(
                            "data" to listOf(mapOf("objectId" to "0xlegacy")),
                            "hasNextPage" to false,
                            "nextCursor" to null,
                        ),
                    )
                    "suix_getAllCoins" -> {
                        val cursor = params.getOrNull(1) as? String
                        if (cursor == null) {
                            mapOf(
                                "result" to mapOf(
                                    "data" to listOf(mapOf("coinObjectId" to "0x1")),
                                    "hasNextPage" to true,
                                    "nextCursor" to "c1",
                                ),
                            )
                        } else {
                            mapOf(
                                "result" to mapOf(
                                    "data" to listOf(mapOf("coinObjectId" to "0x2")),
                                    "hasNextPage" to false,
                                    "nextCursor" to null,
                                ),
                            )
                        }
                    }
                    else -> mapOf("result" to mapOf("ok" to true))
                }
            }
        }
        val client = SuiClient(JsonRpcClient(transport))

        val api = client.discoverRpcApi()
        assertEquals(true, api["ok"])

        val coins = client.iterAllCoins(owner = "0xabc").toList()
        assertEquals(listOf("0x1", "0x2"), coins.map { it["coinObjectId"] })
        assertEquals("123", client.getLatestCheckpointSequenceNumber())
        assertEquals("456", client.getTotalTransactionBlocks())
        val legacy = client.getOwnedObjectsLegacy("0xabc")
        @Suppress("UNCHECKED_CAST")
        val legacyData = legacy["data"] as List<Map<String, Any?>>
        assertEquals("0xlegacy", legacyData.first()["objectId"])
    }
}
