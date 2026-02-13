import 'dart:convert';
import 'dart:typed_data';

import '../bcs/reader.dart';
import '../sui/transactions.dart';
import 'config.dart';
import 'transactions/contracts.dart';

class DeepBookClient {
  DeepBookClient({required this.client, required this.config}) {
    balanceManager = BalanceManagerContract(config);
    deepbook = DeepBookContract(config, balanceManager);
    governance = GovernanceContract(config, balanceManager);
    flashLoans = FlashLoanContract(config);
    admin = DeepBookAdminContract(config);
    marginManager = MarginManagerContract(config);
    marginTpsl = MarginTPSLContract(config);
    swap = SwapMethods(config);
    marginPool = MarginPoolContract(config);
    marginLiquidations = MarginLiquidationsContract(config);
    marginMaintainer = MarginMaintainerContract(config);
    marginRegistry = MarginRegistryContract(config);
    poolProxy = PoolProxyContract(config);
  }

  final dynamic client;
  final DeepBookConfig config;

  late final BalanceManagerContract balanceManager;
  late final DeepBookContract deepbook;
  late final GovernanceContract governance;
  late final FlashLoanContract flashLoans;
  late final DeepBookAdminContract admin;
  late final MarginManagerContract marginManager;
  late final MarginTPSLContract marginTpsl;
  late final SwapMethods swap;
  late final MarginPoolContract marginPool;
  late final MarginLiquidationsContract marginLiquidations;
  late final MarginMaintainerContract marginMaintainer;
  late final MarginRegistryContract marginRegistry;
  late final PoolProxyContract poolProxy;

  Future<Map<String, dynamic>> checkManagerBalance(
      String managerKey, String coinKey) async {
    final tx = Transaction();
    final manager = config.getBalanceManager(managerKey);
    final coin = config.getCoin(coinKey);
    tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::balance',
      [tx.object(manager.address)],
      [coin.type],
    );

    final value = _readU64(await _simulate(tx), 0, 0);
    return {'coinType': coin.type, 'balance': value / coin.scalar};
  }

  Future<bool> whitelisted(String poolKey) async {
    final tx = Transaction();
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::whitelisted',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );

    final raw = _returnBytes(await _simulate(tx), 0, 0);
    return raw.isNotEmpty && raw.first == 1;
  }

  Future<Map<String, dynamic>> getQuoteQuantityOut(
      String poolKey, double baseQuantity) async {
    final tx = Transaction();
    deepbook.getQuoteQuantityOut(tx, poolKey, baseQuantity);
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final sim = await _simulate(tx);
    final baseOut = _readU64(sim, 0, 0);
    final quoteOut = _readU64(sim, 0, 1);
    final deepRequired = _readU64(sim, 0, 2);

    return {
      'baseQuantity': baseQuantity,
      'baseOut': baseOut / base.scalar,
      'quoteOut': quoteOut / quote.scalar,
      'deepRequired': deepRequired / deepScalar,
    };
  }

  Future<Map<String, dynamic>> getBaseQuantityOut(
      String poolKey, double quoteQuantity) async {
    final tx = Transaction();
    deepbook.getBaseQuantityOut(tx, poolKey, quoteQuantity);
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final sim = await _simulate(tx);
    final baseOut = _readU64(sim, 0, 0);
    final quoteOut = _readU64(sim, 0, 1);
    final deepRequired = _readU64(sim, 0, 2);

    return {
      'quoteQuantity': quoteQuantity,
      'baseOut': baseOut / base.scalar,
      'quoteOut': quoteOut / quote.scalar,
      'deepRequired': deepRequired / deepScalar,
    };
  }

  Future<Map<String, dynamic>> getQuantityOut(
      String poolKey, double baseQuantity, double quoteQuantity) async {
    final tx = Transaction();
    deepbook.getQuantityOut(tx, poolKey, baseQuantity, quoteQuantity);
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final sim = await _simulate(tx);
    final baseOut = _readU64(sim, 0, 0);
    final quoteOut = _readU64(sim, 0, 1);
    final deepRequired = _readU64(sim, 0, 2);

    return {
      'baseQuantity': baseQuantity,
      'quoteQuantity': quoteQuantity,
      'baseOut': baseOut / base.scalar,
      'quoteOut': quoteOut / quote.scalar,
      'deepRequired': deepRequired / deepScalar,
    };
  }

  Future<double> midPrice(String poolKey) async {
    final tx = Transaction();
    deepbook.midPrice(tx, poolKey);
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final value = _readU64(await _simulate(tx), 0, 0);
    return (value * base.scalar) / (floatScalar * quote.scalar);
  }

  Future<String> getOrder(String poolKey, String orderId) async {
    final tx = Transaction();
    deepbook.getOrder(tx, poolKey, orderId);
    return base64Encode(_returnBytes(await _simulate(tx), 0, 0));
  }

  Future<String> getMarginAccountOrderDetails(String marginManagerKey) async {
    final tx = Transaction();
    marginManager.getMarginAccountOrderDetails(tx, marginManagerKey);
    return base64Encode(_returnBytes(await _simulate(tx), 1, 0));
  }

  Future<Map<String, dynamic>> _simulate(Transaction tx) async {
    final out = await _rpcCall('sui_dryRunTransactionBlock', [tx.commands]);
    final result = out['result'];
    if (result is Map<String, dynamic>) {
      return result;
    }
    return out;
  }

  Future<Map<String, dynamic>> _rpcCall(
      String method, List<dynamic> params) async {
    try {
      final out = await client.call(method, params);
      return Map<String, dynamic>.from(out as Map);
    } catch (_) {
      final out = await client.execute(method, params);
      return Map<String, dynamic>.from(out as Map);
    }
  }

  Uint8List _returnBytes(
      Map<String, dynamic> sim, int commandIndex, int returnIndex) {
    final commandResults = sim['commandResults'];
    if (commandResults is! List || commandResults.length <= commandIndex) {
      throw StateError('missing commandResults[$commandIndex]');
    }

    final commandResult = commandResults[commandIndex];
    if (commandResult is! Map<String, dynamic>) {
      throw StateError('invalid command result');
    }

    final returnValues = commandResult['returnValues'];
    if (returnValues is! List || returnValues.length <= returnIndex) {
      throw StateError('missing returnValues[$returnIndex]');
    }

    final ret = returnValues[returnIndex];
    if (ret is! Map<String, dynamic> || ret['bcs'] is! String) {
      throw StateError('invalid bcs return value');
    }

    return Uint8List.fromList(base64Decode(ret['bcs'] as String));
  }

  int _readU64(Map<String, dynamic> sim, int commandIndex, int returnIndex) {
    final raw = _returnBytes(sim, commandIndex, returnIndex);
    return BcsReader(raw).readU64();
  }
}
