// Intent system for Dart SDK
library sui_sdk.intents;

const String _COIN_WITH_BALANCE = 'CoinWithBalance';
const String _SUI_TYPE = '0x2::sui::SUI';

/// CoinWithBalance intent
class CoinWithBalance {
  final String name;
  final String coinType;
  final BigInt balance;
  final bool useGasCoin;

  CoinWithBalance({
    required this.name,
    required this.coinType,
    required this.balance,
    this.useGasCoin = true,
  });
}

/// CoinWithBalanceBuilder for building CoinWithBalance intent
class CoinWithBalanceBuilder {
  final BigInt balance;
  String coinType = _SUI_TYPE;
  bool useGasCoin = true;

  CoinWithBalanceBuilder(this.balance);

  CoinWithBalanceBuilder withCoinType(String type) {
    coinType = type;
    return this;
  }

  CoinWithBalanceBuilder withGasCoin(bool use) {
    useGasCoin = use;
    return this;
  }

  CoinWithBalance build() {
    return CoinWithBalance(
      name: _COIN_WITH_BALANCE,
      coinType: coinType,
      balance: balance,
      useGasCoin: useGasCoin,
    );
  }
}

/// Create a CoinWithBalance intent
CoinWithBalanceBuilder coinWithBalance(BigInt balance) {
  return CoinWithBalanceBuilder(balance);
}

/// Create a CoinWithBalance intent with type
CoinWithBalanceBuilder coinWithBalanceWithType(BigInt balance, String type) {
  return CoinWithBalanceBuilder(balance).withCoinType(type);
}

/// IntentScope enum
enum IntentScope {
  transactionData(0),
  personalMessage(1),
  transaction(3);

  final int value;
  const IntentScope(this.value);
}
