// Transaction Executor for Dart SDK
library sui_sdk.executor;

/// TransactionResult
class TransactionResult {
  final bool success;
  final String? error;

  TransactionResult({required this.success, this.error});

  factory TransactionResult.success() {
    return TransactionResult(success: true, error: null);
  }
}

/// Transaction
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

/// Command
class Command {
  final String commandType;
  final Map<String, dynamic>? data;

  Command({
    required this.commandType,
    this.data,
  });
}

/// SerialTransactionExecutor
class SerialTransactionExecutor {
  final _executor = TransactionExecutor();

  Future<TransactionResult> execute(Transaction transaction) async {
    return _executor.execute(transaction);
  }
}

/// ParallelTransactionExecutor
class ParallelTransactionExecutor {
  final int workers;

  ParallelTransactionExecutor({this.workers = 4});

  Future<List<TransactionResult>> executeAll(
      List<Transaction> transactions) async {
    return List.generate(
      transactions.length,
      (_) => TransactionResult.success(),
    );
  }
}

/// TransactionExecutor
class TransactionExecutor {
  final List<Transaction> _queue = [];

  TransactionResult execute(Transaction transaction) {
    return TransactionResult.success();
  }
}

/// ObjectCache
class ObjectCache {
  final Map<String, Map<String, dynamic>> _cache = {};

  Map<String, dynamic>? get(String id) {
    return _cache[id];
  }

  void set(String id, Map<String, dynamic> obj) {
    _cache[id] = obj;
  }

  void delete(String id) {
    _cache.remove(id);
  }

  void clear() {
    _cache.clear();
  }
}
