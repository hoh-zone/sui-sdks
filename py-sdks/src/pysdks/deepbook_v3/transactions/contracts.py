"""DeepBook v3 contracts (baseline)."""

from __future__ import annotations

from dataclasses import dataclass

from pysdks.deepbook_v3.config import FLOAT_SCALAR, MAX_TIMESTAMP, DeepBookConfig
from pysdks.deepbook_v3.transactions.encode import encode_bool, encode_u64, encode_u128, encode_vec_u128
from pysdks.deepbook_v3.transactions.transaction import Transaction
from pysdks.deepbook_v3.types import (
    CanPlaceLimitOrderParams,
    PlaceLimitOrderParams,
    PlaceMarginLimitOrderParams,
    PlaceMarginMarketOrderParams,
    PlaceMarketOrderParams,
)


@dataclass
class BalanceManagerContract:
    config: DeepBookConfig

    def generate_proof(self, tx: Transaction, manager_key: str):
        manager = self.config.get_balance_manager(manager_key)
        if manager.trade_cap:
            return tx.move_call(
                f"{self.config.package_ids.deepbook_package_id}::balance_manager::generate_proof_as_trader",
                [tx.object(manager.address), tx.object(manager.trade_cap)],
                [],
            )
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::balance_manager::generate_proof_as_owner",
            [tx.object(manager.address)],
            [],
        )


@dataclass
class DeepBookContract:
    config: DeepBookConfig
    balance_manager: BalanceManagerContract

    def place_limit_order(self, tx: Transaction, params: PlaceLimitOrderParams):
        pool = self.config.get_pool(params.pool_key)
        manager = self.config.get_balance_manager(params.balance_manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        price = round((params.price * FLOAT_SCALAR * quote.scalar) / base.scalar)
        quantity = round(params.quantity * base.scalar)
        expiration = params.expiration or MAX_TIMESTAMP
        proof = self.balance_manager.generate_proof(tx, params.balance_manager_key)

        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::place_limit_order",
            [
                tx.object(pool.address),
                tx.object(manager.address),
                proof,
                tx.pure(encode_u64(int(params.client_order_id))),
                tx.pure(bytes([int(params.order_type)])),
                tx.pure(bytes([int(params.self_matching_option)])),
                tx.pure(encode_u64(price)),
                tx.pure(encode_u64(quantity)),
                tx.pure(encode_bool(params.is_bid)),
                tx.pure(encode_bool(params.pay_with_deep)),
                tx.pure(encode_u64(expiration)),
                tx.object("0x6"),
            ],
            [base.type, quote.type],
        )

    def place_market_order(self, tx: Transaction, params: PlaceMarketOrderParams):
        pool = self.config.get_pool(params.pool_key)
        manager = self.config.get_balance_manager(params.balance_manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(params.quantity * base.scalar)
        proof = self.balance_manager.generate_proof(tx, params.balance_manager_key)

        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::place_market_order",
            [
                tx.object(pool.address),
                tx.object(manager.address),
                proof,
                tx.pure(encode_u64(int(params.client_order_id))),
                tx.pure(bytes([int(params.self_matching_option)])),
                tx.pure(encode_u64(quantity)),
                tx.pure(encode_bool(params.is_bid)),
                tx.pure(encode_bool(params.pay_with_deep)),
                tx.object("0x6"),
            ],
            [base.type, quote.type],
        )

    def cancel_order(self, tx: Transaction, pool_key: str, balance_manager_key: str, order_id: str):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(balance_manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        proof = self.balance_manager.generate_proof(tx, balance_manager_key)

        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::cancel_order",
            [tx.object(pool.address), tx.object(manager.address), proof, tx.pure(encode_u128(order_id)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def cancel_orders(self, tx: Transaction, pool_key: str, balance_manager_key: str, order_ids: list[str]):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(balance_manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        proof = self.balance_manager.generate_proof(tx, balance_manager_key)

        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::cancel_orders",
            [tx.object(pool.address), tx.object(manager.address), proof, tx.pure(encode_vec_u128(order_ids)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def get_quote_quantity_out(self, tx: Transaction, pool_key: str, base_quantity: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(base_quantity * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_quote_quantity_out",
            [tx.object(pool.address), tx.pure(encode_u64(quantity)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def get_base_quantity_out(self, tx: Transaction, pool_key: str, quote_quantity: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(quote_quantity * quote.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_base_quantity_out",
            [tx.object(pool.address), tx.pure(encode_u64(quantity)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def get_quantity_out(self, tx: Transaction, pool_key: str, base_quantity: float, quote_quantity: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        bq = round(base_quantity * base.scalar)
        qq = round(quote_quantity * quote.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_quantity_out",
            [tx.object(pool.address), tx.pure(encode_u64(bq)), tx.pure(encode_u64(qq)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def mid_price(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::mid_price",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def get_order(self, tx: Transaction, pool_key: str, order_id: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_order",
            [tx.object(pool.address), tx.pure(encode_u128(order_id))],
            [base.type, quote.type],
        )

    def can_place_limit_order(self, tx: Transaction, params: CanPlaceLimitOrderParams):
        pool = self.config.get_pool(params.pool_key)
        manager = self.config.get_balance_manager(params.balance_manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        price = round((params.price * FLOAT_SCALAR * quote.scalar) / base.scalar)
        quantity = round(params.quantity * base.scalar)

        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::can_place_limit_order",
            [
                tx.object(pool.address),
                tx.object(manager.address),
                tx.pure(encode_u64(price)),
                tx.pure(encode_u64(quantity)),
                tx.pure(encode_bool(params.is_bid)),
                tx.pure(encode_bool(params.pay_with_deep)),
                tx.pure(encode_u64(params.expire_timestamp)),
                tx.object("0x6"),
            ],
            [base.type, quote.type],
        )


@dataclass
class GovernanceContract:
    config: DeepBookConfig
    balance_manager: BalanceManagerContract

    def vote(self, tx: Transaction, pool_key: str, balance_manager_key: str, proposal_id: str):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(balance_manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        proof = self.balance_manager.generate_proof(tx, balance_manager_key)

        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::vote",
            [tx.object(pool.address), tx.object(manager.address), proof, tx.pure(encode_u128(proposal_id))],
            [base.type, quote.type],
        )


@dataclass
class FlashLoanContract:
    config: DeepBookConfig

    def borrow_base_asset(self, tx: Transaction, pool_key: str, borrow_amount: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        amount = round(borrow_amount * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::borrow_flashloan_base",
            [tx.object(pool.address), tx.pure(encode_u64(amount))],
            [base.type, quote.type],
        )


@dataclass
class MarginManagerContract:
    config: DeepBookConfig

    def get_margin_account_order_details(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)

        bm = tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::balance_manager",
            [tx.object(manager.address), tx.object(self.config.package_ids.margin_registry_id)],
            [base.type, quote.type],
        )
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_account_order_details",
            [tx.object(pool.address), bm],
            [base.type, quote.type],
        )


@dataclass
class PoolProxyContract:
    config: DeepBookConfig

    def place_limit_order(self, tx: Transaction, params: PlaceMarginLimitOrderParams):
        manager = self.config.get_margin_manager(params.margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        price = round((params.price * FLOAT_SCALAR * quote.scalar) / base.scalar)
        quantity = round(params.quantity * base.scalar)
        expiration = params.expiration or MAX_TIMESTAMP

        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::pool_proxy::place_limit_order",
            [
                tx.object(manager.address),
                tx.pure(encode_u64(int(params.client_order_id))),
                tx.pure(bytes([int(params.order_type)])),
                tx.pure(bytes([int(params.self_matching_option)])),
                tx.pure(encode_u64(price)),
                tx.pure(encode_u64(quantity)),
                tx.pure(encode_bool(params.is_bid)),
                tx.pure(encode_bool(params.pay_with_deep)),
                tx.pure(encode_u64(expiration)),
                tx.object(self.config.package_ids.margin_registry_id),
                tx.object("0x6"),
            ],
            [base.type, quote.type],
        )

    def place_market_order(self, tx: Transaction, params: PlaceMarginMarketOrderParams):
        manager = self.config.get_margin_manager(params.margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(params.quantity * base.scalar)

        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::pool_proxy::place_market_order",
            [
                tx.object(manager.address),
                tx.pure(encode_u64(int(params.client_order_id))),
                tx.pure(bytes([int(params.self_matching_option)])),
                tx.pure(encode_u64(quantity)),
                tx.pure(encode_bool(params.is_bid)),
                tx.pure(encode_bool(params.pay_with_deep)),
                tx.object(self.config.package_ids.margin_registry_id),
                tx.object("0x6"),
            ],
            [base.type, quote.type],
        )


@dataclass
class MarginTPSLContract:
    config: DeepBookConfig

    def cancel_all_conditional_orders(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)

        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::cancel_all_conditional_orders",
            [tx.object(manager.address), tx.object(self.config.package_ids.margin_registry_id)],
            [base.type, quote.type],
        )
