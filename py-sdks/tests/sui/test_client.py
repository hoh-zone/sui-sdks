import unittest

from pysdks.sui.client import SuiClient


class _MockJsonRpcClient:
    def __init__(self):
        self.calls = []

    def call(self, method, params=None):
        self.calls.append((method, params or []))
        return {"jsonrpc": "2.0", "id": 1, "result": {"method": method, "params": params or []}}


class TestSuiClient(unittest.TestCase):
    def test_execute_and_dry_run(self):
        mock = _MockJsonRpcClient()
        c = SuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        c.execute("sui_getLatestCheckpointSequenceNumber")
        c.dry_run("AA==")

        self.assertEqual("sui_getLatestCheckpointSequenceNumber", mock.calls[0][0])
        self.assertEqual("sui_dryRunTransactionBlock", mock.calls[1][0])
        self.assertEqual(["AA=="], mock.calls[1][1])

    def test_get_object_and_get_objects(self):
        mock = _MockJsonRpcClient()
        c = SuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        c.get_object("0x1", {"showContent": True})
        out = c.get_objects(["0x2", "0x3"], {"showType": True})

        self.assertEqual("sui_getObject", mock.calls[0][0])
        self.assertEqual(["0x1", {"showContent": True}], mock.calls[0][1])
        self.assertEqual(2, len(out))
        self.assertEqual("0x2", mock.calls[1][1][0])
        self.assertEqual("0x3", mock.calls[2][1][0])

    def test_get_events(self):
        mock = _MockJsonRpcClient()
        c = SuiClient(endpoint="https://example.invalid", _rpc_client=mock)
        query = {"MoveEventType": "0x2::coin::Coin"}

        c.get_events(query=query, cursor="abc", limit=20, descending_order=True)

        self.assertEqual("suix_queryEvents", mock.calls[0][0])
        self.assertEqual([query, "abc", 20, True], mock.calls[0][1])

    def test_get_package(self):
        mock = _MockJsonRpcClient()
        c = SuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        c.get_package("0x2")

        self.assertEqual("sui_getObject", mock.calls[0][0])
        self.assertEqual("0x2", mock.calls[0][1][0])
        self.assertTrue(mock.calls[0][1][1]["showBcs"])
        self.assertTrue(mock.calls[0][1][1]["showContent"])

    def test_get_gas_and_close(self):
        mock = _MockJsonRpcClient()
        c = SuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        c.get_gas(owner="0xabc")
        c.close()

        self.assertEqual("suix_getCoins", mock.calls[0][0])
        self.assertEqual("0xabc", mock.calls[0][1][0])
        self.assertEqual("0x2::sui::SUI", mock.calls[0][1][1])


if __name__ == "__main__":
    unittest.main()
