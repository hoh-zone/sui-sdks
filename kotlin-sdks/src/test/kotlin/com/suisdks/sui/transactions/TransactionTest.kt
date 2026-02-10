package com.suisdks.sui.transactions

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

private class FakeClient : DynamicClient {
    override fun call(method: String, params: List<Any?>): Any {
        return when (method) {
            "sui_executeTransactionBlock" -> mapOf("result" to mapOf("digest" to "0xabc", "params_size" to params.size))
            "sui_devInspectTransactionBlock" -> mapOf(
                "result" to mapOf(
                    "effects" to mapOf(
                        "gasUsed" to mapOf(
                            "computationCost" to "10",
                            "storageCost" to "20",
                            "storageRebate" to "3",
                        ),
                    ),
                ),
            )
            else -> mapOf("result" to emptyMap<String, Any?>())
        }
    }
}

class TransactionTest {
    @Test
    fun buildRoundtripAndExecute() {
        val tx = Transaction(FakeClient())
        tx.setSender("0x123")
        tx.setGasBudget(1_000)
        tx.transferSui("0x456", 100)
        tx.makeMoveVector(null, emptyList())
        tx.publicTransferObject("0x999", "0x123")

        val serialized = tx.buildBase64()
        val parsed = Transaction.fromSerialized(serialized)
        assertEquals("0x123", parsed.data.sender)
        assertTrue(parsed.data.commands.isNotEmpty())

        val exec = tx.execute()
        val result = exec["result"] as Map<*, *>
        assertEquals("0xabc", result["digest"])
        val asyncResult = tx.executeAsync().get()["result"] as Map<*, *>
        assertEquals("0xabc", asyncResult["digest"])

        val costs = tx.inspectForCost()
        assertEquals(27, costs["total_cost"])
        assertEquals(27, tx.inspectForCostAsync().get()["total_cost"])

        val digest1 = Transaction.digestFromBytes(tx.build())
        val digest2 = Transaction.digestFromB64Str(tx.buildBase64())
        assertEquals(digest1, digest2)
    }
}
