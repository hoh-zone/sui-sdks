import unittest

from pysdks.sui.transactions import CachingExecutor, ParallelExecutor, Resolver, SerialExecutor, Transaction


class _MockClient:
    def __init__(self):
        self.calls = 0

    def call(self, method, params=None):
        self.calls += 1
        if method == "sui_devInspectTransactionBlock":
            return {
                "result": {
                    "effects": {
                        "gasUsed": {
                            "computationCost": "10",
                            "storageCost": "20",
                            "storageRebate": "5",
                        }
                    }
                }
            }
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

    def test_transaction_execution_methods(self):
        client = _MockClient()
        tx = Transaction(client=client)
        tx.set_sender("0x1")
        tx.move_call("0x2::m::f1", [], [])

        data = tx.get_transaction_data()
        self.assertEqual("0x1", data["Sender"])
        self.assertEqual(1, len(data["Commands"]))

        deferred = tx.deferred_execution()
        self.assertEqual("0x1", deferred["sender"])
        self.assertTrue(isinstance(deferred["tx_bytes"], str))

        out = tx.execute()
        self.assertEqual("sui_executeTransactionBlock", out["method"])

        out2 = tx.execute(signatures=["sig1"], options={"showEffects": True})
        self.assertEqual("sui_executeTransactionBlock", out2["method"])
        self.assertEqual(["sig1"], out2["params"][1])
        self.assertTrue(out2["params"][2]["showEffects"])

        tx.inspect_all()
        cost = tx.inspect_for_cost()
        self.assertEqual(10, cost["computation_cost"])
        self.assertEqual(20, cost["storage_cost"])
        self.assertEqual(5, cost["storage_rebate"])
        self.assertEqual(25, cost["total_cost"])

    def test_transaction_execute_requires_client(self):
        tx = Transaction()
        with self.assertRaises(ValueError):
            tx.execute()

    def test_advanced_command_builders(self):
        tx = Transaction()
        tx.set_expiration({"Epoch": 999})
        tx.set_gas_price(10)
        tx.set_gas_owner("0xabc")
        tx.set_gas_payment([{"objectId": "0x1", "digest": "abc", "version": 1}])

        coin = tx.gas()
        amt = tx.pure((100).to_bytes(8, "little"))
        split_res = tx.split_coins(coin, [amt])
        tx.transfer_objects([split_res], tx.pure(b"recipient"))
        tx.merge_coins(coin, [split_res])
        tx.publish([b"\x00\x01"], ["0x2"])
        tx.make_move_vec("0x2::sui::SUI", [coin])

        self.assertEqual({"Epoch": 999}, tx.data.expiration)
        self.assertEqual("10", tx.data.gas_data["price"])
        self.assertEqual("0xabc", tx.data.gas_data["owner"])
        self.assertEqual(5, len(tx.data.commands))
        self.assertEqual("SplitCoins", tx.data.commands[0]["$kind"])
        self.assertEqual("TransferObjects", tx.data.commands[1]["$kind"])
        self.assertEqual("MergeCoins", tx.data.commands[2]["$kind"])
        self.assertEqual("Publish", tx.data.commands[3]["$kind"])
        self.assertEqual("AAE=", tx.data.commands[3]["Publish"]["modules"][0])
        self.assertEqual("MakeMoveVec", tx.data.commands[4]["$kind"])

    def test_transfer_and_split_convenience_methods(self):
        tx = Transaction()

        tx.transfer_sui("0xrecipient", 123)
        self.assertEqual("SplitCoins", tx.data.commands[0]["$kind"])
        self.assertEqual("TransferObjects", tx.data.commands[1]["$kind"])

        coin = tx.gas()
        split_result = tx.split_coin_equal(coin, split_count=3, amount_per_split=7)
        self.assertEqual("Result", split_result["$kind"])
        self.assertEqual("SplitCoins", tx.data.commands[2]["$kind"])
        self.assertEqual(3, len(tx.data.commands[2]["SplitCoins"]["amounts"]))

        tx.split_coin_and_return(coin, amount=5, recipient="0xabc")
        self.assertEqual("SplitCoins", tx.data.commands[3]["$kind"])
        self.assertEqual("TransferObjects", tx.data.commands[4]["$kind"])

    def test_split_coin_equal_validation(self):
        tx = Transaction()
        with self.assertRaises(ValueError):
            tx.split_coin_equal(tx.gas(), split_count=0, amount_per_split=1)

    def test_stake_unstake_baseline(self):
        tx = Transaction()
        tx.stake_coin(coins=["0x11", "0x12"], validator_address="0xvalidator", amount=9)
        tx.unstake_coin(staked_coin="0xstaked")

        self.assertEqual("MakeMoveVec", tx.data.commands[0]["$kind"])
        self.assertEqual("MoveCall", tx.data.commands[1]["$kind"])
        self.assertEqual("request_add_stake", tx.data.commands[1]["MoveCall"]["function"])
        self.assertEqual("MoveCall", tx.data.commands[2]["$kind"])
        self.assertEqual("request_withdraw_stake", tx.data.commands[2]["MoveCall"]["function"])

    def test_upgrade_command_builders(self):
        tx = Transaction()
        ticket = {"$kind": "Result", "Result": 0}
        tx.upgrade([b"\x01\x02"], ["0x2"], "0xpackage", ticket)
        tx.publish_upgrade([b"\x03\x04"], ["0x3"], "0xpackage2", ticket)
        tx.custom_upgrade([b"\x05\x06"], ["0x4"], "0xpackage3", ticket)

        self.assertEqual("Upgrade", tx.data.commands[0]["$kind"])
        self.assertEqual("AQI=", tx.data.commands[0]["Upgrade"]["modules"][0])
        self.assertEqual("0xpackage", tx.data.commands[0]["Upgrade"]["package"])
        self.assertEqual(ticket, tx.data.commands[0]["Upgrade"]["ticket"])
        self.assertEqual("Upgrade", tx.data.commands[1]["$kind"])
        self.assertEqual("Upgrade", tx.data.commands[2]["$kind"])


if __name__ == "__main__":
    unittest.main()
