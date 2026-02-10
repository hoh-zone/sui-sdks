"""DeepBook v3 shared types."""

from __future__ import annotations

from dataclasses import dataclass
from enum import IntEnum


@dataclass
class BalanceManager:
    address: str
    trade_cap: str = ""
    deposit_cap: str = ""
    withdraw_cap: str = ""


@dataclass
class MarginManager:
    address: str
    pool_key: str


@dataclass
class Coin:
    address: str
    type: str
    scalar: float


@dataclass
class Pool:
    address: str
    base_coin: str
    quote_coin: str


class OrderType(IntEnum):
    NO_RESTRICTION = 0
    IMMEDIATE_OR_CANCEL = 1
    FILL_OR_KILL = 2
    POST_ONLY = 3


class SelfMatchingOptions(IntEnum):
    SELF_MATCHING_ALLOWED = 0
    CANCEL_TAKER = 1
    CANCEL_MAKER = 2


@dataclass
class PlaceLimitOrderParams:
    pool_key: str
    balance_manager_key: str
    client_order_id: str
    price: float
    quantity: float
    is_bid: bool
    expiration: int = 0
    order_type: OrderType = OrderType.NO_RESTRICTION
    self_matching_option: SelfMatchingOptions = SelfMatchingOptions.SELF_MATCHING_ALLOWED
    pay_with_deep: bool = True


@dataclass
class PlaceMarketOrderParams:
    pool_key: str
    balance_manager_key: str
    client_order_id: str
    quantity: float
    is_bid: bool
    self_matching_option: SelfMatchingOptions = SelfMatchingOptions.SELF_MATCHING_ALLOWED
    pay_with_deep: bool = True


@dataclass
class CanPlaceLimitOrderParams:
    pool_key: str
    balance_manager_key: str
    price: float
    quantity: float
    is_bid: bool
    pay_with_deep: bool
    expire_timestamp: int


@dataclass
class CanPlaceMarketOrderParams:
    pool_key: str
    balance_manager_key: str
    quantity: float
    is_bid: bool
    pay_with_deep: bool


@dataclass
class PlaceMarginLimitOrderParams:
    pool_key: str
    margin_manager_key: str
    client_order_id: str
    price: float
    quantity: float
    is_bid: bool
    expiration: int = 0
    order_type: OrderType = OrderType.NO_RESTRICTION
    self_matching_option: SelfMatchingOptions = SelfMatchingOptions.SELF_MATCHING_ALLOWED
    pay_with_deep: bool = True


@dataclass
class PlaceMarginMarketOrderParams:
    pool_key: str
    margin_manager_key: str
    client_order_id: str
    quantity: float
    is_bid: bool
    self_matching_option: SelfMatchingOptions = SelfMatchingOptions.SELF_MATCHING_ALLOWED
    pay_with_deep: bool = True
