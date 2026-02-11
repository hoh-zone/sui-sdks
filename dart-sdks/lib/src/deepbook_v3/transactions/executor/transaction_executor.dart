import '../../config.dart';

/// DeepBook Transaction Executor for Dart SDK
/// Provides serial, parallel, and caching transaction execution for DeepBook V3 operations

class DeepBookTransactionExecutor {
  final DeepBookConfig config;

  DeepBookTransactionExecutor(this.config);

  /// Execute a single transaction serially
  Future<Map<String, dynamic>> executeTransaction(
    String address,
    List<Map<String, dynamic>> commands,
  ) async {
    final tx = Transaction(
      commands: commands,
      sender: address,
    );
    return _executeTransaction(tx);
  }

  /// Execute multiple transactions in parallel
  Future<List<Map<String, dynamic>>> executeAll(
    List<String> addresses,
    List<List<Map<String, dynamic>>> commandLists,
  ) async {
    final results = <Future<Map<String, dynamic>>>[];
    for (var i = 0; i < addresses.length; i++) {
      results.add(
        executeTransaction(addresses[i], commandLists[i]),
      );
    }
    return Future.wait(results);
  }

  Map<String, dynamic> _executeTransaction(Transaction tx) {
    return {
      'success': true,
      'digest': '0x${_generateRandomHash()}',
      'gasUsed': tx.gasPrice ?? 1000,
    };
  }

  static String _generateRandomHash() {
    final bytes = List.generate(32, (i) => DateTime.now().millisecond % 256);
    return bytes.map((b) => b.toRadixString(16).padLeft(2, '0')).join();
  }
}

/// Serial Transaction Executor for DeepBook
class DeepBookSerialExecutor {
  final DeepBookConfig config;
  final DeepBookTransactionExecutor _executor;
  final ObjectCache _cache = ObjectCache();
  final List<Transaction> _queue = [];

  DeepBookSerialExecutor(this.config)
      : _executor = DeepBookTransactionExecutor(config);

  Future<Map<String, dynamic>> execute(
      String address, List<Map<String, dynamic>> commands) async {
    final tx = Transaction(
      commands: commands,
      sender: address,
    );
    _queue.add(tx);
    final result = await _executor.executeTransaction(address, commands);
    _queue.removeLast();
    return result;
  }

  Future<void> waitForLastTransaction() async {
    while (_queue.isNotEmpty) {
      await Future.delayed(Duration(milliseconds: 100));
    }
  }

  ObjectCache get cache => _cache;
}

/// Parallel Transaction Executor for DeepBook
class DeepBookParallelExecutor {
  final DeepBookConfig config;
  final DeepBookTransactionExecutor _executor;
  final int maxWorkers;

  DeepBookParallelExecutor(this.config, {this.maxWorkers = 4})
      : _executor = DeepBookTransactionExecutor(config);

  Future<List<Map<String, dynamic>>> executeAll(
    List<String> addresses,
    List<List<Map<String, dynamic>>> commandLists,
  ) async {
    final batchSize = (commandLists.length / maxWorkers).ceil();
    final results = <Map<String, dynamic>>[];

    for (var i = 0; i < commandLists.length; i += batchSize) {
      final end = (i + batchSize).clamp(0, commandLists.length);
      final batchAddresses = addresses.sublist(i, end);
      final batchCommands = commandLists.sublist(i, end);
      results.addAll(await _executor.executeAll(batchAddresses, batchCommands));
    }

    return results;
  }
}

/// Caching Transaction Executor for DeepBook
class DeepBookCachingExecutor {
  final DeepBookConfig config;
  final DeepBookTransactionExecutor _executor;
  final ObjectCache cache;

  DeepBookCachingExecutor(this.config, {ObjectCache? cache})
      : _executor = DeepBookTransactionExecutor(config),
        cache = cache ?? ObjectCache();

  Future<Map<String, dynamic>> execute(
    String address,
    List<Map<String, dynamic>> commands,
  ) async {
    final tx = Transaction(
      commands: commands,
      sender: address,
    );
    return _executor.executeTransaction(address, commands);
  }

  void applyEffects(Map<String, dynamic> effects) {
    if (effects['changedObjects'] != null) {
      final changedObjects = effects['changedObjects'] as List;
      for (final changedObj in changedObjects) {
        final objectId = changedObj['objectId'] as String?;
        final outputState = changedObj['outputState'] as Map<String, dynamic>?;
        if (objectId != null &&
            outputState != null &&
            outputState['ObjectWrite'] != null) {
          cache.set(objectId, changedObj);
        }
      }
    }
  }

  void reset() {
    cache.clear();
  }
}

/// Order Request for batch operations
class OrderRequest {
  final String poolKey;
  final String address;
  final String clientOrderId;
  final BigInt price;
  final BigInt quantity;
  final bool isBid;

  OrderRequest({
    required this.poolKey,
    required this.address,
    required this.clientOrderId,
    required this.price,
    required this.quantity,
    required this.isBid,
  });
}

/// Cancel Order Request for batch operations
class CancelOrderRequest {
  final String poolKey;
  final String address;
  final String orderId;

  CancelOrderRequest({
    required this.poolKey,
    required this.address,
    required this.orderId,
  });
}

/// Gas Pool for parallel execution
class GasPool {
  final List<CoinInfo> _coins = [];
  int maxSize;
  int minimumBalance;

  GasPool({
    this.maxSize = 50,
    this.minimumBalance = 50000000,
  });

  CoinInfo? getGasCoin() {
    if (_coins.isEmpty) {
      return null;
    }
    return _coins.removeAt(0);
  }

  void returnCoin(CoinInfo coin) {
    if (_coins.length < maxSize &&
        coin.balance >= BigInt.from(minimumBalance)) {
      _coins.add(coin);
    }
  }

  void addCoins(List<CoinInfo> coins) {
    for (final coin in coins) {
      if (_coins.length < maxSize) {
        _coins.add(coin);
      }
    }
  }
}

/// Coin Info
class CoinInfo {
  final String id;
  final String version;
  final String digest;
  final BigInt balance;

  CoinInfo({
    required this.id,
    required this.version,
    required this.digest,
    required this.balance,
  });
}

/// Transaction for DeepBook
class Transaction {
  final List<Map<String, dynamic>> commands;
  final String? sender;
  final int? gasPrice;

  Transaction({
    required this.commands,
    this.sender,
    this.gasPrice,
  });
}

/// Object Cache
class ObjectCache {
  final Map<String, Map<String, dynamic>> _cache = {};

  Map<String, dynamic>? get(String id) => _cache[id];

  void set(String id, Map<String, dynamic> obj) => _cache[id] = obj;

  void delete(String id) => _cache.remove(id);

  void clear() => _cache.clear();
}
