"""DeepBook v3 client (dry-run based baseline)."""

from __future__ import annotations

import base64
from dataclasses import dataclass
from typing import Any, Dict, List, Protocol

from pysdks.bcs import BCSReader
from pysdks.deepbook_v3.config import DEEP_SCALAR, FLOAT_SCALAR, DeepBookConfig
from pysdks.deepbook_v3.transactions import (
    BalanceManagerContract,
    DeepBookContract,
    FlashLoanContract,
    GovernanceContract,
    MarginManagerContract,
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

    def mid_price(self, pool_key: str) -> float:
        tx = Transaction()
        self.deepbook.mid_price(tx, pool_key)
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        value = self._read_u64(self._simulate(tx), 0, 0)
        return (value * base.scalar) / (FLOAT_SCALAR * quote.scalar)

    def get_order(self, pool_key: str, order_id: str) -> str:
        tx = Transaction()
        self.deepbook.get_order(tx, pool_key, order_id)
        return base64.b64encode(self._return_bytes(self._simulate(tx), 0, 0)).decode()

    def get_margin_account_order_details(self, margin_manager_key: str) -> str:
        tx = Transaction()
        self.margin_manager.get_margin_account_order_details(tx, margin_manager_key)
        return base64.b64encode(self._return_bytes(self._simulate(tx), 1, 0)).decode()

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
