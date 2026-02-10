import json
import unittest
from unittest.mock import MagicMock, patch

from pysdks.sui import FaucetClient, FaucetRateLimitError, get_faucet_host


class TestFaucet(unittest.TestCase):
    def test_get_host(self):
        self.assertIn("testnet", get_faucet_host("testnet"))
        with self.assertRaises(ValueError):
            get_faucet_host("mainnet")

    def test_request_success(self):
        c = FaucetClient.from_network("testnet")
        mock_resp = MagicMock()
        mock_resp.__enter__.return_value.read.return_value = json.dumps({"transferredGasObjects": []}).encode()
        with patch("pysdks.sui.faucet.urlopen", return_value=mock_resp):
            out = c.request_sui_from_faucet_v2("0x1")
            self.assertIn("transferredGasObjects", out)

    def test_request_429(self):
        c = FaucetClient.from_network("testnet")
        with patch("pysdks.sui.faucet.urlopen", side_effect=RuntimeError("HTTP Error 429")):
            with self.assertRaises(FaucetRateLimitError):
                c.request_sui_from_faucet_v2("0x1")


if __name__ == "__main__":
    unittest.main()
