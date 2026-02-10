"""DeepBook v3 config and defaults."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict

from .types import BalanceManager, Coin, MarginManager, Pool

FLOAT_SCALAR = 1_000_000_000.0
DEEP_SCALAR = 1_000_000.0
MAX_TIMESTAMP = 1_844_674_407_370_955_161


@dataclass
class PackageIDs:
    deepbook_package_id: str
    registry_id: str
    deep_treasury_id: str
    margin_package_id: str
    margin_registry_id: str


TESTNET_PACKAGE_IDS = PackageIDs(
    deepbook_package_id="0x22be4cade64bf2d02412c7e8d0e8beea2f78828b948118d46735315409371a3c",
    registry_id="0x7c256edbda983a2cd6f946655f4bf3f00a41043993781f8674a7046e8c0e11d1",
    deep_treasury_id="0x69fffdae0075f8f71f4fa793549c11079266910e8905169845af1f5d00e09dcb",
    margin_package_id="0xd6a42f4df4db73d68cbeb52be66698d2fe6a9464f45ad113ca52b0c6ebd918b6",
    margin_registry_id="0x48d7640dfae2c6e9ceeada197a7a1643984b5a24c55a0c6c023dac77e0339f75",
)


TESTNET_COINS: Dict[str, Coin] = {
    "DEEP": Coin(
        address="0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8",
        type="0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP",
        scalar=1_000_000,
    ),
    "SUI": Coin(address="0x2", type="0x2::sui::SUI", scalar=1_000_000_000),
    "DBUSDC": Coin(
        address="0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7",
        type="0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC",
        scalar=1_000_000,
    ),
}

TESTNET_POOLS: Dict[str, Pool] = {
    "DEEP_SUI": Pool(
        address="0x48c95963e9eac37a316b7ae04a0deb761bcdcc2b67912374d6036e7f0e9bae9f",
        base_coin="DEEP",
        quote_coin="SUI",
    )
}


@dataclass
class DeepBookConfig:
    address: str
    network: str = "testnet"
    balance_managers: Dict[str, BalanceManager] = field(default_factory=dict)
    margin_managers: Dict[str, MarginManager] = field(default_factory=dict)
    coins: Dict[str, Coin] = field(default_factory=lambda: dict(TESTNET_COINS))
    pools: Dict[str, Pool] = field(default_factory=lambda: dict(TESTNET_POOLS))
    package_ids: PackageIDs = TESTNET_PACKAGE_IDS

    def get_coin(self, key: str) -> Coin:
        if key not in self.coins:
            raise KeyError(f"coin not found: {key}")
        return self.coins[key]

    def get_pool(self, key: str) -> Pool:
        if key not in self.pools:
            raise KeyError(f"pool not found: {key}")
        return self.pools[key]

    def get_balance_manager(self, key: str) -> BalanceManager:
        if key not in self.balance_managers:
            raise KeyError(f"balance manager not found: {key}")
        return self.balance_managers[key]

    def get_margin_manager(self, key: str) -> MarginManager:
        if key not in self.margin_managers:
            raise KeyError(f"margin manager not found: {key}")
        return self.margin_managers[key]
