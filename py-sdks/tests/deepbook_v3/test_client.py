import base64
import unittest

from pysdks.bcs import BCSWriter
from pysdks.deepbook_v3 import BalanceManager, DeepBookClient, DeepBookConfig, MarginManager


class _MockDryRunClient:
    def call(self, method, params=None):
        self._last_method = method
        _ = params
        w = BCSWriter()
        w.write_u64(100)
        one = base64.b64encode(w.to_bytes()).decode()

        w2 = BCSWriter()
        w2.write_u64(200)
        two = base64.b64encode(w2.to_bytes()).decode()

        w3 = BCSWriter()
        w3.write_u64(300)
        three = base64.b64encode(w3.to_bytes()).decode()

        return {
            "commandResults": [
                {"returnValues": [{"bcs": one}, {"bcs": two}, {"bcs": three}]},
                {"returnValues": [{"bcs": one}]},
            ]
        }


class _MockBadClient:
    def call(self, method, params=None):
        _ = method
        _ = params
        return {}


class TestDeepBookClient(unittest.TestCase):
    def _client(self):
        cfg = DeepBookConfig(
            address="0x1",
            balance_managers={"m1": BalanceManager(address="0x2")},
            margin_managers={"mm1": MarginManager(address="0x3", pool_key="DEEP_SUI")},
        )
        return DeepBookClient(client=_MockDryRunClient(), config=cfg)

    def test_quantity_methods(self):
        c = self._client()
        out = c.get_quote_quantity_out("DEEP_SUI", 1)
        self.assertIn("deepRequired", out)
        out2 = c.get_base_quantity_out("DEEP_SUI", 1)
        self.assertIn("deepRequired", out2)
        out3 = c.get_quantity_out("DEEP_SUI", 1, 0)
        self.assertIn("deepRequired", out3)

    def test_mid_price_and_order_paths(self):
        c = self._client()
        self.assertGreater(c.mid_price("DEEP_SUI"), 0)
        self.assertTrue(c.get_order("DEEP_SUI", "1"))
        self.assertTrue(c.get_margin_account_order_details("mm1"))

    def test_check_manager_balance(self):
        c = self._client()
        out = c.check_manager_balance("m1", "SUI")
        self.assertIn("coinType", out)

    def test_whitelisted_false_for_non_bool_byte(self):
        c = self._client()
        self.assertFalse(c.whitelisted("DEEP_SUI"))

    def test_error_path(self):
        cfg = DeepBookConfig(address="0x1", balance_managers={"m1": BalanceManager(address="0x2")})
        c = DeepBookClient(client=_MockBadClient(), config=cfg)
        with self.assertRaises(ValueError):
            c.check_manager_balance("m1", "SUI")


if __name__ == "__main__":
    unittest.main()
