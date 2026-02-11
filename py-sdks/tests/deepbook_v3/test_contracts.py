import unittest

from pysdks.bcs import BCSReader
from pysdks.deepbook_v3.config import DeepBookConfig, MAX_TIMESTAMP
from pysdks.deepbook_v3.transactions.contracts import (
    BalanceManagerContract,
    DeepBookContract,
    FlashLoanContract,
    GovernanceContract,
    MarginManagerContract,
    MarginRegistryContract,
    MarginTPSLContract,
    PoolProxyContract,
)
from pysdks.deepbook_v3.types import (
    BalanceManager,
    CanPlaceLimitOrderParams,
    CanPlaceMarketOrderParams,
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

    def test_deepbook_get_orders_u128_vec(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        c = DeepBookContract(cfg, bm)
        tx = Transaction()
        c.get_orders(tx, "DEEP_SUI", ["9", "10"])
        call = self._last_call(tx)
        self.assertIn("::pool::get_orders", self._target(call))
        vec = self._arg_bytes(tx, call["arguments"][1])
        self.assertEqual(2, vec[0])
        self.assertEqual(9, int.from_bytes(vec[1:17], "little"))
        self.assertEqual(10, int.from_bytes(vec[17:33], "little"))

    def test_referral_read_calls(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        deep = DeepBookContract(cfg, bm)
        tx = Transaction()
        bm.balance_manager_referral_owner(tx, "0xref")
        call = self._last_call(tx)
        self.assertIn("::balance_manager::balance_manager_referral_owner", self._target(call))

        tx2 = Transaction()
        bm.balance_manager_referral_pool_id(tx2, "0xref")
        call2 = self._last_call(tx2)
        self.assertIn("::balance_manager::balance_manager_referral_pool_id", self._target(call2))

        tx3 = Transaction()
        bm.get_balance_manager_referral_id(tx3, "m1", "DEEP_SUI")
        call3 = self._last_call(tx3)
        self.assertIn("::balance_manager::get_balance_manager_referral_id", self._target(call3))
        pool_id_bytes = self._arg_bytes(tx3, call3["arguments"][1])
        self.assertEqual(32, len(pool_id_bytes))

        tx4 = Transaction()
        deep.get_pool_referral_balances(tx4, "DEEP_SUI", "0xref")
        call4 = self._last_call(tx4)
        self.assertIn("::pool::get_pool_referral_balances", self._target(call4))

        tx5 = Transaction()
        deep.pool_referral_multiplier(tx5, "DEEP_SUI", "0xref")
        call5 = self._last_call(tx5)
        self.assertIn("::pool::pool_referral_multiplier", self._target(call5))

    def test_additional_read_calls(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        deep = DeepBookContract(cfg, bm)

        tx1 = Transaction()
        deep.get_account_order_details(tx1, "DEEP_SUI", "m1")
        self.assertIn("::pool::get_account_order_details", self._target(self._last_call(tx1)))

        tx2 = Transaction()
        deep.account_exists(tx2, "DEEP_SUI", "m1")
        self.assertIn("::pool::account_exists", self._target(self._last_call(tx2)))

        tx3 = Transaction()
        deep.pool_trade_params_next(tx3, "DEEP_SUI")
        self.assertIn("::pool::pool_trade_params_next", self._target(self._last_call(tx3)))

        tx4 = Transaction()
        deep.quorum(tx4, "DEEP_SUI")
        self.assertIn("::pool::quorum", self._target(self._last_call(tx4)))

        tx5 = Transaction()
        deep.pool_id(tx5, "DEEP_SUI")
        self.assertIn("::pool::id", self._target(self._last_call(tx5)))

        tx6 = Transaction()
        deep.get_base_quantity_in(tx6, "DEEP_SUI", 1.0, True)
        self.assertIn("::pool::get_base_quantity_in", self._target(self._last_call(tx6)))

        tx7 = Transaction()
        deep.get_quote_quantity_in(tx7, "DEEP_SUI", 1.0, False)
        self.assertIn("::pool::get_quote_quantity_in", self._target(self._last_call(tx7)))

        tx8 = Transaction()
        deep.get_order_deep_required(tx8, "DEEP_SUI", 1.0, 0.2)
        self.assertIn("::pool::get_order_deep_required", self._target(self._last_call(tx8)))

        tx9 = Transaction()
        deep.check_market_order_params(tx9, "DEEP_SUI", 1.0)
        self.assertIn("::pool::check_market_order_params", self._target(self._last_call(tx9)))

        tx10 = Transaction()
        deep.check_limit_order_params(tx10, "DEEP_SUI", 0.2, 1.0, 123)
        self.assertIn("::pool::check_limit_order_params", self._target(self._last_call(tx10)))

        tx11 = Transaction()
        deep.get_quote_quantity_out_input_fee(tx11, "DEEP_SUI", 1.0)
        self.assertIn("::pool::get_quote_quantity_out_input_fee", self._target(self._last_call(tx11)))

        tx12 = Transaction()
        deep.get_base_quantity_out_input_fee(tx12, "DEEP_SUI", 1.0)
        self.assertIn("::pool::get_base_quantity_out_input_fee", self._target(self._last_call(tx12)))

        tx13 = Transaction()
        deep.get_quantity_out_input_fee(tx13, "DEEP_SUI", 1.0, 1.0)
        self.assertIn("::pool::get_quantity_out_input_fee", self._target(self._last_call(tx13)))

        tx14 = Transaction()
        deep.stable_pool(tx14, "DEEP_SUI")
        self.assertIn("::pool::stable_pool", self._target(self._last_call(tx14)))

        tx15 = Transaction()
        deep.registered_pool(tx15, "DEEP_SUI")
        self.assertIn("::pool::registered_pool", self._target(self._last_call(tx15)))

        mr = MarginRegistryContract(cfg)
        tx16 = Transaction()
        mr.allowed_maintainers(tx16)
        self.assertIn("::margin_registry::allowed_maintainers", self._target(self._last_call(tx16)))

        tx17 = Transaction()
        mr.allowed_pause_caps(tx17)
        self.assertIn("::margin_registry::allowed_pause_caps", self._target(self._last_call(tx17)))

        tx18 = Transaction()
        mr.pool_enabled(tx18, "DEEP_SUI")
        self.assertIn("::margin_registry::pool_enabled", self._target(self._last_call(tx18)))

        tx19 = Transaction()
        mr.get_margin_pool_id(tx19, "SUI")
        self.assertIn("::margin_registry::get_margin_pool_id", self._target(self._last_call(tx19)))

        tx20 = Transaction()
        mr.get_deepbook_pool_margin_pool_ids(tx20, "DEEP_SUI")
        self.assertIn(
            "::margin_registry::get_deepbook_pool_margin_pool_ids",
            self._target(self._last_call(tx20)),
        )

        tx21 = Transaction()
        mr.get_margin_manager_ids(tx21, "0x1")
        self.assertIn("::margin_registry::get_margin_manager_ids", self._target(self._last_call(tx21)))

        tx22 = Transaction()
        mr.base_margin_pool_id(tx22, "DEEP_SUI")
        self.assertIn("::margin_registry::base_margin_pool_id", self._target(self._last_call(tx22)))

        tx23 = Transaction()
        mr.quote_margin_pool_id(tx23, "DEEP_SUI")
        self.assertIn("::margin_registry::quote_margin_pool_id", self._target(self._last_call(tx23)))

        tx24 = Transaction()
        mr.min_withdraw_risk_ratio(tx24, "DEEP_SUI")
        self.assertIn("::margin_registry::min_withdraw_risk_ratio", self._target(self._last_call(tx24)))

        tx25 = Transaction()
        mr.min_borrow_risk_ratio(tx25, "DEEP_SUI")
        self.assertIn("::margin_registry::min_borrow_risk_ratio", self._target(self._last_call(tx25)))

        tx26 = Transaction()
        mr.liquidation_risk_ratio(tx26, "DEEP_SUI")
        self.assertIn("::margin_registry::liquidation_risk_ratio", self._target(self._last_call(tx26)))

        tx27 = Transaction()
        mr.target_liquidation_risk_ratio(tx27, "DEEP_SUI")
        self.assertIn(
            "::margin_registry::target_liquidation_risk_ratio",
            self._target(self._last_call(tx27)),
        )

        tx28 = Transaction()
        mr.user_liquidation_reward(tx28, "DEEP_SUI")
        self.assertIn("::margin_registry::user_liquidation_reward", self._target(self._last_call(tx28)))

        tx29 = Transaction()
        mr.pool_liquidation_reward(tx29, "DEEP_SUI")
        self.assertIn("::margin_registry::pool_liquidation_reward", self._target(self._last_call(tx29)))

        mm = MarginManagerContract(cfg)
        tx30 = Transaction()
        mm.owner(tx30, "mm1")
        self.assertIn("::margin_manager::owner", self._target(self._last_call(tx30)))

        tx31 = Transaction()
        mm.deepbook_pool(tx31, "mm1")
        self.assertIn("::margin_manager::deepbook_pool", self._target(self._last_call(tx31)))

        tx32 = Transaction()
        mm.margin_pool_id(tx32, "mm1")
        self.assertIn("::margin_manager::margin_pool_id", self._target(self._last_call(tx32)))

        tx33 = Transaction()
        mm.borrowed_shares(tx33, "mm1")
        self.assertIn("::margin_manager::borrowed_shares", self._target(self._last_call(tx33)))

        tx34 = Transaction()
        mm.borrowed_base_shares(tx34, "mm1")
        self.assertIn("::margin_manager::borrowed_base_shares", self._target(self._last_call(tx34)))

        tx35 = Transaction()
        mm.borrowed_quote_shares(tx35, "mm1")
        self.assertIn("::margin_manager::borrowed_quote_shares", self._target(self._last_call(tx35)))

        tx36 = Transaction()
        mm.has_base_debt(tx36, "mm1")
        self.assertIn("::margin_manager::has_base_debt", self._target(self._last_call(tx36)))

        tx37 = Transaction()
        mm.balance_manager(tx37, "mm1")
        self.assertIn("::margin_manager::balance_manager", self._target(self._last_call(tx37)))

        tx38 = Transaction()
        mm.calculate_assets(tx38, "mm1")
        self.assertIn("::margin_manager::calculate_assets", self._target(self._last_call(tx38)))

        tx39 = Transaction()
        mm.calculate_debts(tx39, "mm1", "DEEP")
        self.assertIn("::margin_manager::calculate_debts", self._target(self._last_call(tx39)))

        tx40 = Transaction()
        mm.manager_state(tx40, "mm1")
        self.assertIn("::margin_manager::manager_state", self._target(self._last_call(tx40)))

        tx41 = Transaction()
        mm.manager_state_by_id(tx41, "DEEP_SUI", "0x3")
        self.assertIn("::margin_manager::manager_state", self._target(self._last_call(tx41)))

        tx42 = Transaction()
        mm.base_balance(tx42, "mm1")
        self.assertIn("::margin_manager::base_balance", self._target(self._last_call(tx42)))

        tx43 = Transaction()
        mm.quote_balance(tx43, "mm1")
        self.assertIn("::margin_manager::quote_balance", self._target(self._last_call(tx43)))

        tx44 = Transaction()
        mm.deep_balance(tx44, "mm1")
        self.assertIn("::margin_manager::deep_balance", self._target(self._last_call(tx44)))

        tpsl = MarginTPSLContract(cfg)
        tx45 = Transaction()
        tpsl.conditional_order_ids(tx45, "mm1")
        self.assertIn("::margin_manager::conditional_order_ids", self._target(self._last_call(tx45)))

        tx46 = Transaction()
        tpsl.lowest_trigger_above_price(tx46, "mm1")
        self.assertIn("::margin_manager::lowest_trigger_above_price", self._target(self._last_call(tx46)))

        tx47 = Transaction()
        tpsl.highest_trigger_below_price(tx47, "mm1")
        self.assertIn("::margin_manager::highest_trigger_below_price", self._target(self._last_call(tx47)))

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

    def test_deepbook_can_place_market_order(self):
        cfg = self._config()
        bm = BalanceManagerContract(cfg)
        c = DeepBookContract(cfg, bm)
        tx = Transaction()
        c.can_place_market_order(
            tx,
            CanPlaceMarketOrderParams(
                pool_key="DEEP_SUI",
                balance_manager_key="m1",
                quantity=1.0,
                is_bid=True,
                pay_with_deep=False,
            ),
        )
        call = self._last_call(tx)
        self.assertIn("::pool::can_place_market_order", self._target(call))
        self.assertEqual(b"\x01", self._arg_bytes(tx, call["arguments"][3]))
        self.assertEqual(b"\x00", self._arg_bytes(tx, call["arguments"][4]))

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
