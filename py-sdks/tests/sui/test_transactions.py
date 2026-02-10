import unittest

from pysdks.sui.transactions import CachingExecutor, ParallelExecutor, Resolver, SerialExecutor, Transaction


class _MockClient:
    def __init__(self):
        self.calls = 0

    def call(self, method, params=None):
        self.calls += 1
        return {"method": method, "params": params, "calls": self.calls}


class TestTransactions(unittest.TestCase):
    def test_build_and_restore(self):
        tx = Transaction()
        tx.set_sender("0x1")
        tx.set_gas_budget_if_not_set(100)
        tx.move_call("0x2::foo::bar", [tx.object("0xabc")], ["0x2::sui::SUI"])

        b64 = tx.build_base64()
        restored = Transaction.from_serialized(b64)
        self.assertEqual("0x1", restored.data.sender)
        self.assertEqual(1, len(restored.data.commands))

    def test_resolver_plugin_pipeline(self):
        tx = Transaction()
        tx.object("0x123")

        seen = {"count": 0}

        def plugin(ctx):
            seen["count"] = len(ctx.unresolved_inputs)

        resolver = Resolver()
        resolver.add_plugin(plugin)
        resolver.resolve(tx)
        self.assertEqual(1, seen["count"])

    def test_caching_executor(self):
        client = _MockClient()
        executor = CachingExecutor(client)
        tx = Transaction()
        tx.move_call("0x2::foo::bar", [], [])

        a = executor.execute_transaction(tx)
        b = executor.execute_transaction(tx)
        self.assertEqual(a, b)
        self.assertEqual(1, client.calls)

    def test_serial_and_parallel_executor(self):
        client = _MockClient()
        cache_exec = CachingExecutor(client)
        serial = SerialExecutor(cache_exec)
        parallel = ParallelExecutor(cache_exec, max_workers=2)

        tx1 = Transaction()
        tx1.move_call("0x2::m::f1", [], [])
        tx2 = Transaction()
        tx2.move_call("0x2::m::f2", [], [])

        out1 = serial.execute([tx1, tx2])
        self.assertEqual(2, len(out1))

        out2 = parallel.execute([tx1, tx2])
        self.assertEqual(2, len(out2))


if __name__ == "__main__":
    unittest.main()
