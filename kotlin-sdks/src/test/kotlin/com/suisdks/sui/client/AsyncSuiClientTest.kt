package com.suisdks.sui.client

import com.suisdks.sui.jsonrpc.JsonRpcClient
import com.suisdks.sui.jsonrpc.JsonRpcTransport
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class AsyncSuiClientTest {
    @Test
    fun fromEndpointFactoryExists() {
        AsyncSuiClient.fromEndpoint("http://127.0.0.1:9000", timeoutMs = 1000).close().get()
    }

    @Test
    fun asyncMethodsReturnExpectedValues() {
        var txLookupCalls = 0
        val transport = object : JsonRpcTransport {
            override fun request(method: String, params: List<Any?>): Map<String, Any?> {
                return when (method) {
                    "rpc.discover" -> mapOf("result" to mapOf("info" to mapOf("version" to "1.1.0")))
                    "sui_getTotalTransactionBlocks" -> mapOf("result" to "9")
                    "sui_verifyZkLoginSignature" -> mapOf("result" to mapOf("success" to true))
                    "suix_getNetworkMetrics" -> mapOf("result" to mapOf("currentTps" to 3))
                    "suix_getCurrentEpoch" -> mapOf("result" to mapOf("epoch" to "2"))
                    "sui_executeTransactionBlock" -> mapOf("result" to mapOf("digest" to "0xA"))
                    "sui_dryRunTransactionBlock" -> mapOf("result" to mapOf("effects" to mapOf("status" to "success")))
                    "sui_devInspectTransactionBlock" -> mapOf("result" to mapOf("effects" to mapOf("status" to "success")))
                    "sui_getTransactionBlock" -> {
                        txLookupCalls += 1
                        if (txLookupCalls < 2) {
                            mapOf("error" to mapOf("message" to "not found"))
                        } else {
                            mapOf("result" to mapOf("digest" to "0xA"))
                        }
                    }
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
        assertEquals(true, (asyncClient.rpc().call("noop")["result"] as Map<*, *>)["ok"])

        assertEquals(true, asyncClient.discoverRpcApi().get()["ok"])
        assertEquals("1.1.0", asyncClient.getRpcApiVersion().get())
        assertEquals("success", (asyncClient.dryRunTransactionBlock("AQ==").get()["effects"] as Map<*, *>)["status"])
        assertEquals("success", (asyncClient.devInspectTransactionBlock("0x1", "AQ==").get()["effects"] as Map<*, *>)["status"])
        val coins = asyncClient.iterAllCoins("0xabc").get()
        assertEquals("0x1", coins.first()["coinObjectId"])
        assertEquals("9", asyncClient.getTotalTransactionBlocks().get())
        assertEquals(true, asyncClient.verifyZkLoginSignature("b", "s", 3, "0x1").get()["success"])
        assertEquals("0xA", asyncClient.executeTransactionBlock("AQ==", "sig").get()["digest"])
        assertEquals("0xA", asyncClient.executeTransactionBlock(byteArrayOf(1), "sig").get()["digest"])
        assertEquals(3, asyncClient.getNetworkMetrics().get()["currentTps"])
        assertEquals("2", asyncClient.getCurrentEpoch().get()["epoch"])
        assertEquals(
            "0xA",
            asyncClient.signAndExecuteTransaction(
                byteArrayOf(1, 2, 3),
                TransactionSigner { bytes -> SignedTransaction(signature = "sig", bytes = bytes) },
            ).get()["digest"],
        )
        assertEquals("0xA", asyncClient.waitForTransaction("0xA", timeoutMs = 1000, pollIntervalMs = 1).get()["digest"])
        assertFailsWith<java.util.concurrent.ExecutionException> {
            asyncClient.waitForTransaction("0xA", timeoutMs = 1000, pollIntervalMs = 1, shouldCancel = { true }).get()
        }
        asyncClient.close().get()
    }
}
