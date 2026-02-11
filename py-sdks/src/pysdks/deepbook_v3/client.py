"""DeepBook v3 client (dry-run based baseline)."""

from __future__ import annotations

import base64
from dataclasses import dataclass
from typing import Any, Dict, List, Protocol

from pysdks.bcs import BCSReader
from pysdks.deepbook_v3.config import DEEP_SCALAR, FLOAT_SCALAR, DeepBookConfig
from pysdks.deepbook_v3.types import CanPlaceLimitOrderParams, CanPlaceMarketOrderParams
from pysdks.deepbook_v3.transactions import (
    BalanceManagerContract,
    DeepBookContract,
    FlashLoanContract,
    GovernanceContract,
    encode_address,
    MarginManagerContract,
    MarginRegistryContract,
    MarginTPSLContract,
    PoolProxyContract,
    Transaction,
)


class CompatibleClient(Protocol):
    def call(self, method: str, params: List[Any] | None = None) -> Dict[str, Any]:
        ...


@dataclass
class DeepBookClient:
    client: CompatibleClient
    config: DeepBookConfig

    def __post_init__(self):
        self.balance_manager = BalanceManagerContract(self.config)
        self.deepbook = DeepBookContract(self.config, self.balance_manager)
        self.governance = GovernanceContract(self.config, self.balance_manager)
        self.flash_loans = FlashLoanContract(self.config)
        self.margin_manager = MarginManagerContract(self.config)
        self.margin_registry = MarginRegistryContract(self.config)
        self.pool_proxy = PoolProxyContract(self.config)
        self.margin_tpsl = MarginTPSLContract(self.config)

    def check_manager_balance(self, manager_key: str, coin_key: str) -> Dict[str, Any]:
        tx = Transaction()
        manager = self.config.get_balance_manager(manager_key)
        coin = self.config.get_coin(coin_key)
        tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::balance_manager::balance",
            [tx.object(manager.address)],
            [coin.type],
        )
        value = self._read_u64(self._simulate(tx), 0, 0)
        return {"coinType": coin.type, "balance": value / coin.scalar}

    def whitelisted(self, pool_key: str) -> bool:
        tx = Transaction()
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::whitelisted",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )
        raw = self._return_bytes(self._simulate(tx), 0, 0)
        return len(raw) > 0 and raw[0] == 1

    def get_quote_quantity_out(self, pool_key: str, base_quantity: float) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_quote_quantity_out(tx, pool_key, base_quantity)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_out = self._read_u64(sim, 0, 0)
        quote_out = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "baseQuantity": base_quantity,
            "baseOut": base_out / base.scalar,
            "quoteOut": quote_out / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def get_quote_quantity_out_input_fee(self, pool_key: str, base_quantity: float) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_quote_quantity_out_input_fee(tx, pool_key, base_quantity)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_out = self._read_u64(sim, 0, 0)
        quote_out = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "baseQuantity": base_quantity,
            "baseOut": base_out / base.scalar,
            "quoteOut": quote_out / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def get_base_quantity_out(self, pool_key: str, quote_quantity: float) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_base_quantity_out(tx, pool_key, quote_quantity)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_out = self._read_u64(sim, 0, 0)
        quote_out = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "quoteQuantity": quote_quantity,
            "baseOut": base_out / base.scalar,
            "quoteOut": quote_out / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def get_base_quantity_out_input_fee(self, pool_key: str, quote_quantity: float) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_base_quantity_out_input_fee(tx, pool_key, quote_quantity)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_out = self._read_u64(sim, 0, 0)
        quote_out = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "quoteQuantity": quote_quantity,
            "baseOut": base_out / base.scalar,
            "quoteOut": quote_out / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def get_quantity_out(self, pool_key: str, base_quantity: float, quote_quantity: float) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_quantity_out(tx, pool_key, base_quantity, quote_quantity)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_out = self._read_u64(sim, 0, 0)
        quote_out = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "baseQuantity": base_quantity,
            "quoteQuantity": quote_quantity,
            "baseOut": base_out / base.scalar,
            "quoteOut": quote_out / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def get_quantity_out_input_fee(self, pool_key: str, base_quantity: float, quote_quantity: float) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_quantity_out_input_fee(tx, pool_key, base_quantity, quote_quantity)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_out = self._read_u64(sim, 0, 0)
        quote_out = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "baseQuantity": base_quantity,
            "quoteQuantity": quote_quantity,
            "baseOut": base_out / base.scalar,
            "quoteOut": quote_out / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def get_base_quantity_in(self, pool_key: str, target_quote_quantity: float, pay_with_deep: bool) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_base_quantity_in(tx, pool_key, target_quote_quantity, pay_with_deep)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_in = self._read_u64(sim, 0, 0)
        quote_out = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "baseIn": base_in / base.scalar,
            "quoteOut": quote_out / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def get_quote_quantity_in(self, pool_key: str, target_base_quantity: float, pay_with_deep: bool) -> Dict[str, Any]:
        tx = Transaction()
        self.deepbook.get_quote_quantity_in(tx, pool_key, target_base_quantity, pay_with_deep)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_out = self._read_u64(sim, 0, 0)
        quote_in = self._read_u64(sim, 0, 1)
        deep_required = self._read_u64(sim, 0, 2)
        return {
            "baseOut": base_out / base.scalar,
            "quoteIn": quote_in / quote.scalar,
            "deepRequired": deep_required / DEEP_SCALAR,
        }

    def mid_price(self, pool_key: str) -> float:
        tx = Transaction()
        self.deepbook.mid_price(tx, pool_key)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        value = self._read_u64(self._simulate(tx), 0, 0)
        return (value * base.scalar) / (FLOAT_SCALAR * quote.scalar)

    def get_order(self, pool_key: str, order_id: str) -> Dict[str, Any] | None:
        tx = Transaction()
        self.deepbook.get_order(tx, pool_key, order_id)
        try:
            return self._read_order(self._return_bytes(self._simulate(tx), 0, 0))
        except Exception:
            return None

    def get_order_normalized(self, pool_key: str, order_id: str) -> Dict[str, Any] | None:
        order = self.get_order(pool_key, order_id)
        if not order:
            return None

        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        decoded = self.decode_order_id(int(order["order_id"]))

        normalized = dict(order)
        normalized["quantity"] = f"{(int(order['quantity']) / base.scalar):.9f}"
        normalized["filled_quantity"] = f"{(int(order['filled_quantity']) / base.scalar):.9f}"
        normalized["order_deep_price"] = {
            **order["order_deep_price"],
            "deep_per_asset": f"{(int(order['order_deep_price']['deep_per_asset']) / DEEP_SCALAR):.9f}",
        }
        normalized["is_bid"] = decoded["isBid"]
        normalized["normalized_price"] = f"{(int(decoded['price']) * base.scalar / quote.scalar / FLOAT_SCALAR):.9f}"
        return normalized

    def get_orders(self, pool_key: str, order_ids: List[str]) -> List[Dict[str, Any]] | None:
        tx = Transaction()
        self.deepbook.get_orders(tx, pool_key, order_ids)
        try:
            return self._read_vec_order(self._return_bytes(self._simulate(tx), 0, 0))
        except Exception:
            return None

    def account_open_orders(self, pool_key: str, manager_key: str) -> List[str]:
        tx = Transaction()
        self.deepbook.account_open_orders(tx, pool_key, manager_key)
        return [str(v) for v in self._read_vec_u128(self._return_bytes(self._simulate(tx), 0, 0))]

    def get_pool_id_by_assets(self, base_type: str, quote_type: str) -> str:
        tx = Transaction()
        self.deepbook.get_pool_id_by_assets(tx, base_type, quote_type)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_balance_manager_ids(self, owner: str) -> List[str]:
        tx = Transaction()
        self.deepbook.get_balance_manager_ids(tx, owner)
        return self._read_vec_address(self._return_bytes(self._simulate(tx), 0, 0))

    def balance_manager_referral_owner(self, referral: str) -> str:
        tx = Transaction()
        self.balance_manager.balance_manager_referral_owner(tx, referral)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_pool_referral_balances(self, pool_key: str, referral: str) -> Dict[str, float]:
        tx = Transaction()
        self.deepbook.get_pool_referral_balances(tx, pool_key, referral)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_balance = self._read_u64(sim, 0, 0)
        quote_balance = self._read_u64(sim, 0, 1)
        deep_balance = self._read_u64(sim, 0, 2)
        return {
            "base": base_balance / base.scalar,
            "quote": quote_balance / quote.scalar,
            "deep": deep_balance / DEEP_SCALAR,
        }

    def balance_manager_referral_pool_id(self, referral: str) -> str:
        tx = Transaction()
        self.balance_manager.balance_manager_referral_pool_id(tx, referral)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def pool_referral_multiplier(self, pool_key: str, referral: str) -> float:
        tx = Transaction()
        self.deepbook.pool_referral_multiplier(tx, pool_key, referral)
        return self._read_u64(self._simulate(tx), 0, 0) / FLOAT_SCALAR

    def get_allowed_maintainers(self) -> List[str]:
        tx = Transaction()
        self.margin_registry.allowed_maintainers(tx)
        return self._read_vec_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_allowed_pause_caps(self) -> List[str]:
        tx = Transaction()
        self.margin_registry.allowed_pause_caps(tx)
        return self._read_vec_address(self._return_bytes(self._simulate(tx), 0, 0))

    def is_pool_enabled_for_margin(self, pool_key: str) -> bool:
        tx = Transaction()
        self.margin_registry.pool_enabled(tx, pool_key)
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def get_margin_pool_id(self, coin_key: str) -> str:
        tx = Transaction()
        self.margin_registry.get_margin_pool_id(tx, coin_key)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_deepbook_pool_margin_pool_ids(self, pool_key: str) -> Dict[str, str]:
        tx = Transaction()
        self.margin_registry.get_deepbook_pool_margin_pool_ids(tx, pool_key)
        raw = self._return_bytes(self._simulate(tx), 0, 0)
        r = BCSReader(raw)
        return {
            "baseMarginPoolId": self._read_address(r.read_bytes(32)),
            "quoteMarginPoolId": self._read_address(r.read_bytes(32)),
        }

    def get_margin_manager_ids_for_owner(self, owner: str) -> List[str]:
        tx = Transaction()
        self.margin_registry.get_margin_manager_ids(tx, owner)
        return self._read_vec_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_base_margin_pool_id(self, pool_key: str) -> str:
        tx = Transaction()
        self.margin_registry.base_margin_pool_id(tx, pool_key)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_quote_margin_pool_id(self, pool_key: str) -> str:
        tx = Transaction()
        self.margin_registry.quote_margin_pool_id(tx, pool_key)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_min_withdraw_risk_ratio(self, pool_key: str) -> float:
        tx = Transaction()
        self.margin_registry.min_withdraw_risk_ratio(tx, pool_key)
        return self._read_u64(self._simulate(tx), 0, 0) / FLOAT_SCALAR

    def get_min_borrow_risk_ratio(self, pool_key: str) -> float:
        tx = Transaction()
        self.margin_registry.min_borrow_risk_ratio(tx, pool_key)
        return self._read_u64(self._simulate(tx), 0, 0) / FLOAT_SCALAR

    def get_liquidation_risk_ratio(self, pool_key: str) -> float:
        tx = Transaction()
        self.margin_registry.liquidation_risk_ratio(tx, pool_key)
        return self._read_u64(self._simulate(tx), 0, 0) / FLOAT_SCALAR

    def get_target_liquidation_risk_ratio(self, pool_key: str) -> float:
        tx = Transaction()
        self.margin_registry.target_liquidation_risk_ratio(tx, pool_key)
        return self._read_u64(self._simulate(tx), 0, 0) / FLOAT_SCALAR

    def get_user_liquidation_reward(self, pool_key: str) -> float:
        tx = Transaction()
        self.margin_registry.user_liquidation_reward(tx, pool_key)
        return self._read_u64(self._simulate(tx), 0, 0) / FLOAT_SCALAR

    def get_pool_liquidation_reward(self, pool_key: str) -> float:
        tx = Transaction()
        self.margin_registry.pool_liquidation_reward(tx, pool_key)
        return self._read_u64(self._simulate(tx), 0, 0) / FLOAT_SCALAR

    def is_deepbook_pool_allowed(self, coin_key: str, deepbook_pool_id: str) -> bool:
        tx = Transaction()
        coin = self.config.get_coin(coin_key)
        margin_pool_id = self.margin_registry.get_margin_pool_id(tx, coin_key)
        tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_pool::deepbook_pool_allowed",
            [margin_pool_id, tx.pure(encode_address(deepbook_pool_id))],
            [coin.type],
        )
        return BCSReader(self._return_bytes(self._simulate(tx), 1, 0)).read_bool()

    def get_margin_pool_total_supply(self, coin_key: str, decimals: int = 6) -> str:
        return self._margin_pool_amount_view(coin_key, "total_supply", decimals)

    def get_margin_pool_supply_shares(self, coin_key: str, decimals: int = 6) -> str:
        return self._margin_pool_amount_view(coin_key, "supply_shares", decimals)

    def get_margin_pool_total_borrow(self, coin_key: str, decimals: int = 6) -> str:
        return self._margin_pool_amount_view(coin_key, "total_borrow", decimals)

    def get_margin_pool_borrow_shares(self, coin_key: str, decimals: int = 6) -> str:
        return self._margin_pool_amount_view(coin_key, "borrow_shares", decimals)

    def get_margin_pool_last_update_timestamp(self, coin_key: str) -> int:
        return self._margin_pool_u64_view(coin_key, "last_update_timestamp")

    def get_margin_pool_supply_cap(self, coin_key: str, decimals: int = 6) -> str:
        return self._margin_pool_amount_view(coin_key, "supply_cap", decimals)

    def get_margin_pool_max_utilization_rate(self, coin_key: str) -> float:
        return self._margin_pool_u64_view(coin_key, "max_utilization_rate") / FLOAT_SCALAR

    def get_margin_pool_protocol_spread(self, coin_key: str) -> float:
        return self._margin_pool_u64_view(coin_key, "protocol_spread") / FLOAT_SCALAR

    def get_margin_pool_min_borrow(self, coin_key: str, decimals: int = 6) -> str:
        return self._margin_pool_amount_view(coin_key, "min_borrow", decimals)

    def get_margin_pool_interest_rate(self, coin_key: str) -> float:
        return self._margin_pool_u64_view(coin_key, "interest_rate") / FLOAT_SCALAR

    def get_user_supply_shares(self, coin_key: str, supplier_cap_id: str, decimals: int = 6) -> str:
        return self._margin_pool_user_u64_view(coin_key, "user_supply_shares", supplier_cap_id, decimals, include_clock=False)

    def get_user_supply_amount(self, coin_key: str, supplier_cap_id: str, decimals: int = 6) -> str:
        return self._margin_pool_user_u64_view(coin_key, "user_supply_amount", supplier_cap_id, decimals, include_clock=True)

    def get_balance_manager_referral_id(self, manager_key: str, pool_key: str) -> str | None:
        tx = Transaction()
        self.balance_manager.get_balance_manager_referral_id(tx, manager_key, pool_key)
        try:
            return self._read_option_address(self._return_bytes(self._simulate(tx), 0, 0))
        except Exception:
            return None

    def vault_balances(self, pool_key: str) -> Dict[str, float]:
        tx = Transaction()
        self.deepbook.vault_balances(tx, pool_key)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_in_vault = self._read_u64(sim, 0, 0)
        quote_in_vault = self._read_u64(sim, 0, 1)
        deep_in_vault = self._read_u64(sim, 0, 2)
        return {
            "base": base_in_vault / base.scalar,
            "quote": quote_in_vault / quote.scalar,
            "deep": deep_in_vault / DEEP_SCALAR,
        }

    def pool_trade_params(self, pool_key: str) -> Dict[str, float]:
        tx = Transaction()
        self.deepbook.pool_trade_params(tx, pool_key)
        sim = self._simulate(tx)
        taker_fee = self._read_u64(sim, 0, 0)
        maker_fee = self._read_u64(sim, 0, 1)
        stake_required = self._read_u64(sim, 0, 2)
        return {
            "takerFee": taker_fee / FLOAT_SCALAR,
            "makerFee": maker_fee / FLOAT_SCALAR,
            "stakeRequired": stake_required / DEEP_SCALAR,
        }

    def pool_trade_params_next(self, pool_key: str) -> Dict[str, float]:
        tx = Transaction()
        self.deepbook.pool_trade_params_next(tx, pool_key)
        sim = self._simulate(tx)
        taker_fee = self._read_u64(sim, 0, 0)
        maker_fee = self._read_u64(sim, 0, 1)
        stake_required = self._read_u64(sim, 0, 2)
        return {
            "takerFee": taker_fee / FLOAT_SCALAR,
            "makerFee": maker_fee / FLOAT_SCALAR,
            "stakeRequired": stake_required / DEEP_SCALAR,
        }

    def pool_book_params(self, pool_key: str) -> Dict[str, float]:
        tx = Transaction()
        self.deepbook.pool_book_params(tx, pool_key)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        tick_size = self._read_u64(sim, 0, 0)
        lot_size = self._read_u64(sim, 0, 1)
        min_size = self._read_u64(sim, 0, 2)
        return {
            "tickSize": (tick_size * base.scalar) / (FLOAT_SCALAR * quote.scalar),
            "lotSize": lot_size / base.scalar,
            "minSize": min_size / base.scalar,
        }

    def locked_balance(self, pool_key: str, manager_key: str) -> Dict[str, float]:
        tx = Transaction()
        self.deepbook.locked_balance(tx, pool_key, manager_key)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_locked = self._read_u64(sim, 0, 0)
        quote_locked = self._read_u64(sim, 0, 1)
        deep_locked = self._read_u64(sim, 0, 2)
        return {
            "base": base_locked / base.scalar,
            "quote": quote_locked / quote.scalar,
            "deep": deep_locked / DEEP_SCALAR,
        }

    def get_level2_range(self, pool_key: str, price_low: float, price_high: float, is_bid: bool) -> Dict[str, List[float]]:
        tx = Transaction()
        self.deepbook.get_level2_range(tx, pool_key, price_low, price_high, is_bid)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        parsed_prices = self._read_vec_u64(self._return_bytes(sim, 0, 0))
        parsed_quantities = self._read_vec_u64(self._return_bytes(sim, 0, 1))
        return {
            "prices": [((v / FLOAT_SCALAR / quote.scalar) * base.scalar) for v in parsed_prices],
            "quantities": [(v / base.scalar) for v in parsed_quantities],
        }

    def get_level2_ticks_from_mid(self, pool_key: str, ticks: int) -> Dict[str, List[float]]:
        tx = Transaction()
        self.deepbook.get_level2_ticks_from_mid(tx, pool_key, ticks)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        bid_prices = self._read_vec_u64(self._return_bytes(sim, 0, 0))
        bid_quantities = self._read_vec_u64(self._return_bytes(sim, 0, 1))
        ask_prices = self._read_vec_u64(self._return_bytes(sim, 0, 2))
        ask_quantities = self._read_vec_u64(self._return_bytes(sim, 0, 3))
        return {
            "bid_prices": [((v / FLOAT_SCALAR / quote.scalar) * base.scalar) for v in bid_prices],
            "bid_quantities": [(v / base.scalar) for v in bid_quantities],
            "ask_prices": [((v / FLOAT_SCALAR / quote.scalar) * base.scalar) for v in ask_prices],
            "ask_quantities": [(v / base.scalar) for v in ask_quantities],
        }

    def get_pool_deep_price(self, pool_key: str) -> Dict[str, float | bool]:
        tx = Transaction()
        self.deepbook.get_pool_deep_price(tx, pool_key)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        deep = self.config.get_coin("DEEP")
        raw = self._return_bytes(self._simulate(tx), 0, 0)
        asset_is_base, deep_per_asset = self._read_order_deep_price(raw)
        if asset_is_base:
            return {
                "asset_is_base": True,
                "deep_per_base": ((deep_per_asset / FLOAT_SCALAR) * base.scalar) / deep.scalar,
            }
        return {
            "asset_is_base": False,
            "deep_per_quote": ((deep_per_asset / FLOAT_SCALAR) * quote.scalar) / deep.scalar,
        }

    def decode_order_id(self, encoded_order_id: int) -> Dict[str, int | bool]:
        n = int(encoded_order_id)
        is_bid = (n >> 127) == 0
        price = (n >> 64) & ((1 << 63) - 1)
        order_id = n & ((1 << 64) - 1)
        return {"isBid": is_bid, "price": price, "orderId": order_id}

    def account(self, pool_key: str, manager_key: str) -> str:
        # Baseline parity: return raw account bytes as base64. Full struct parsing can be added incrementally.
        tx = Transaction()
        self.deepbook.account(tx, pool_key, manager_key)
        return base64.b64encode(self._return_bytes(self._simulate(tx), 0, 0)).decode()

    def get_account_order_details(self, pool_key: str, manager_key: str) -> List[Dict[str, Any]]:
        tx = Transaction()
        self.deepbook.get_account_order_details(tx, pool_key, manager_key)
        try:
            return self._read_vec_order(self._return_bytes(self._simulate(tx), 0, 0))
        except Exception:
            return []

    def account_exists(self, pool_key: str, manager_key: str) -> bool:
        tx = Transaction()
        self.deepbook.account_exists(tx, pool_key, manager_key)
        raw = self._return_bytes(self._simulate(tx), 0, 0)
        return BCSReader(raw).read_bool()

    def get_order_deep_required(self, pool_key: str, base_quantity: float, price: float) -> Dict[str, float]:
        tx = Transaction()
        self.deepbook.get_order_deep_required(tx, pool_key, base_quantity, price)
        sim = self._simulate(tx)
        deep_required_taker = self._read_u64(sim, 0, 0)
        deep_required_maker = self._read_u64(sim, 0, 1)
        return {
            "deepRequiredTaker": deep_required_taker / DEEP_SCALAR,
            "deepRequiredMaker": deep_required_maker / DEEP_SCALAR,
        }

    def quorum(self, pool_key: str) -> float:
        tx = Transaction()
        self.deepbook.quorum(tx, pool_key)
        return self._read_u64(self._simulate(tx), 0, 0) / DEEP_SCALAR

    def pool_id(self, pool_key: str) -> str:
        tx = Transaction()
        self.deepbook.pool_id(tx, pool_key)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def stable_pool(self, pool_key: str) -> bool:
        tx = Transaction()
        self.deepbook.stable_pool(tx, pool_key)
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def registered_pool(self, pool_key: str) -> bool:
        tx = Transaction()
        self.deepbook.registered_pool(tx, pool_key)
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def can_place_market_order(
        self,
        pool_key: str,
        balance_manager_key: str,
        quantity: float,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> bool:
        tx = Transaction()
        self.deepbook.can_place_market_order(
            tx,
            CanPlaceMarketOrderParams(
                pool_key=pool_key,
                balance_manager_key=balance_manager_key,
                quantity=quantity,
                is_bid=is_bid,
                pay_with_deep=pay_with_deep,
            ),
        )
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def can_place_limit_order(
        self,
        pool_key: str,
        balance_manager_key: str,
        price: float,
        quantity: float,
        is_bid: bool,
        pay_with_deep: bool,
        expire_timestamp: int,
    ) -> bool:
        tx = Transaction()
        self.deepbook.can_place_limit_order(
            tx,
            CanPlaceLimitOrderParams(
                pool_key=pool_key,
                balance_manager_key=balance_manager_key,
                price=price,
                quantity=quantity,
                is_bid=is_bid,
                pay_with_deep=pay_with_deep,
                expire_timestamp=expire_timestamp,
            ),
        )
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def check_market_order_params(self, pool_key: str, quantity: float) -> bool:
        tx = Transaction()
        self.deepbook.check_market_order_params(tx, pool_key, quantity)
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def check_limit_order_params(self, pool_key: str, price: float, quantity: float, expire_timestamp: int) -> bool:
        tx = Transaction()
        self.deepbook.check_limit_order_params(tx, pool_key, price, quantity, expire_timestamp)
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def get_margin_account_order_details(self, margin_manager_key: str) -> str:
        tx = Transaction()
        self.margin_manager.get_margin_account_order_details(tx, margin_manager_key)
        return base64.b64encode(self._return_bytes(self._simulate(tx), 1, 0)).decode()

    def get_margin_manager_owner(self, margin_manager_key: str) -> str:
        tx = Transaction()
        self.margin_manager.owner(tx, margin_manager_key)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_margin_manager_deepbook_pool(self, margin_manager_key: str) -> str:
        tx = Transaction()
        self.margin_manager.deepbook_pool(tx, margin_manager_key)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_margin_manager_margin_pool_id(self, margin_manager_key: str) -> str | None:
        tx = Transaction()
        self.margin_manager.margin_pool_id(tx, margin_manager_key)
        return self._read_option_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_margin_manager_borrowed_shares(self, margin_manager_key: str) -> Dict[str, str]:
        tx = Transaction()
        self.margin_manager.borrowed_shares(tx, margin_manager_key)
        sim = self._simulate(tx)
        return {
            "baseShares": str(self._read_u64(sim, 0, 0)),
            "quoteShares": str(self._read_u64(sim, 0, 1)),
        }

    def get_margin_manager_borrowed_base_shares(self, margin_manager_key: str) -> str:
        tx = Transaction()
        self.margin_manager.borrowed_base_shares(tx, margin_manager_key)
        return str(self._read_u64(self._simulate(tx), 0, 0))

    def get_margin_manager_borrowed_quote_shares(self, margin_manager_key: str) -> str:
        tx = Transaction()
        self.margin_manager.borrowed_quote_shares(tx, margin_manager_key)
        return str(self._read_u64(self._simulate(tx), 0, 0))

    def get_margin_manager_has_base_debt(self, margin_manager_key: str) -> bool:
        tx = Transaction()
        self.margin_manager.has_base_debt(tx, margin_manager_key)
        return BCSReader(self._return_bytes(self._simulate(tx), 0, 0)).read_bool()

    def get_margin_manager_balance_manager_id(self, margin_manager_key: str) -> str:
        tx = Transaction()
        self.margin_manager.balance_manager(tx, margin_manager_key)
        return self._read_address(self._return_bytes(self._simulate(tx), 0, 0))

    def get_margin_manager_base_balance(self, margin_manager_key: str, decimals: int = 9) -> str:
        tx = Transaction()
        self.margin_manager.base_balance(tx, margin_manager_key)
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        coin = self.config.get_coin(pool.base_coin)
        return self._format_token_amount(self._read_u64(self._simulate(tx), 0, 0), coin.scalar, decimals)

    def get_margin_manager_quote_balance(self, margin_manager_key: str, decimals: int = 9) -> str:
        tx = Transaction()
        self.margin_manager.quote_balance(tx, margin_manager_key)
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        coin = self.config.get_coin(pool.quote_coin)
        return self._format_token_amount(self._read_u64(self._simulate(tx), 0, 0), coin.scalar, decimals)

    def get_margin_manager_deep_balance(self, margin_manager_key: str, decimals: int = 6) -> str:
        tx = Transaction()
        self.margin_manager.deep_balance(tx, margin_manager_key)
        coin = self.config.get_coin("DEEP")
        return self._format_token_amount(self._read_u64(self._simulate(tx), 0, 0), coin.scalar, decimals)

    def get_conditional_order_ids(self, margin_manager_key: str) -> List[str]:
        tx = Transaction()
        self.margin_tpsl.conditional_order_ids(tx, margin_manager_key)
        return [str(v) for v in self._read_vec_u64(self._return_bytes(self._simulate(tx), 0, 0))]

    def get_lowest_trigger_above_price(self, margin_manager_key: str) -> int:
        tx = Transaction()
        self.margin_tpsl.lowest_trigger_above_price(tx, margin_manager_key)
        return self._read_u64(self._simulate(tx), 0, 0)

    def get_highest_trigger_below_price(self, margin_manager_key: str) -> int:
        tx = Transaction()
        self.margin_tpsl.highest_trigger_below_price(tx, margin_manager_key)
        return self._read_u64(self._simulate(tx), 0, 0)

    def get_price_info_object_age(self, coin_key: str) -> int:
        coin = self.config.get_coin(coin_key)
        if not coin.price_info_object_id:
            raise ValueError(f"price_info_object_id not configured for coin: {coin_key}")
        out = self.client.call("sui_getObject", [coin.price_info_object_id, {"showContent": True}])
        arrival = self._extract_arrival_time(out)
        if arrival is None:
            raise ValueError(f"arrival_time not found in price info object for coin: {coin_key}")
        return int(arrival)

    def get_price_info_object(self, coin_key: str) -> str:
        coin = self.config.get_coin(coin_key)
        if not coin.price_info_object_id:
            raise ValueError(f"price_info_object_id not configured for coin: {coin_key}")
        # Read-path baseline: return configured object id. Staleness handling can be layered with Pyth update flow.
        _ = self.get_price_info_object_age(coin_key)
        return coin.price_info_object_id

    def get_price_info_objects(self, coin_keys: List[str]) -> Dict[str, str]:
        out: Dict[str, str] = {}
        for coin_key in coin_keys:
            out[coin_key] = self.get_price_info_object(coin_key)
        return out

    def get_margin_manager_assets(self, margin_manager_key: str, decimals: int = 6) -> Dict[str, str]:
        tx = Transaction()
        self.margin_manager.calculate_assets(tx, margin_manager_key)
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base_coin = self.config.get_coin(pool.base_coin)
        quote_coin = self.config.get_coin(pool.quote_coin)
        sim = self._simulate(tx)
        base_asset = self._format_token_amount(self._read_u64(sim, 0, 0), base_coin.scalar, decimals)
        quote_asset = self._format_token_amount(self._read_u64(sim, 0, 1), quote_coin.scalar, decimals)
        return {"baseAsset": base_asset, "quoteAsset": quote_asset}

    def get_margin_manager_debts(self, margin_manager_key: str, decimals: int = 6) -> Dict[str, str]:
        has_base_debt = self.get_margin_manager_has_base_debt(margin_manager_key)
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        debt_coin_key = pool.base_coin if has_base_debt else pool.quote_coin

        tx = Transaction()
        self.margin_manager.calculate_debts(tx, margin_manager_key, debt_coin_key)
        debt_coin = self.config.get_coin(debt_coin_key)
        sim = self._simulate(tx)
        # calculate_debts is the second command (first resolves margin pool)
        base_debt = self._format_token_amount(self._read_u64(sim, 1, 0), debt_coin.scalar, decimals)
        quote_debt = self._format_token_amount(self._read_u64(sim, 1, 1), debt_coin.scalar, decimals)
        return {"baseDebt": base_debt, "quoteDebt": quote_debt}

    def get_margin_manager_state(self, margin_manager_key: str, decimals: int = 6) -> Dict[str, Any]:
        tx = Transaction()
        self.margin_manager.manager_state(tx, margin_manager_key)
        sim = self._simulate(tx)

        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base_coin = self.config.get_coin(pool.base_coin)
        quote_coin = self.config.get_coin(pool.quote_coin)

        manager_id = self._read_address(self._return_bytes(sim, 0, 0))
        deepbook_pool_id = self._read_address(self._return_bytes(sim, 0, 1))
        risk_ratio = self._read_u64(sim, 0, 2) / FLOAT_SCALAR
        base_asset = self._format_token_amount(self._read_u64(sim, 0, 3), base_coin.scalar, decimals)
        quote_asset = self._format_token_amount(self._read_u64(sim, 0, 4), quote_coin.scalar, decimals)
        base_debt = self._format_token_amount(self._read_u64(sim, 0, 5), base_coin.scalar, decimals)
        quote_debt = self._format_token_amount(self._read_u64(sim, 0, 6), quote_coin.scalar, decimals)
        base_pyth_price = str(self._read_u64(sim, 0, 7))
        base_pyth_decimals = BCSReader(self._return_bytes(sim, 0, 8)).read_u8()
        quote_pyth_price = str(self._read_u64(sim, 0, 9))
        quote_pyth_decimals = BCSReader(self._return_bytes(sim, 0, 10)).read_u8()
        current_price = self._read_u64(sim, 0, 11)
        lowest_trigger_above_price = self._read_u64(sim, 0, 12)
        highest_trigger_below_price = self._read_u64(sim, 0, 13)

        return {
            "managerId": manager_id,
            "deepbookPoolId": deepbook_pool_id,
            "riskRatio": risk_ratio,
            "baseAsset": base_asset,
            "quoteAsset": quote_asset,
            "baseDebt": base_debt,
            "quoteDebt": quote_debt,
            "basePythPrice": base_pyth_price,
            "basePythDecimals": base_pyth_decimals,
            "quotePythPrice": quote_pyth_price,
            "quotePythDecimals": quote_pyth_decimals,
            "currentPrice": current_price,
            "lowestTriggerAbovePrice": lowest_trigger_above_price,
            "highestTriggerBelowPrice": highest_trigger_below_price,
        }

    def get_margin_manager_states(self, margin_managers: Dict[str, str], decimals: int = 6) -> Dict[str, Dict[str, Any]]:
        entries = list(margin_managers.items())
        if not entries:
            return {}

        tx = Transaction()
        for manager_id, pool_key in entries:
            self.margin_manager.manager_state_by_id(tx, pool_key, manager_id)
        sim = self._simulate(tx)

        results: Dict[str, Dict[str, Any]] = {}
        for i, (_manager_id_input, pool_key) in enumerate(entries):
            pool = self.config.get_pool(pool_key)
            base_coin = self.config.get_coin(pool.base_coin)
            quote_coin = self.config.get_coin(pool.quote_coin)
            manager_id = self._read_address(self._return_bytes(sim, i, 0))
            results[manager_id] = {
                "managerId": manager_id,
                "deepbookPoolId": self._read_address(self._return_bytes(sim, i, 1)),
                "riskRatio": self._read_u64(sim, i, 2) / FLOAT_SCALAR,
                "baseAsset": self._format_token_amount(self._read_u64(sim, i, 3), base_coin.scalar, decimals),
                "quoteAsset": self._format_token_amount(self._read_u64(sim, i, 4), quote_coin.scalar, decimals),
                "baseDebt": self._format_token_amount(self._read_u64(sim, i, 5), base_coin.scalar, decimals),
                "quoteDebt": self._format_token_amount(self._read_u64(sim, i, 6), quote_coin.scalar, decimals),
                "basePythPrice": str(self._read_u64(sim, i, 7)),
                "basePythDecimals": BCSReader(self._return_bytes(sim, i, 8)).read_u8(),
                "quotePythPrice": str(self._read_u64(sim, i, 9)),
                "quotePythDecimals": BCSReader(self._return_bytes(sim, i, 10)).read_u8(),
                "currentPrice": self._read_u64(sim, i, 11),
                "lowestTriggerAbovePrice": self._read_u64(sim, i, 12),
                "highestTriggerBelowPrice": self._read_u64(sim, i, 13),
            }
        return results

    def _simulate(self, tx: Transaction) -> Dict[str, Any]:
        out = self.client.call("sui_dryRunTransactionBlock", [tx.commands])
        if isinstance(out, dict) and isinstance(out.get("result"), dict):
            return out["result"]
        return out

    def _return_bytes(self, sim: Dict[str, Any], command_index: int, return_index: int) -> bytes:
        command_results = sim.get("commandResults")
        if not isinstance(command_results, list) or len(command_results) <= command_index:
            raise ValueError(f"missing commandResults[{command_index}]")
        command_result = command_results[command_index]
        if not isinstance(command_result, dict):
            raise ValueError("invalid command result")
        return_values = command_result.get("returnValues")
        if not isinstance(return_values, list) or len(return_values) <= return_index:
            raise ValueError(f"missing returnValues[{return_index}]")
        ret = return_values[return_index]
        if not isinstance(ret, dict) or not isinstance(ret.get("bcs"), str):
            raise ValueError("invalid bcs return value")
        return base64.b64decode(ret["bcs"])

    def _read_u64(self, sim: Dict[str, Any], command_index: int, return_index: int) -> int:
        raw = self._return_bytes(sim, command_index, return_index)
        return BCSReader(raw).read_u64()

    def _read_vec_u64(self, raw: bytes) -> List[int]:
        r = BCSReader(raw)
        length = r.read_uleb128()
        out: List[int] = []
        for _ in range(length):
            out.append(r.read_u64())
        return out

    def _read_vec_u128(self, raw: bytes) -> List[int]:
        r = BCSReader(raw)
        length = r.read_uleb128()
        out: List[int] = []
        for _ in range(length):
            out.append(int.from_bytes(r.read_bytes(16), "little"))
        return out

    def _read_address(self, raw: bytes) -> str:
        if len(raw) != 32:
            raise ValueError("invalid address bytes")
        return "0x" + raw.hex()

    def _read_vec_address(self, raw: bytes) -> List[str]:
        r = BCSReader(raw)
        length = r.read_uleb128()
        out: List[str] = []
        for _ in range(length):
            out.append(self._read_address(r.read_bytes(32)))
        return out

    def _read_order_deep_price(self, raw: bytes) -> tuple[bool, int]:
        r = BCSReader(raw)
        return (r.read_bool(), r.read_u64())

    def _read_option_address(self, raw: bytes) -> str | None:
        r = BCSReader(raw)
        has_value = r.read_bool()
        if not has_value:
            return None
        return self._read_address(r.read_bytes(32))

    def _read_order(self, raw: bytes) -> Dict[str, Any]:
        r = BCSReader(raw)
        return self._read_order_from_reader(r)

    def _read_order_from_reader(self, r: BCSReader) -> Dict[str, Any]:
        balance_manager_id = self._read_address(r.read_bytes(32))
        order_id = int.from_bytes(r.read_bytes(16), "little")
        client_order_id = r.read_u64()
        quantity = r.read_u64()
        filled_quantity = r.read_u64()
        fee_is_deep = r.read_bool()
        order_deep_price = {
            "asset_is_base": r.read_bool(),
            "deep_per_asset": r.read_u64(),
        }
        return {
            "balance_manager_id": balance_manager_id,
            "order_id": order_id,
            "client_order_id": client_order_id,
            "quantity": quantity,
            "filled_quantity": filled_quantity,
            "fee_is_deep": fee_is_deep,
            "order_deep_price": order_deep_price,
            "epoch": r.read_u64(),
            "status": r.read_u8(),
            "expire_timestamp": r.read_u64(),
        }

    def _read_vec_order(self, raw: bytes) -> List[Dict[str, Any]]:
        r = BCSReader(raw)
        length = r.read_uleb128()
        out: List[Dict[str, Any]] = []
        for _ in range(length):
            out.append(self._read_order_from_reader(r))
        return out

    def _format_token_amount(self, raw_amount: int, scalar: float, decimals: int) -> str:
        if scalar <= 0:
            raise ValueError("invalid scalar")
        scaled = raw_amount / scalar
        out = f"{scaled:.{max(0, decimals)}f}"
        out = out.rstrip("0").rstrip(".")
        return out or "0"

    def _extract_arrival_time(self, value: Any) -> int | None:
        if isinstance(value, dict):
            for key, val in value.items():
                if key == "arrival_time":
                    if isinstance(val, int):
                        return val
                    if isinstance(val, str) and val.isdigit():
                        return int(val)
                nested = self._extract_arrival_time(val)
                if nested is not None:
                    return nested
        elif isinstance(value, list):
            for item in value:
                nested = self._extract_arrival_time(item)
                if nested is not None:
                    return nested
        return None

    def _margin_pool_u64_view(self, coin_key: str, fn_name: str) -> int:
        tx = Transaction()
        coin = self.config.get_coin(coin_key)
        margin_pool_id = self.margin_registry.get_margin_pool_id(tx, coin_key)
        tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_pool::{fn_name}",
            [margin_pool_id],
            [coin.type],
        )
        return self._read_u64(self._simulate(tx), 1, 0)

    def _margin_pool_amount_view(self, coin_key: str, fn_name: str, decimals: int) -> str:
        coin = self.config.get_coin(coin_key)
        raw = self._margin_pool_u64_view(coin_key, fn_name)
        return self._format_token_amount(raw, coin.scalar, decimals)

    def _margin_pool_user_u64_view(
        self,
        coin_key: str,
        fn_name: str,
        supplier_cap_id: str,
        decimals: int,
        include_clock: bool,
    ) -> str:
        tx = Transaction()
        coin = self.config.get_coin(coin_key)
        margin_pool_id = self.margin_registry.get_margin_pool_id(tx, coin_key)
        args = [margin_pool_id, tx.pure(encode_address(supplier_cap_id))]
        if include_clock:
            args.append(tx.object("0x6"))
        tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_pool::{fn_name}",
            args,
            [coin.type],
        )
        raw = self._read_u64(self._simulate(tx), 1, 0)
        return self._format_token_amount(raw, coin.scalar, decimals)
