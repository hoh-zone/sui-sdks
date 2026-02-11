package com.suisdks.sui.client

import com.suisdks.sui.jsonrpc.JsonRpcClient
import com.suisdks.sui.jsonrpc.JsonRpcTransport
import com.suisdks.sui.transactions.Transaction
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class SuiClientTest {
    @Test
    fun fromEndpointFactoryExists() {
        SuiClient.fromEndpoint("http://127.0.0.1:9000", timeoutMs = 1000)
    }

    @Test
    fun executeAndIterHelpers() {
        var txLookupCalls = 0
        val transport = object : JsonRpcTransport {
            override fun request(method: String, params: List<Any?>): Map<String, Any?> {
                return when (method) {
                    "rpc.discover" -> mapOf("result" to mapOf("info" to mapOf("version" to "1.0.0")))
                    "sui_getLatestCheckpointSequenceNumber" -> mapOf("result" to "123")
                    "sui_getTotalTransactionBlocks" -> mapOf("result" to "456")
                    "sui_verifyZkLoginSignature" -> mapOf("result" to mapOf("success" to true))
                    "suix_getNetworkMetrics" -> mapOf("result" to mapOf("currentTps" to 1))
                    "suix_getLatestAddressMetrics" -> mapOf("result" to mapOf("cumulativeAddresses" to "2"))
                    "suix_getEpochMetrics" -> mapOf("result" to mapOf("data" to emptyList<Any>(), "hasNextPage" to false))
                    "suix_getAllEpochAddressMetrics" -> mapOf("result" to listOf(mapOf("epoch" to "1")))
                    "suix_getEpochs" -> mapOf("result" to mapOf("data" to listOf(mapOf("epoch" to "1")), "hasNextPage" to false))
                    "suix_getMoveCallMetrics" -> mapOf("result" to mapOf("rank3Days" to emptyList<Any>()))
                    "suix_getCurrentEpoch" -> mapOf("result" to mapOf("epoch" to "1"))
                    "sui_executeTransactionBlock" -> mapOf("result" to mapOf("digest" to "0xtx"))
                    "sui_dryRunTransactionBlock" -> mapOf("result" to mapOf("effects" to mapOf("status" to "success")))
                    "sui_devInspectTransactionBlock" -> mapOf("result" to mapOf("effects" to mapOf("status" to "success")))
                    "sui_getTransactionBlock" -> {
                        txLookupCalls += 1
                        if (txLookupCalls < 2) {
                            mapOf("error" to mapOf("message" to "not found"))
                        } else {
                            mapOf("result" to mapOf("digest" to (params.firstOrNull() ?: "")))
                        }
                    }
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
        assertEquals(true, (client.rpc().call("noop")["result"] as Map<*, *>)["ok"])

        val api = client.discoverRpcApi()
        assertEquals("1.0.0", client.getRpcApiVersion())
        assertEquals(true, api["ok"] ?: true)
        assertEquals("success", (client.dryRunTransactionBlock("AQ==")["effects"] as Map<*, *>)["status"])
        assertEquals("success", (client.devInspectTransactionBlock("0x1", "AQ==")["effects"] as Map<*, *>)["status"])

        val coins = client.iterAllCoins(owner = "0xabc").toList()
        assertEquals(listOf("0x1", "0x2"), coins.map { it["coinObjectId"] })
        assertEquals("123", client.getLatestCheckpointSequenceNumber())
        assertEquals("456", client.getTotalTransactionBlocks())
        val legacy = client.getOwnedObjectsLegacy("0xabc")
        @Suppress("UNCHECKED_CAST")
        val legacyData = legacy["data"] as List<Map<String, Any?>>
        assertEquals("0xlegacy", legacyData.first()["objectId"])

        val executed = client.executeTransactionBlock("AQ==", "sig", mapOf("showEffects" to true))
        assertEquals("0xtx", executed["digest"])
        val executedBytes = client.executeTransactionBlock(byteArrayOf(1), "sig", mapOf("showEffects" to true))
        assertEquals("0xtx", executedBytes["digest"])

        val waited = client.waitForTransaction("0xD", timeoutMs = 1000, pollIntervalMs = 1)
        assertEquals("0xD", waited["digest"])
        assertEquals(true, client.verifyZkLoginSignature("b", "s", 3, "0x1")["success"])

        val tx = Transaction().apply { setSender("0xabc") }
        val signed = client.signAndExecuteTransaction(
            transaction = tx,
            signer = TransactionSigner { bytes -> SignedTransaction(signature = "sig", bytes = bytes) },
        )
        assertEquals("0xtx", signed["digest"])
        val signedBytes = client.signAndExecuteTransaction(
            transactionBytes = byteArrayOf(1, 2, 3),
            signer = TransactionSigner { bytes -> SignedTransaction(signature = "sig", bytes = bytes) },
        )
        assertEquals("0xtx", signedBytes["digest"])
        assertEquals(1, client.getNetworkMetrics()["currentTps"])
        assertEquals("2", client.getAddressMetrics()["cumulativeAddresses"])
        assertEquals(false, client.getEpochMetrics()["hasNextPage"])
        assertEquals("1", (client.getAllEpochAddressMetrics()["result"] as? List<*>)?.firstOrNull()?.let { (it as Map<*, *>)["epoch"] })
        assertEquals(false, client.getEpochs()["hasNextPage"])
        assertEquals("1", client.getCurrentEpoch()["epoch"])

        assertFailsWith<IllegalArgumentException> {
            client.multiGetTransactionBlocks(listOf("0x1", "0x1"))
        }
        assertFailsWith<IllegalArgumentException> {
            client.multiGetObjects(listOf("0x1", "0x1"))
        }
        assertFailsWith<java.util.concurrent.CancellationException> {
            client.waitForTransaction("0xD", timeoutMs = 1000, pollIntervalMs = 1, shouldCancel = { true })
        }
    }
}
