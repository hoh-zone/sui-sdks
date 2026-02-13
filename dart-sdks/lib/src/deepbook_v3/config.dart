import 'types.dart';

const double floatScalar = 1000000000.0;
const double deepScalar = 1000000.0;
const int maxTimestamp = 1844674407370955161;

class PackageIds {
  const PackageIds({
    required this.deepbookPackageId,
    required this.registryId,
    required this.deepTreasuryId,
    required this.marginPackageId,
    required this.marginRegistryId,
    this.liquidationPackageId,
  });

  final String deepbookPackageId;
  final String registryId;
  final String deepTreasuryId;
  final String marginPackageId;
  final String marginRegistryId;
  final String? liquidationPackageId;
}

const PackageIds testnetPackageIds = PackageIds(
  deepbookPackageId:
      '0x22be4cade64bf2d02412c7e8d0e8beea2f78828b948118d46735315409371a3c',
  registryId:
      '0x7c256edbda983a2cd6f946655f4bf3f00a41043993781f8674a7046e8c0e11d1',
  deepTreasuryId:
      '0x69fffdae0075f8f71f4fa793549c11079266910e8905169845af1f5d00e09dcb',
  marginPackageId:
      '0xd6a42f4df4db73d68cbeb52be66698d2fe6a9464f45ad113ca52b0c6ebd918b6',
  marginRegistryId:
      '0x48d7640dfae2c6e9ceeada197a7a1643984b5a24c55a0c6c023dac77e0339f75',
  liquidationPackageId:
      '0xd6a42f4df4db73d68cbeb52be66698d2fe6a9464f45ad113ca52b0c6ebd918b6',
);

const Map<String, Coin> testnetCoins = {
  'DEEP': Coin(
    address:
        '0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8',
    type:
        '0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP',
    scalar: 1000000,
  ),
  'SUI': Coin(address: '0x2', type: '0x2::sui::SUI', scalar: 1000000000),
  'DBUSDC': Coin(
    address:
        '0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7',
    type:
        '0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC',
    scalar: 1000000,
  ),
};

const Map<String, Pool> testnetPools = {
  'DEEP_SUI': Pool(
    address:
        '0x48c95963e9eac37a316b7ae04a0deb761bcdcc2b67912374d6036e7f0e9bae9f',
    baseCoin: 'DEEP',
    quoteCoin: 'SUI',
  ),
};

const Map<String, MarginPool> testnetMarginPools = {
  'DEEP': MarginPool(
    address:
        '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
    type:
        '0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP',
  ),
  'SUI': MarginPool(
    address:
        '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
    type: '0x2::sui::SUI',
  ),
  'DBUSDC': MarginPool(
    address:
        '0x567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234',
    type:
        '0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC',
  ),
};

class DeepBookConfig {
  DeepBookConfig({
    required this.address,
    this.network = 'testnet',
    Map<String, BalanceManager>? balanceManagers,
    Map<String, MarginManager>? marginManagers,
    Map<String, Coin>? coins,
    Map<String, Pool>? pools,
    Map<String, MarginPool>? marginPools,
    this.packageIds = testnetPackageIds,
    this.marginMaintainerCap,
  })  : balanceManagers = balanceManagers ?? <String, BalanceManager>{},
        marginManagers = marginManagers ?? <String, MarginManager>{},
        coins = coins ?? Map<String, Coin>.from(testnetCoins),
        pools = pools ?? Map<String, Pool>.from(testnetPools),
        marginPools =
            marginPools ?? Map<String, MarginPool>.from(testnetMarginPools);

  final String address;
  final String network;
  final Map<String, BalanceManager> balanceManagers;
  final Map<String, MarginManager> marginManagers;
  final Map<String, Coin> coins;
  final Map<String, Pool> pools;
  final Map<String, MarginPool> marginPools;
  final PackageIds packageIds;
  final String? marginMaintainerCap;

  Coin getCoin(String key) {
    final coin = coins[key];
    if (coin == null) {
      throw StateError('coin not found: $key');
    }
    return coin;
  }

  Pool getPool(String key) {
    final pool = pools[key];
    if (pool == null) {
      throw StateError('pool not found: $key');
    }
    return pool;
  }

  MarginPool getMarginPool(String key) {
    final pool = marginPools[key];
    if (pool == null) {
      throw StateError('margin pool not found: $key');
    }
    return pool;
  }

  BalanceManager getBalanceManager(String key) {
    final manager = balanceManagers[key];
    if (manager == null) {
      throw StateError('balance manager not found: $key');
    }
    return manager;
  }

  MarginManager getMarginManager(String key) {
    final manager = marginManagers[key];
    if (manager == null) {
      throw StateError('margin manager not found: $key');
    }
    return manager;
  }
}
