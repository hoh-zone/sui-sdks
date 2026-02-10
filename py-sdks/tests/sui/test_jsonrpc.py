import json
import unittest
from unittest.mock import MagicMock, patch

from pysdks.sui import JsonRpcClient


class TestJsonRpc(unittest.TestCase):
    def test_from_network(self):
        c = JsonRpcClient.from_network("testnet")
        self.assertIn("testnet", c.endpoint)

    def test_call_success(self):
        client = JsonRpcClient(endpoint="https://example.invalid")
        mock_resp = MagicMock()
        mock_resp.__enter__.return_value.read.return_value = json.dumps({"jsonrpc": "2.0", "result": {"ok": True}}).encode()

        with patch("pysdks.sui.jsonrpc.urlopen", return_value=mock_resp):
            out = client.call("sui_getLatestCheckpointSequenceNumber")
            self.assertIn("result", out)

    def test_call_error(self):
        client = JsonRpcClient(endpoint="https://example.invalid")
        mock_resp = MagicMock()
        mock_resp.__enter__.return_value.read.return_value = json.dumps({"error": {"code": -1}}).encode()

        with patch("pysdks.sui.jsonrpc.urlopen", return_value=mock_resp):
            with self.assertRaises(RuntimeError):
                client.call("bad_method")


if __name__ == "__main__":
    unittest.main()
