# Intent system for Python SDK
"""
Intent system for Python SDK
"""

COIN_WITH_BALANCE = "CoinWithBalance"
SUI_TYPE = "0x2::sui::SUI"


class CoinWithBalance:
    """CoinWithBalance intent"""

    def __init__(self, name: str, coin_type: str, balance: int, use_gas_coin: bool = True):
        self.name = name
        self.coin_type = coin_type
        self.balance = balance
        self.use_gas_coin = use_gas_coin


class CoinWithBalanceBuilder:
    """Builder for CoinWithBalance intent"""

    def __init__(self, balance: int):
        self.balance = balance
        self.coin_type = SUI_TYPE
        self.use_gas_coin = True

    def with_coin_type(self, coin_type: str) -> "CoinWithBalanceBuilder":
        self.coin_type = coin_type
        return self

    def with_gas_coin(self, use: bool) -> "CoinWithBalanceBuilder":
        self.use_gas_coin = use
        return self

    def build(self) -> CoinWithBalance:
        return CoinWithBalance(
            COIN_WITH_BALANCE,
            self.coin_type,
            self.balance,
            self.use_gas_coin
        )


def coin_with_balance(balance: int) -> CoinWithBalanceBuilder:
    """Create a CoinWithBalance intent builder"""
    return CoinWithBalanceBuilder(balance)


def coin_with_balance_with_type(balance: int, coin_type: str) -> CoinWithBalanceBuilder:
    """Create a CoinWithBalance intent with type"""
    return CoinWithBalanceBuilder(balance).with_coin_type(coin_type)


class IntentResolver:
    """Base class for intent resolvers"""

    def resolve(self, intent: CoinWithBalance):
        """Resolve the intent"""
        pass


class CoinWithBalanceResolver(IntentResolver):
    """Resolver for CoinWithBalance intents"""

    def __init__(self, sender: str):
        self.sender = sender

    def resolve_coin_balance(self, intents: list[CoinWithBalance], sender: str) -> dict:
        """Resolve coin balance for intents"""
        coin_types = set()
        total_by_type = {}

        for intent in intents:
            if intent.coin_type != "gas" and intent.balance > 0:
                coin_types.add(intent.coin_type)

            if intent.coin_type not in total_by_type:
                total_by_type[intent.coin_type] = 0
            total_by_type[intent.coin_type] += intent.balance

        return {
            "coinTypes": coin_types,
            "totalByType": total_by_type,
            "sender": sender
        }

    def resolve(self, intent: CoinWithBalance):
        """Resolve the intent"""
        pass


class IntentScope:
    """Intent scope enum"""
    TRANSACTION_DATA = 0
    PERSONAL_MESSAGE = 1
    TRANSACTION = 3