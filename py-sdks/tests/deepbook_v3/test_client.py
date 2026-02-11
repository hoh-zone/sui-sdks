import base64
import unittest

from pysdks.bcs import BCSWriter
from pysdks.deepbook_v3 import BalanceManager, DeepBookClient, DeepBookConfig, MarginManager


class _MockDryRunClient:
    def call(self, method, params=None):
        self._last_method = method
        if method == "sui_getObject":
            return {
                "result": {
                    "data": {
                        "content": {
                            "fields": {
                                "price_info": {
                                    "fields": {
                                        "arrival_time": "1710000000",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        commands = params[0] if params else []
        fn = ""
        if commands and isinstance(commands[0], dict):
            move = commands[0].get("MoveCall") or {}
            mod = move.get("module", "")
            fun = move.get("function", "")
            fn = f"{mod}::{fun}"

        def _u64_b64(value: int) -> str:
            w = BCSWriter()
            w.write_u64(value)
            return base64.b64encode(w.to_bytes()).decode()

        def _state_result(manager_byte: int, pool_byte: int):
            return {
                "returnValues": [
                    {"bcs": base64.b64encode(bytes([manager_byte]) * 32).decode()},
                    {"bcs": base64.b64encode(bytes([pool_byte]) * 32).decode()},
                    {"bcs": _u64_b64(250_000_000)},  # risk ratio
                    {"bcs": _u64_b64(2_000_000)},  # base asset
                    {"bcs": _u64_b64(3_000_000_000)},  # quote asset
                    {"bcs": _u64_b64(1_000_000)},  # base debt
                    {"bcs": _u64_b64(1_500_000_000)},  # quote debt
                    {"bcs": _u64_b64(123456)},  # base pyth price
                    {"bcs": base64.b64encode(b"\x08").decode()},  # base pyth decimals
                    {"bcs": _u64_b64(654321)},  # quote pyth price
                    {"bcs": base64.b64encode(b"\x09").decode()},  # quote pyth decimals
                    {"bcs": _u64_b64(1111)},  # current price
                    {"bcs": _u64_b64(2222)},  # lowest trigger above
                    {"bcs": _u64_b64(3333)},  # highest trigger below
                ]
            }

        if fn == "pool::account_open_orders":
            # vector<u128>[11, 22]
            payload = b"\x02" + (11).to_bytes(16, "little") + (22).to_bytes(16, "little")
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "registry::get_balance_manager_ids":
            # vector<address>[0x..aa, 0x..bb]
            payload = b"\x02" + (b"\xAA" * 32) + (b"\xBB" * 32)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "balance_manager::balance_manager_referral_owner":
            payload = b"\xDD" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "balance_manager::balance_manager_referral_pool_id":
            payload = b"\xEE" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "balance_manager::get_balance_manager_referral_id":
            # Option<address>::Some(0x..ff)
            payload = b"\x01" + (b"\xFF" * 32)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "pool::get_pool_id_by_asset":
            payload = b"\xCC" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "pool::get_level2_range":
            # prices vec<u64>[1_000_000_000_000], quantities vec<u64>[2_000_000]
            prices = b"\x01" + (1_000_000_000_000).to_bytes(8, "little")
            quantities = b"\x01" + (2_000_000).to_bytes(8, "little")
            return {
                "commandResults": [
                    {"returnValues": [{"bcs": base64.b64encode(prices).decode()}, {"bcs": base64.b64encode(quantities).decode()}]}
                ]
            }

        if fn == "pool::get_level2_ticks_from_mid":
            bid_prices = b"\x01" + (1_000_000_000_000).to_bytes(8, "little")
            bid_qty = b"\x01" + (3_000_000).to_bytes(8, "little")
            ask_prices = b"\x01" + (2_000_000_000_000).to_bytes(8, "little")
            ask_qty = b"\x01" + (4_000_000).to_bytes(8, "little")
            return {
                "commandResults": [
                    {
                        "returnValues": [
                            {"bcs": base64.b64encode(bid_prices).decode()},
                            {"bcs": base64.b64encode(bid_qty).decode()},
                            {"bcs": base64.b64encode(ask_prices).decode()},
                            {"bcs": base64.b64encode(ask_qty).decode()},
                        ]
                    }
                ]
            }

        if fn == "pool::get_order_deep_price":
            # OrderDeepPrice { asset_is_base: true, deep_per_asset: 1_000_000_000 }
            payload = b"\x01" + (1_000_000_000).to_bytes(8, "little")
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "pool::get_pool_referral_balances":
            w1 = BCSWriter()
            w1.write_u64(100)
            w2 = BCSWriter()
            w2.write_u64(200)
            w3 = BCSWriter()
            w3.write_u64(300)
            return {
                "commandResults": [
                    {
                        "returnValues": [
                            {"bcs": base64.b64encode(w1.to_bytes()).decode()},
                            {"bcs": base64.b64encode(w2.to_bytes()).decode()},
                            {"bcs": base64.b64encode(w3.to_bytes()).decode()},
                        ]
                    }
                ]
            }

        if fn == "pool::pool_referral_multiplier":
            w = BCSWriter()
            w.write_u64(250_000_000)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(w.to_bytes()).decode()}]}]}

        if fn in {
            "pool::can_place_limit_order",
            "pool::can_place_market_order",
            "pool::check_market_order_params",
            "pool::check_limit_order_params",
            "pool::stable_pool",
            "pool::registered_pool",
        }:
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(b'\x01').decode()}]}]}

        if fn in {"margin_registry::allowed_maintainers", "margin_registry::allowed_pause_caps"}:
            payload = b"\x02" + (b"\x12" * 32) + (b"\x34" * 32)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_registry::pool_enabled":
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(b"\x01").decode()}]}]}

        if fn == "margin_registry::get_margin_pool_id":
            if len(commands) > 1 and isinstance(commands[1], dict):
                move2 = commands[1].get("MoveCall") or {}
                fn2 = f"{move2.get('module', '')}::{move2.get('function', '')}"
                if fn2 == "margin_manager::calculate_debts":
                    return {
                        "commandResults": [
                            {"returnValues": [{"bcs": base64.b64encode((b'\x56' * 32)).decode()}]},
                            {"returnValues": [{"bcs": _u64_b64(1_000_000)}, {"bcs": _u64_b64(2_000_000)}]},
                        ]
                    }
                if fn2 in {
                    "margin_pool::total_supply",
                    "margin_pool::supply_shares",
                    "margin_pool::total_borrow",
                    "margin_pool::borrow_shares",
                    "margin_pool::last_update_timestamp",
                    "margin_pool::supply_cap",
                    "margin_pool::max_utilization_rate",
                    "margin_pool::protocol_spread",
                    "margin_pool::min_borrow",
                    "margin_pool::interest_rate",
                    "margin_pool::user_supply_shares",
                    "margin_pool::user_supply_amount",
                }:
                    value = 1_234_567
                    if fn2 in {"margin_pool::max_utilization_rate", "margin_pool::protocol_spread", "margin_pool::interest_rate"}:
                        value = 250_000_000
                    if fn2 == "margin_pool::last_update_timestamp":
                        value = 1_700_000_000
                    return {
                        "commandResults": [
                            {"returnValues": [{"bcs": base64.b64encode((b'\x56' * 32)).decode()}]},
                            {"returnValues": [{"bcs": _u64_b64(value)}]},
                        ]
                    }
                if fn2 == "margin_pool::deepbook_pool_allowed":
                    return {
                        "commandResults": [
                            {"returnValues": [{"bcs": base64.b64encode((b'\x56' * 32)).decode()}]},
                            {"returnValues": [{"bcs": base64.b64encode(b"\x01").decode()}]},
                        ]
                    }
            payload = b"\x56" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_registry::get_deepbook_pool_margin_pool_ids":
            payload = (b"\x78" * 32) + (b"\x9A" * 32)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_registry::get_margin_manager_ids":
            payload = b"\x02" + (b"\x21" * 32) + (b"\x43" * 32)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_registry::base_margin_pool_id":
            payload = b"\xA1" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_registry::quote_margin_pool_id":
            payload = b"\xB2" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn in {
            "margin_registry::min_withdraw_risk_ratio",
            "margin_registry::min_borrow_risk_ratio",
            "margin_registry::liquidation_risk_ratio",
            "margin_registry::target_liquidation_risk_ratio",
            "margin_registry::user_liquidation_reward",
            "margin_registry::pool_liquidation_reward",
        }:
            w = BCSWriter()
            w.write_u64(250_000_000)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(w.to_bytes()).decode()}]}]}

        if fn == "margin_manager::owner":
            payload = b"\xAB" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_manager::deepbook_pool":
            payload = b"\xBC" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_manager::margin_pool_id":
            payload = b"\x01" + (b"\xCD" * 32)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_manager::borrowed_shares":
            w1 = BCSWriter()
            w1.write_u64(777)
            w2 = BCSWriter()
            w2.write_u64(888)
            return {
                "commandResults": [
                    {
                        "returnValues": [
                            {"bcs": base64.b64encode(w1.to_bytes()).decode()},
                            {"bcs": base64.b64encode(w2.to_bytes()).decode()},
                        ]
                    }
                ]
            }

        if fn == "margin_manager::borrowed_base_shares":
            w = BCSWriter()
            w.write_u64(999)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(w.to_bytes()).decode()}]}]}

        if fn == "margin_manager::borrowed_quote_shares":
            w = BCSWriter()
            w.write_u64(1001)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(w.to_bytes()).decode()}]}]}

        if fn == "margin_manager::has_base_debt":
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(b'\x01').decode()}]}]}

        if fn == "margin_manager::balance_manager":
            if len(commands) > 1:
                # Path used by get_margin_account_order_details: first command returns a handle,
                # second command returns the actual details payload.
                w = BCSWriter()
                w.write_u64(100)
                one = base64.b64encode(w.to_bytes()).decode()
                return {
                    "commandResults": [
                        {"returnValues": [{"bcs": one}]},
                        {"returnValues": [{"bcs": one}]},
                    ]
                }
            payload = b"\xDE" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_manager::base_balance":
            return {"commandResults": [{"returnValues": [{"bcs": _u64_b64(2_000_000)}]}]}

        if fn == "margin_manager::quote_balance":
            return {"commandResults": [{"returnValues": [{"bcs": _u64_b64(3_000_000_000)}]}]}

        if fn == "margin_manager::deep_balance":
            return {"commandResults": [{"returnValues": [{"bcs": _u64_b64(4_000_000)}]}]}

        if fn == "margin_manager::conditional_order_ids":
            payload = b"\x02" + (11).to_bytes(8, "little") + (22).to_bytes(8, "little")
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "margin_manager::lowest_trigger_above_price":
            return {"commandResults": [{"returnValues": [{"bcs": _u64_b64(4444)}]}]}

        if fn == "margin_manager::highest_trigger_below_price":
            return {"commandResults": [{"returnValues": [{"bcs": _u64_b64(5555)}]}]}

        if fn == "margin_manager::calculate_assets":
            return {"commandResults": [{"returnValues": [{"bcs": _u64_b64(2_000_000)}, {"bcs": _u64_b64(3_000_000_000)}]}]}

        if fn == "margin_manager::manager_state":
            if len(commands) > 1:
                results = []
                for i in range(len(commands)):
                    results.append(_state_result(0xA0 + i, 0xB0 + i))
                return {"commandResults": results}
            return {"commandResults": [_state_result(0xA1, 0xB1)]}

        if fn == "pool::account_exists":
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(b"\x01").decode()}]}]}

        if fn == "pool::id":
            payload = b"\x11" * 32
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "pool::quorum":
            w = BCSWriter()
            w.write_u64(1_500_000)
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(w.to_bytes()).decode()}]}]}

        if fn in {
            "pool::get_quote_quantity_out_input_fee",
            "pool::get_base_quantity_out_input_fee",
            "pool::get_quantity_out_input_fee",
        }:
            return {
                "commandResults": [
                    {
                        "returnValues": [
                            {"bcs": _u64_b64(100)},
                            {"bcs": _u64_b64(200)},
                            {"bcs": _u64_b64(300)},
                        ]
                    }
                ]
            }

        def _order_bytes(
            manager_byte: int,
            order_id: int,
            client_order_id: int,
            quantity: int,
            filled_quantity: int,
            fee_is_deep: bool,
            asset_is_base: bool,
            deep_per_asset: int,
            epoch: int,
            status: int,
            expire_timestamp: int,
        ) -> bytes:
            return b"".join(
                [
                    bytes([manager_byte]) * 32,
                    int(order_id).to_bytes(16, "little"),
                    int(client_order_id).to_bytes(8, "little"),
                    int(quantity).to_bytes(8, "little"),
                    int(filled_quantity).to_bytes(8, "little"),
                    (b"\x01" if fee_is_deep else b"\x00"),
                    (b"\x01" if asset_is_base else b"\x00"),
                    int(deep_per_asset).to_bytes(8, "little"),
                    int(epoch).to_bytes(8, "little"),
                    int(status).to_bytes(1, "little"),
                    int(expire_timestamp).to_bytes(8, "little"),
                ]
            )

        if fn == "pool::get_order":
            # is_bid=true (bit127=0), price=12345, order seq=77
            encoded_order_id = (12345 << 64) + 77
            payload = _order_bytes(
                manager_byte=0xAA,
                order_id=encoded_order_id,
                client_order_id=42,
                quantity=2_000_000,
                filled_quantity=500_000,
                fee_is_deep=True,
                asset_is_base=True,
                deep_per_asset=1_000_000_000,
                epoch=9,
                status=1,
                expire_timestamp=123456,
            )
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "pool::get_orders":
            # vector<Order>[2]
            order1 = _order_bytes(
                manager_byte=0xAA,
                order_id=(12345 << 64) + 77,
                client_order_id=42,
                quantity=2_000_000,
                filled_quantity=500_000,
                fee_is_deep=True,
                asset_is_base=True,
                deep_per_asset=1_000_000_000,
                epoch=9,
                status=1,
                expire_timestamp=123456,
            )
            # is_bid=false via bit127=1
            order2 = _order_bytes(
                manager_byte=0xBB,
                order_id=(1 << 127) + (67890 << 64) + 88,
                client_order_id=43,
                quantity=3_000_000,
                filled_quantity=1_000_000,
                fee_is_deep=False,
                asset_is_base=False,
                deep_per_asset=2_000_000_000,
                epoch=10,
                status=2,
                expire_timestamp=654321,
            )
            payload = b"\x02" + order1 + order2
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        if fn == "pool::get_account_order_details":
            order = _order_bytes(
                manager_byte=0xAA,
                order_id=(12345 << 64) + 77,
                client_order_id=42,
                quantity=2_000_000,
                filled_quantity=500_000,
                fee_is_deep=True,
                asset_is_base=True,
                deep_per_asset=1_000_000_000,
                epoch=9,
                status=1,
                expire_timestamp=123456,
            )
            payload = b"\x01" + order
            return {"commandResults": [{"returnValues": [{"bcs": base64.b64encode(payload).decode()}]}]}

        one = _u64_b64(100)
        two = _u64_b64(200)
        three = _u64_b64(300)

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
        order = c.get_order("DEEP_SUI", "1")
        self.assertIsNotNone(order)
        assert order is not None
        self.assertEqual(42, order["client_order_id"])
        self.assertTrue(order["fee_is_deep"])
        self.assertTrue(c.get_margin_account_order_details("mm1"))

    def test_additional_ts_parity_helpers(self):
        c = self._client()
        self.assertEqual(["11", "22"], c.account_open_orders("DEEP_SUI", "m1"))
        self.assertEqual("0x" + ("cc" * 32), c.get_pool_id_by_assets("0x2::sui::SUI", "0x2::sui::SUI"))
        self.assertEqual(
            ["0x" + ("aa" * 32), "0x" + ("bb" * 32)],
            c.get_balance_manager_ids("0x1"),
        )
        self.assertEqual("0x" + ("dd" * 32), c.balance_manager_referral_owner("0xref"))
        self.assertEqual("0x" + ("ee" * 32), c.balance_manager_referral_pool_id("0xref"))
        self.assertEqual("0x" + ("ff" * 32), c.get_balance_manager_referral_id("m1", "DEEP_SUI"))

    def test_more_ts_parity_helpers(self):
        c = self._client()
        vault = c.vault_balances("DEEP_SUI")
        self.assertIn("base", vault)
        self.assertIn("quote", vault)
        self.assertIn("deep", vault)

        trade = c.pool_trade_params("DEEP_SUI")
        self.assertIn("takerFee", trade)
        self.assertIn("makerFee", trade)
        self.assertIn("stakeRequired", trade)

        book = c.pool_book_params("DEEP_SUI")
        self.assertIn("tickSize", book)
        self.assertIn("lotSize", book)
        self.assertIn("minSize", book)

        locked = c.locked_balance("DEEP_SUI", "m1")
        self.assertIn("base", locked)
        self.assertIn("quote", locked)
        self.assertIn("deep", locked)

        self.assertTrue(c.account("DEEP_SUI", "m1"))

        l2 = c.get_level2_range("DEEP_SUI", 0.1, 0.2, True)
        self.assertEqual(1, len(l2["prices"]))
        self.assertEqual(1, len(l2["quantities"]))

        ticks = c.get_level2_ticks_from_mid("DEEP_SUI", 5)
        self.assertEqual(1, len(ticks["bid_prices"]))
        self.assertEqual(1, len(ticks["bid_quantities"]))
        self.assertEqual(1, len(ticks["ask_prices"]))
        self.assertEqual(1, len(ticks["ask_quantities"]))

        deep_price = c.get_pool_deep_price("DEEP_SUI")
        self.assertTrue(deep_price["asset_is_base"])
        self.assertIn("deep_per_base", deep_price)

        referral_balances = c.get_pool_referral_balances("DEEP_SUI", "0xref")
        self.assertIn("base", referral_balances)
        self.assertIn("quote", referral_balances)
        self.assertIn("deep", referral_balances)
        self.assertAlmostEqual(0.25, c.pool_referral_multiplier("DEEP_SUI", "0xref"))
        self.assertTrue(c.account_exists("DEEP_SUI", "m1"))
        self.assertEqual("0x" + ("11" * 32), c.pool_id("DEEP_SUI"))
        self.assertAlmostEqual(1.5, c.quorum("DEEP_SUI"))
        trade_next = c.pool_trade_params_next("DEEP_SUI")
        self.assertIn("takerFee", trade_next)
        self.assertIn("makerFee", trade_next)
        self.assertIn("stakeRequired", trade_next)
        deep_required = c.get_order_deep_required("DEEP_SUI", 1.0, 0.2)
        self.assertIn("deepRequiredTaker", deep_required)
        self.assertIn("deepRequiredMaker", deep_required)
        self.assertTrue(c.can_place_limit_order("DEEP_SUI", "m1", 0.2, 1.0, True, True, 123))
        self.assertTrue(c.can_place_market_order("DEEP_SUI", "m1", 1.0, True, True))
        self.assertTrue(c.check_market_order_params("DEEP_SUI", 1.0))
        self.assertTrue(c.check_limit_order_params("DEEP_SUI", 0.2, 1.0, 123))
        self.assertTrue(c.stable_pool("DEEP_SUI"))
        self.assertTrue(c.registered_pool("DEEP_SUI"))
        self.assertEqual(
            ["0x" + ("12" * 32), "0x" + ("34" * 32)],
            c.get_allowed_maintainers(),
        )
        self.assertEqual(
            ["0x" + ("12" * 32), "0x" + ("34" * 32)],
            c.get_allowed_pause_caps(),
        )
        self.assertTrue(c.is_pool_enabled_for_margin("DEEP_SUI"))
        self.assertEqual("0x" + ("56" * 32), c.get_margin_pool_id("SUI"))
        mm_pool_ids = c.get_deepbook_pool_margin_pool_ids("DEEP_SUI")
        self.assertEqual("0x" + ("78" * 32), mm_pool_ids["baseMarginPoolId"])
        self.assertEqual("0x" + ("9a" * 32), mm_pool_ids["quoteMarginPoolId"])
        self.assertEqual(
            ["0x" + ("21" * 32), "0x" + ("43" * 32)],
            c.get_margin_manager_ids_for_owner("0x1"),
        )
        self.assertEqual("0x" + ("a1" * 32), c.get_base_margin_pool_id("DEEP_SUI"))
        self.assertEqual("0x" + ("b2" * 32), c.get_quote_margin_pool_id("DEEP_SUI"))
        self.assertAlmostEqual(0.25, c.get_min_withdraw_risk_ratio("DEEP_SUI"))
        self.assertAlmostEqual(0.25, c.get_min_borrow_risk_ratio("DEEP_SUI"))
        self.assertAlmostEqual(0.25, c.get_liquidation_risk_ratio("DEEP_SUI"))
        self.assertAlmostEqual(0.25, c.get_target_liquidation_risk_ratio("DEEP_SUI"))
        self.assertAlmostEqual(0.25, c.get_user_liquidation_reward("DEEP_SUI"))
        self.assertAlmostEqual(0.25, c.get_pool_liquidation_reward("DEEP_SUI"))

        decoded = c.decode_order_id((1 << 127) + (12345 << 64) + 77)
        self.assertFalse(decoded["isBid"])
        self.assertEqual(12345, decoded["price"])
        self.assertEqual(77, decoded["orderId"])

    def test_order_normalized_and_orders(self):
        c = self._client()
        normalized = c.get_order_normalized("DEEP_SUI", "1")
        self.assertIsNotNone(normalized)
        assert normalized is not None
        self.assertEqual("2.000000000", normalized["quantity"])
        self.assertEqual("0.500000000", normalized["filled_quantity"])
        self.assertTrue(normalized["is_bid"])
        pool = c.config.get_pool("DEEP_SUI")
        base = c.config.get_coin(pool.base_coin)
        quote = c.config.get_coin(pool.quote_coin)
        expected = f"{(12345 * base.scalar / quote.scalar / 1_000_000_000):.9f}"
        self.assertEqual(expected, normalized["normalized_price"])

        orders = c.get_orders("DEEP_SUI", ["1", "2"])
        self.assertIsNotNone(orders)
        assert orders is not None
        self.assertEqual(2, len(orders))
        self.assertEqual(42, orders[0]["client_order_id"])
        self.assertEqual(43, orders[1]["client_order_id"])
        account_orders = c.get_account_order_details("DEEP_SUI", "m1")
        self.assertEqual(1, len(account_orders))
        self.assertEqual(42, account_orders[0]["client_order_id"])
        base_in = c.get_base_quantity_in("DEEP_SUI", 1.0, True)
        self.assertIn("baseIn", base_in)
        self.assertIn("quoteOut", base_in)
        self.assertIn("deepRequired", base_in)
        quote_in = c.get_quote_quantity_in("DEEP_SUI", 1.0, True)
        self.assertIn("baseOut", quote_in)
        self.assertIn("quoteIn", quote_in)
        self.assertIn("deepRequired", quote_in)
        out_input_fee = c.get_quote_quantity_out_input_fee("DEEP_SUI", 1.0)
        self.assertIn("baseOut", out_input_fee)
        self.assertIn("quoteOut", out_input_fee)
        self.assertIn("deepRequired", out_input_fee)
        base_input_fee = c.get_base_quantity_out_input_fee("DEEP_SUI", 1.0)
        self.assertIn("baseOut", base_input_fee)
        self.assertIn("quoteOut", base_input_fee)
        self.assertIn("deepRequired", base_input_fee)
        both_input_fee = c.get_quantity_out_input_fee("DEEP_SUI", 1.0, 1.0)
        self.assertIn("baseOut", both_input_fee)
        self.assertIn("quoteOut", both_input_fee)
        self.assertIn("deepRequired", both_input_fee)
        self.assertEqual("0x" + ("ab" * 32), c.get_margin_manager_owner("mm1"))
        self.assertEqual("0x" + ("bc" * 32), c.get_margin_manager_deepbook_pool("mm1"))
        self.assertEqual("0x" + ("cd" * 32), c.get_margin_manager_margin_pool_id("mm1"))
        borrowed = c.get_margin_manager_borrowed_shares("mm1")
        self.assertEqual("777", borrowed["baseShares"])
        self.assertEqual("888", borrowed["quoteShares"])
        self.assertEqual("999", c.get_margin_manager_borrowed_base_shares("mm1"))
        self.assertEqual("1001", c.get_margin_manager_borrowed_quote_shares("mm1"))
        self.assertTrue(c.get_margin_manager_has_base_debt("mm1"))
        self.assertEqual("0x" + ("de" * 32), c.get_margin_manager_balance_manager_id("mm1"))
        assets = c.get_margin_manager_assets("mm1")
        self.assertIn("baseAsset", assets)
        self.assertIn("quoteAsset", assets)
        debts = c.get_margin_manager_debts("mm1")
        self.assertIn("baseDebt", debts)
        self.assertIn("quoteDebt", debts)
        state = c.get_margin_manager_state("mm1")
        self.assertIn("managerId", state)
        self.assertIn("riskRatio", state)
        self.assertIn("currentPrice", state)
        states = c.get_margin_manager_states(
            {
                "0x" + ("11" * 32): "DEEP_SUI",
                "0x" + ("22" * 32): "DEEP_SUI",
            }
        )
        self.assertEqual(2, len(states))
        for st in states.values():
            self.assertIn("managerId", st)
            self.assertIn("deepbookPoolId", st)
        self.assertTrue(c.is_deepbook_pool_allowed("SUI", "0x" + ("ab" * 32)))
        self.assertTrue(c.get_margin_pool_total_supply("SUI"))
        self.assertTrue(c.get_margin_pool_supply_shares("SUI"))
        self.assertTrue(c.get_margin_pool_total_borrow("SUI"))
        self.assertTrue(c.get_margin_pool_borrow_shares("SUI"))
        self.assertEqual(1_700_000_000, c.get_margin_pool_last_update_timestamp("SUI"))
        self.assertTrue(c.get_margin_pool_supply_cap("SUI"))
        self.assertAlmostEqual(0.25, c.get_margin_pool_max_utilization_rate("SUI"))
        self.assertAlmostEqual(0.25, c.get_margin_pool_protocol_spread("SUI"))
        self.assertTrue(c.get_margin_pool_min_borrow("SUI"))
        self.assertAlmostEqual(0.25, c.get_margin_pool_interest_rate("SUI"))
        self.assertTrue(c.get_user_supply_shares("SUI", "0x" + ("ef" * 32)))
        self.assertTrue(c.get_user_supply_amount("SUI", "0x" + ("ef" * 32)))
        self.assertEqual("2", c.get_margin_manager_base_balance("mm1"))
        self.assertEqual("3", c.get_margin_manager_quote_balance("mm1"))
        self.assertEqual("4", c.get_margin_manager_deep_balance("mm1"))
        self.assertEqual(["11", "22"], c.get_conditional_order_ids("mm1"))
        self.assertEqual(4444, c.get_lowest_trigger_above_price("mm1"))
        self.assertEqual(5555, c.get_highest_trigger_below_price("mm1"))
        c.config.coins["SUI"].price_info_object_id = "0x" + ("77" * 32)
        self.assertEqual(1710000000, c.get_price_info_object_age("SUI"))
        self.assertEqual("0x" + ("77" * 32), c.get_price_info_object("SUI"))
        self.assertEqual(
            {"SUI": "0x" + ("77" * 32)},
            c.get_price_info_objects(["SUI"]),
        )

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
