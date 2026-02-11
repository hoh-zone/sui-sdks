"""DeepBook v3 contracts (baseline)."""

from __future__ import annotations

from dataclasses import dataclass

from pysdks.deepbook_v3.config import FLOAT_SCALAR, MAX_TIMESTAMP, DeepBookConfig
from pysdks.deepbook_v3.transactions.encode import (
    encode_address,
    encode_bool,
    encode_u64,
    encode_u128,
    encode_vec_u128,
)
from pysdks.deepbook_v3.transactions.transaction import Transaction
from pysdks.deepbook_v3.types import (
    CanPlaceLimitOrderParams,
    CanPlaceMarketOrderParams,
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

    def balance_manager_referral_owner(self, tx: Transaction, referral_id: str):
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::balance_manager::balance_manager_referral_owner",
            [tx.object(referral_id)],
            [],
        )

    def balance_manager_referral_pool_id(self, tx: Transaction, referral_id: str):
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::balance_manager::balance_manager_referral_pool_id",
            [tx.object(referral_id)],
            [],
        )

    def get_balance_manager_referral_id(self, tx: Transaction, manager_key: str, pool_key: str):
        manager = self.config.get_balance_manager(manager_key)
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::balance_manager::get_balance_manager_referral_id",
            [tx.object(manager.address), tx.pure(encode_address(pool.address))],
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

    def get_quote_quantity_out_input_fee(self, tx: Transaction, pool_key: str, base_quantity: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(base_quantity * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_quote_quantity_out_input_fee",
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

    def get_base_quantity_out_input_fee(self, tx: Transaction, pool_key: str, quote_quantity: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(quote_quantity * quote.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_base_quantity_out_input_fee",
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

    def get_quantity_out_input_fee(self, tx: Transaction, pool_key: str, base_quantity: float, quote_quantity: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        bq = round(base_quantity * base.scalar)
        qq = round(quote_quantity * quote.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_quantity_out_input_fee",
            [tx.object(pool.address), tx.pure(encode_u64(bq)), tx.pure(encode_u64(qq)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def get_base_quantity_in(self, tx: Transaction, pool_key: str, target_quote_quantity: float, pay_with_deep: bool):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(target_quote_quantity * quote.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_base_quantity_in",
            [tx.object(pool.address), tx.pure(encode_u64(quantity)), tx.pure(encode_bool(pay_with_deep)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def get_quote_quantity_in(self, tx: Transaction, pool_key: str, target_base_quantity: float, pay_with_deep: bool):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(target_base_quantity * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_quote_quantity_in",
            [tx.object(pool.address), tx.pure(encode_u64(quantity)), tx.pure(encode_bool(pay_with_deep)), tx.object("0x6")],
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

    def get_orders(self, tx: Transaction, pool_key: str, order_ids: list[str]):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_orders",
            [tx.object(pool.address), tx.pure(encode_vec_u128(order_ids))],
            [base.type, quote.type],
        )

    def get_level2_range(
        self, tx: Transaction, pool_key: str, price_low: float, price_high: float, is_bid: bool
    ):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        input_low = round((price_low * FLOAT_SCALAR * quote.scalar) / base.scalar)
        input_high = round((price_high * FLOAT_SCALAR * quote.scalar) / base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_level2_range",
            [tx.object(pool.address), tx.pure(encode_u64(input_low)), tx.pure(encode_u64(input_high)), tx.pure(encode_bool(is_bid)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def get_level2_ticks_from_mid(self, tx: Transaction, pool_key: str, ticks: int):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_level2_ticks_from_mid",
            [tx.object(pool.address), tx.pure(encode_u64(ticks)), tx.object("0x6")],
            [base.type, quote.type],
        )

    def account_open_orders(self, tx: Transaction, pool_key: str, manager_key: str):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::account_open_orders",
            [tx.object(pool.address), tx.object(manager.address)],
            [base.type, quote.type],
        )

    def get_pool_id_by_assets(self, tx: Transaction, base_type: str, quote_type: str):
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_pool_id_by_asset",
            [tx.object(self.config.package_ids.registry_id)],
            [base_type, quote_type],
        )

    def get_balance_manager_ids(self, tx: Transaction, owner: str):
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::registry::get_balance_manager_ids",
            [tx.object(self.config.package_ids.registry_id), tx.pure(owner.encode("utf-8"))],
            [],
        )

    def get_pool_referral_balances(self, tx: Transaction, pool_key: str, referral: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_pool_referral_balances",
            [tx.object(pool.address), tx.object(referral)],
            [base.type, quote.type],
        )

    def pool_referral_multiplier(self, tx: Transaction, pool_key: str, referral: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::pool_referral_multiplier",
            [tx.object(pool.address), tx.object(referral)],
            [base.type, quote.type],
        )

    def stable_pool(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::stable_pool",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def registered_pool(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::registered_pool",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def vault_balances(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::vault_balances",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def pool_trade_params(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::pool_trade_params",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def pool_trade_params_next(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::pool_trade_params_next",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def pool_book_params(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::pool_book_params",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def account(self, tx: Transaction, pool_key: str, manager_key: str):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::account",
            [tx.object(pool.address), tx.object(manager.address)],
            [base.type, quote.type],
        )

    def get_account_order_details(self, tx: Transaction, pool_key: str, manager_key: str):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_account_order_details",
            [tx.object(pool.address), tx.object(manager.address)],
            [base.type, quote.type],
        )

    def get_order_deep_required(self, tx: Transaction, pool_key: str, base_quantity: float, price: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        input_price = round((price * FLOAT_SCALAR * quote.scalar) / base.scalar)
        input_quantity = round(base_quantity * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_order_deep_required",
            [tx.object(pool.address), tx.pure(encode_u64(input_quantity)), tx.pure(encode_u64(input_price))],
            [base.type, quote.type],
        )

    def account_exists(self, tx: Transaction, pool_key: str, manager_key: str):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::account_exists",
            [tx.object(pool.address), tx.object(manager.address)],
            [base.type, quote.type],
        )

    def locked_balance(self, tx: Transaction, pool_key: str, manager_key: str):
        pool = self.config.get_pool(pool_key)
        manager = self.config.get_balance_manager(manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::locked_balance",
            [tx.object(pool.address), tx.object(manager.address)],
            [base.type, quote.type],
        )

    def get_pool_deep_price(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::get_order_deep_price",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def quorum(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::quorum",
            [tx.object(pool.address)],
            [base.type, quote.type],
        )

    def pool_id(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::id",
            [tx.object(pool.address)],
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

    def can_place_market_order(self, tx: Transaction, params: CanPlaceMarketOrderParams):
        pool = self.config.get_pool(params.pool_key)
        manager = self.config.get_balance_manager(params.balance_manager_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        quantity = round(params.quantity * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::can_place_market_order",
            [
                tx.object(pool.address),
                tx.object(manager.address),
                tx.pure(encode_u64(quantity)),
                tx.pure(encode_bool(params.is_bid)),
                tx.pure(encode_bool(params.pay_with_deep)),
                tx.object("0x6"),
            ],
            [base.type, quote.type],
        )

    def check_market_order_params(self, tx: Transaction, pool_key: str, quantity: float):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        input_quantity = round(quantity * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::check_market_order_params",
            [tx.object(pool.address), tx.pure(encode_u64(input_quantity))],
            [base.type, quote.type],
        )

    def check_limit_order_params(self, tx: Transaction, pool_key: str, price: float, quantity: float, expire_timestamp: int):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        input_price = round((price * FLOAT_SCALAR * quote.scalar) / base.scalar)
        input_quantity = round(quantity * base.scalar)
        return tx.move_call(
            f"{self.config.package_ids.deepbook_package_id}::pool::check_limit_order_params",
            [
                tx.object(pool.address),
                tx.pure(encode_u64(input_price)),
                tx.pure(encode_u64(input_quantity)),
                tx.pure(encode_u64(expire_timestamp)),
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

    def owner(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::owner",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def deepbook_pool(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::deepbook_pool",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def margin_pool_id(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::margin_pool_id",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def borrowed_shares(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::borrowed_shares",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def borrowed_base_shares(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::borrowed_base_shares",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def borrowed_quote_shares(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::borrowed_quote_shares",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def has_base_debt(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::has_base_debt",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def balance_manager(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::balance_manager",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def base_balance(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::base_balance",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def quote_balance(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::quote_balance",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def deep_balance(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::deep_balance",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def calculate_assets(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::calculate_assets",
            [tx.object(manager.address), tx.object(pool.address)],
            [base.type, quote.type],
        )

    def calculate_debts(self, tx: Transaction, margin_manager_key: str, debt_coin_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        debt_coin = self.config.get_coin(debt_coin_key)
        margin_pool = tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::get_margin_pool_id",
            [tx.object(self.config.package_ids.margin_registry_id)],
            [debt_coin.type],
        )
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::calculate_debts",
            [tx.object(manager.address), margin_pool, tx.object("0x6")],
            [base.type, quote.type, debt_coin.type],
        )

    def manager_state(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        return self.manager_state_by_id(tx, manager.pool_key, manager.address)

    def manager_state_by_id(self, tx: Transaction, pool_key: str, margin_manager_id: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::manager_state",
            [
                tx.object(margin_manager_id),
                tx.object(self.config.package_ids.margin_registry_id),
                tx.object(pool.address),
                tx.object("0x6"),
            ],
            [base.type, quote.type],
        )


@dataclass
class MarginRegistryContract:
    config: DeepBookConfig

    def pool_enabled(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::pool_enabled",
            [tx.object(self.config.package_ids.margin_registry_id), tx.object(pool.address)],
            [base.type, quote.type],
        )

    def get_margin_pool_id(self, tx: Transaction, coin_key: str):
        coin = self.config.get_coin(coin_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::get_margin_pool_id",
            [tx.object(self.config.package_ids.margin_registry_id)],
            [coin.type],
        )

    def get_deepbook_pool_margin_pool_ids(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::get_deepbook_pool_margin_pool_ids",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def get_margin_manager_ids(self, tx: Transaction, owner: str):
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::get_margin_manager_ids",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(owner))],
            [],
        )

    def base_margin_pool_id(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::base_margin_pool_id",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def quote_margin_pool_id(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::quote_margin_pool_id",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def min_withdraw_risk_ratio(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::min_withdraw_risk_ratio",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def min_borrow_risk_ratio(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::min_borrow_risk_ratio",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def liquidation_risk_ratio(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::liquidation_risk_ratio",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def target_liquidation_risk_ratio(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::target_liquidation_risk_ratio",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def user_liquidation_reward(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::user_liquidation_reward",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def pool_liquidation_reward(self, tx: Transaction, pool_key: str):
        pool = self.config.get_pool(pool_key)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::pool_liquidation_reward",
            [tx.object(self.config.package_ids.margin_registry_id), tx.pure(encode_address(pool.address))],
            [],
        )

    def allowed_maintainers(self, tx: Transaction):
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::allowed_maintainers",
            [tx.object(self.config.package_ids.margin_registry_id)],
            [],
        )

    def allowed_pause_caps(self, tx: Transaction):
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_registry::allowed_pause_caps",
            [tx.object(self.config.package_ids.margin_registry_id)],
            [],
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

    def conditional_order_ids(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::conditional_order_ids",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def lowest_trigger_above_price(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::lowest_trigger_above_price",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )

    def highest_trigger_below_price(self, tx: Transaction, margin_manager_key: str):
        manager = self.config.get_margin_manager(margin_manager_key)
        pool = self.config.get_pool(manager.pool_key)
        base = self.config.get_coin(pool.base_coin)
        quote = self.config.get_coin(pool.quote_coin)
        return tx.move_call(
            f"{self.config.package_ids.margin_package_id}::margin_manager::highest_trigger_below_price",
            [tx.object(manager.address)],
            [base.type, quote.type],
        )
