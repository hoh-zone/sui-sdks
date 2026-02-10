import unittest

from pysdks.bcs import BCSReader
from pysdks.deepbook_v3.config import DeepBookConfig, MAX_TIMESTAMP
from pysdks.deepbook_v3.transactions.contracts import (
    BalanceManagerContract,
    DeepBookContract,
    FlashLoanContract,
    GovernanceContract,
    MarginManagerContract,
    MarginTPSLContract,
    PoolProxyContract,
)
from pysdks.deepbook_v3.types import (
    BalanceManager,
    CanPlaceLimitOrderParams,
    MarginManager,
    PlaceLimitOrderParams,
    PlaceMarginLimitOrderParams,
)
from pysdks.sui.transactions import Transaction


class TestContracts(unittest.TestCase):
    def _config(self):
        return DeepBookConfig(
            address="0x1",
            balance_managers={"m1": BalanceManager(address="0x2")},
            margin_managers={"mm1": MarginManager(address="0x3", pool_key="DEEP_SUI")},
        )

    def _last_call(self, tx: Transaction):
        return tx.commands[-1]["MoveCall"]

    def _target(self, call):
        if "target" in call:
            return call["target"]
        return f"{call['package']}::{call['module']}::{call['function']}"

    def _arg_bytes(self, tx: Transaction, arg):
        if isinstance(arg, dict) and arg.get("kind") == "pure":
            return arg["bytes"]
        if isinstance(arg, dict) and arg.get("$kind") == "Input":
            idx = arg["Input"]
            inp = tx.inputs[idx]
            pure = inp.get("Pure")
            if pure and isinstance(pure.get("bytes"), str):
                import base64

                return base64.b64decode(pure["bytes"])
        raise ValueError("argument is not pure bytes")

    def test_deepbook_place_limit_order(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        c = DeepBookContract(cfg, bm)
        tx = Transaction()
        c.place_limit_order(
            tx,
            PlaceLimitOrderParams(
                pool_key="DEEP_SUI",
                balance_manager_key="m1",
                client_order_id="42",
                price=1.5,
                quantity=2.0,
                is_bid=True,
            ),
        )
        call = self._last_call(tx)
        self.assertIn("::pool::place_limit_order", self._target(call))

        expiration = self._arg_bytes(tx, call["arguments"][10])
        self.assertEqual(MAX_TIMESTAMP, BCSReader(expiration).read_u64())

    def test_deepbook_cancel_orders_u128_vec(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        c = DeepBookContract(cfg, bm)
        tx = Transaction()
        c.cancel_orders(tx, "DEEP_SUI", "m1", ["1", "2"])
        call = self._last_call(tx)
        self.assertIn("::pool::cancel_orders", self._target(call))
        vec = self._arg_bytes(tx, call["arguments"][3])
        self.assertEqual(2, vec[0])
        self.assertEqual(1, int.from_bytes(vec[1:17], "little"))
        self.assertEqual(2, int.from_bytes(vec[17:33], "little"))

    def test_deepbook_can_place_limit_order(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        c = DeepBookContract(cfg, bm)
        tx = Transaction()
        c.can_place_limit_order(
            tx,
            CanPlaceLimitOrderParams(
                pool_key="DEEP_SUI",
                balance_manager_key="m1",
                price=1,
                quantity=1,
                is_bid=False,
                pay_with_deep=True,
                expire_timestamp=123,
            ),
        )
        call = self._last_call(tx)
        self.assertIn("::pool::can_place_limit_order", self._target(call))
        self.assertEqual(b"\x00", self._arg_bytes(tx, call["arguments"][4]))
        self.assertEqual(b"\x01", self._arg_bytes(tx, call["arguments"][5]))

    def test_governance_vote(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        c = GovernanceContract(cfg, bm)
        tx = Transaction()
        c.vote(tx, "DEEP_SUI", "m1", "7")
        call = self._last_call(tx)
        self.assertIn("::pool::vote", self._target(call))

    def test_flash_loan_amount_encoding(self):
        cfg = self._config()
        c = FlashLoanContract(cfg)
        tx = Transaction()
        c.borrow_base_asset(tx, "DEEP_SUI", 1.5)
        call = self._last_call(tx)
        self.assertIn("::pool::borrow_flashloan_base", self._target(call))
        amt = BCSReader(self._arg_bytes(tx, call["arguments"][1])).read_u64()
        self.assertGreater(amt, 0)

    def test_margin_manager_order_details_two_calls(self):
        cfg = self._config()
        c = MarginManagerContract(cfg)
        tx = Transaction()
        c.get_margin_account_order_details(tx, "mm1")
        self.assertEqual(2, len(tx.commands))
        self.assertIn("::margin_manager::balance_manager", self._target(tx.commands[0]["MoveCall"]))
        self.assertIn("::pool::get_account_order_details", self._target(tx.commands[1]["MoveCall"]))

    def test_pool_proxy_place_limit_default_expiration(self):
        cfg = self._config()
        c = PoolProxyContract(cfg)
        tx = Transaction()
        c.place_limit_order(
            tx,
            PlaceMarginLimitOrderParams(
                pool_key="DEEP_SUI",
                margin_manager_key="mm1",
                client_order_id="10",
                price=1,
                quantity=2,
                is_bid=True,
            ),
        )
        call = self._last_call(tx)
        exp = BCSReader(self._arg_bytes(tx, call["arguments"][8])).read_u64()
        self.assertEqual(MAX_TIMESTAMP, exp)

    def test_margin_tpsl_cancel_all(self):
        cfg = self._config()
        c = MarginTPSLContract(cfg)
        tx = Transaction()
        c.cancel_all_conditional_orders(tx, "mm1")
        call = self._last_call(tx)
        self.assertIn("cancel_all_conditional_orders", self._target(call))


if __name__ == "__main__":
    unittest.main()
