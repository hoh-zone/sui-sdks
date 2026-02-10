import unittest

from pysdks.sui.transactions import (
    AsyncCachingExecutor,
    AsyncParallelExecutor,
    AsyncSerialExecutor,
    Transaction,
)


class _MockAsyncClient:
    def __init__(self):
        self.calls = 0
        self.method_calls = []

    async def execute(self, method, params=None):
        self.calls += 1
        self.method_calls.append((method, params or []))
        if method == "sui_devInspectTransactionBlock":
            return {
                "result": {
                    "effects": {
                        "gasUsed": {
                            "computationCost": "8",
                            "storageCost": "12",
                            "storageRebate": "3",
                        }
                    }
                }
            }
        return {"method": method, "params": params or [], "calls": self.calls}


class TestTransactionsAsync(unittest.IsolatedAsyncioTestCase):
    async def test_execute_async_and_inspect_cost(self):
        client = _MockAsyncClient()
        tx = Transaction(client=client)
        tx.set_sender("0x1")
        tx.move_call("0x2::m::f", [], [])

        out = await tx.execute_async()
        self.assertEqual("sui_executeTransactionBlock", out["method"])

        inspect = await tx.inspect_all_async()
        self.assertIn("result", inspect)

        cost = await tx.inspect_for_cost_async()
        self.assertEqual(8, cost["computation_cost"])
        self.assertEqual(12, cost["storage_cost"])
        self.assertEqual(3, cost["storage_rebate"])
        self.assertEqual(17, cost["total_cost"])

    async def test_async_executors(self):
        client = _MockAsyncClient()
        cache_exec = AsyncCachingExecutor(client)
        serial_exec = AsyncSerialExecutor(cache_exec)
        parallel_exec = AsyncParallelExecutor(cache_exec, max_workers=2)

        tx1 = Transaction()
        tx1.move_call("0x2::m::f1", [], [])
        tx2 = Transaction()
        tx2.move_call("0x2::m::f2", [], [])

        serial_out = await serial_exec.execute([tx1, tx2])
        self.assertEqual(2, len(serial_out))

        parallel_out = await parallel_exec.execute([tx1, tx2])
        self.assertEqual(2, len(parallel_out))

        # One extra execution path happened in the second run due to caching semantics:
        # tx1 and tx2 were already cached after serial execution and reused in parallel.
        self.assertEqual(2, client.calls)

    async def test_async_caching_executor(self):
        client = _MockAsyncClient()
        exec_ = AsyncCachingExecutor(client)
        tx = Transaction()
        tx.move_call("0x2::m::f", [], [])

        first = await exec_.execute_transaction(tx)
        second = await exec_.execute_transaction(tx)
        self.assertEqual(first, second)
        self.assertEqual(1, client.calls)


if __name__ == "__main__":
    unittest.main()
