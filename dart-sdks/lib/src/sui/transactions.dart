import 'dart:convert';
import 'dart:typed_data';

const String systemStateObjectId = '0x5';
const String stakeRequestTarget = '0x3::sui_system::request_add_stake';
const String unstakeRequestTarget = '0x3::sui_system::request_withdraw_stake';

class ObjectRef {
  const ObjectRef({required this.objectId, required this.digest, required this.version});

  final String objectId;
  final String digest;
  final int version;
}

class Inputs {
  static Map<String, dynamic> pure(List<int> value) {
    return {
      '\$kind': 'Pure',
      'Pure': {'bytes': base64Encode(value)},
    };
  }

  static Map<String, dynamic> objectRef(ObjectRef ref) {
    return {
      '\$kind': 'Object',
      'Object': {
        '\$kind': 'ImmOrOwnedObject',
        'ImmOrOwnedObject': {
          'objectId': ref.objectId,
          'digest': ref.digest,
          'version': ref.version,
        },
      },
    };
  }

  static Map<String, dynamic> sharedObjectRef(String objectId, bool mutable, int initialSharedVersion) {
    return {
      '\$kind': 'Object',
      'Object': {
        '\$kind': 'SharedObject',
        'SharedObject': {
          'objectId': objectId,
          'mutable': mutable,
          'initialSharedVersion': initialSharedVersion,
        },
      },
    };
  }
}

class TransactionCommands {
  static Map<String, dynamic> moveCall(
    String target,
    List<Map<String, dynamic>> args, [
    List<String>? typeArgs,
  ]) {
    final segments = target.split('::');
    final package = segments.isNotEmpty ? segments[0] : '';
    final module = segments.length > 1 ? segments[1] : '';
    final function = segments.length > 2 ? segments[2] : '';

    return {
      '\$kind': 'MoveCall',
      'MoveCall': {
        'package': package,
        'module': module,
        'function': function,
        'arguments': args,
        'typeArguments': typeArgs ?? <String>[],
      },
    };
  }

  static Map<String, dynamic> splitCoins(Map<String, dynamic> coin, List<Map<String, dynamic>> amounts) {
    return {
      '\$kind': 'SplitCoins',
      'SplitCoins': {'coin': coin, 'amounts': amounts},
    };
  }

  static Map<String, dynamic> transferObjects(List<Map<String, dynamic>> objects, Map<String, dynamic> address) {
    return {
      '\$kind': 'TransferObjects',
      'TransferObjects': {'objects': objects, 'address': address},
    };
  }

  static Map<String, dynamic> mergeCoins(Map<String, dynamic> destination, List<Map<String, dynamic>> sources) {
    return {
      '\$kind': 'MergeCoins',
      'MergeCoins': {'destination': destination, 'sources': sources},
    };
  }

  static Map<String, dynamic> publish(List<List<int>> modules, List<String> dependencies) {
    return {
      '\$kind': 'Publish',
      'Publish': {
        'modules': modules.map(base64Encode).toList(),
        'dependencies': dependencies,
      },
    };
  }

  static Map<String, dynamic> upgrade(
    List<List<int>> modules,
    List<String> dependencies,
    String packageId,
    Map<String, dynamic> ticket,
  ) {
    return {
      '\$kind': 'Upgrade',
      'Upgrade': {
        'modules': modules.map(base64Encode).toList(),
        'dependencies': dependencies,
        'package': packageId,
        'ticket': ticket,
      },
    };
  }

  static Map<String, dynamic> makeMoveVec(String? typeTag, List<Map<String, dynamic>> elements) {
    return {
      '\$kind': 'MakeMoveVec',
      'MakeMoveVec': {'type': typeTag, 'elements': elements},
    };
  }
}

class TransactionData {
  String sender = '';
  dynamic expiration;
  final Map<String, dynamic> gasData = <String, dynamic>{};
  final List<Map<String, dynamic>> inputs = <Map<String, dynamic>>[];
  final List<Map<String, dynamic>> commands = <Map<String, dynamic>>[];
}

class Transaction {
  Transaction({this.client});

  final TransactionData data = TransactionData();
  final dynamic client;

  List<Map<String, dynamic>> get commands => data.commands;
  List<Map<String, dynamic>> get inputs => data.inputs;

  void setSender(String sender) {
    data.sender = sender;
  }

  void setSenderIfNotSet(String sender) {
    if (data.sender.isEmpty) {
      data.sender = sender;
    }
  }

  void setGasBudget(int budget) {
    data.gasData['budget'] = budget.toString();
  }

  void setGasBudgetIfNotSet(int budget) {
    if (data.gasData['budget'] == null) {
      setGasBudget(budget);
    }
  }

  void setExpiration(dynamic expiration) {
    data.expiration = expiration;
  }

  void setGasPrice(int price) {
    data.gasData['price'] = price.toString();
  }

  void setGasOwner(String owner) {
    data.gasData['owner'] = owner;
  }

  void setGasPayment(List<Map<String, dynamic>> payments) {
    data.gasData['payment'] = payments;
  }

  Map<String, dynamic> gas() => {
        '\$kind': 'GasCoin',
        'GasCoin': true,
      };

  Map<String, dynamic> addInput(Map<String, dynamic> arg) {
    data.inputs.add(arg);
    return {
      '\$kind': 'Input',
      'Input': data.inputs.length - 1,
    };
  }

  Map<String, dynamic> object(dynamic value) {
    if (value is String) {
      return addInput({
        '\$kind': 'UnresolvedObject',
        'UnresolvedObject': {'objectId': value},
      });
    }
    if (value is Map<String, dynamic> && value['\$kind'] == 'Input') {
      return value;
    }
    return addInput(Map<String, dynamic>.from(value as Map));
  }

  Map<String, dynamic> pure(List<int> value) {
    return addInput(Inputs.pure(value));
  }

  Map<String, dynamic> addCommand(Map<String, dynamic> cmd) {
    data.commands.add(cmd);
    return {
      '\$kind': 'Result',
      'Result': data.commands.length - 1,
    };
  }

  Map<String, dynamic> moveCall(String target, List<Map<String, dynamic>> args, [List<String>? typeArgs]) {
    return addCommand(TransactionCommands.moveCall(target, args, typeArgs));
  }

  Map<String, dynamic> transferObjects(List<Map<String, dynamic>> objects, Map<String, dynamic> address) {
    return addCommand(TransactionCommands.transferObjects(objects, address));
  }

  Map<String, dynamic> splitCoins(Map<String, dynamic> coin, List<Map<String, dynamic>> amounts) {
    return addCommand(TransactionCommands.splitCoins(coin, amounts));
  }

  Map<String, dynamic> mergeCoins(Map<String, dynamic> destination, List<Map<String, dynamic>> sources) {
    return addCommand(TransactionCommands.mergeCoins(destination, sources));
  }

  Map<String, dynamic> publish(List<List<int>> modules, List<String> dependencies) {
    return addCommand(TransactionCommands.publish(modules, dependencies));
  }

  Map<String, dynamic> upgrade(
    List<List<int>> modules,
    List<String> dependencies,
    String packageId,
    Map<String, dynamic> ticket,
  ) {
    return addCommand(TransactionCommands.upgrade(modules, dependencies, packageId, ticket));
  }

  Map<String, dynamic> publishUpgrade(
    List<List<int>> modules,
    List<String> dependencies,
    String packageId,
    Map<String, dynamic> ticket,
  ) {
    return upgrade(modules, dependencies, packageId, ticket);
  }

  Map<String, dynamic> customUpgrade(
    List<List<int>> modules,
    List<String> dependencies,
    String packageId,
    Map<String, dynamic> ticket,
  ) {
    return upgrade(modules, dependencies, packageId, ticket);
  }

  Map<String, dynamic> makeMoveVec(String? typeTag, List<Map<String, dynamic>> elements) {
    return addCommand(TransactionCommands.makeMoveVec(typeTag, elements));
  }

  Map<String, dynamic> transferSui(String recipient, int amount) {
    final splitResult = splitCoins(gas(), [pure(_u64Bytes(amount))]);
    return transferObjects([splitResult], pure(utf8.encode(recipient)));
  }

  Map<String, dynamic> splitCoinEqual(Map<String, dynamic> coin, {required int splitCount, required int amountPerSplit}) {
    if (splitCount <= 0) {
      throw ArgumentError.value(splitCount, 'splitCount', 'must be positive');
    }
    final amounts = List.generate(splitCount, (_) => pure(_u64Bytes(amountPerSplit)));
    return splitCoins(coin, amounts);
  }

  Map<String, dynamic> splitCoinAndReturn(Map<String, dynamic> coin, {required int amount, required String recipient}) {
    final splitResult = splitCoins(coin, [pure(_u64Bytes(amount))]);
    transferObjects([splitResult], pure(utf8.encode(recipient)));
    return splitResult;
  }

  Map<String, dynamic> stakeCoin({
    required List<dynamic> coins,
    required String validatorAddress,
    int? amount,
    String systemStateObject = systemStateObjectId,
  }) {
    final coinArgs = coins.map((coin) => object(coin)).toList();
    final coinsVec = makeMoveVec(null, coinArgs);
    final amountArg = pure(_optionU64Bytes(amount));

    return moveCall(
      stakeRequestTarget,
      [
        object(systemStateObject),
        coinsVec,
        amountArg,
        pure(utf8.encode(validatorAddress)),
      ],
    );
  }

  Map<String, dynamic> unstakeCoin({required dynamic stakedCoin, String systemStateObject = systemStateObjectId}) {
    return moveCall(
      unstakeRequestTarget,
      [object(systemStateObject), object(stakedCoin)],
    );
  }

  Uint8List build() {
    final payload = <String, dynamic>{
      'Sender': data.sender,
      'Expiration': data.expiration,
      'GasData': data.gasData,
      'Inputs': data.inputs,
      'Commands': data.commands,
    };
    return Uint8List.fromList(utf8.encode(jsonEncode(payload)));
  }

  String buildBase64() => base64Encode(build());

  String serialize() => utf8.decode(build());

  Map<String, dynamic> getTransactionData() {
    return <String, dynamic>{
      'Sender': data.sender,
      'Expiration': data.expiration,
      'GasData': data.gasData,
      'Inputs': data.inputs,
      'Commands': data.commands,
    };
  }

  Map<String, dynamic> deferredExecution() {
    return {'sender': data.sender, 'tx_bytes': buildBase64()};
  }

  Future<Map<String, dynamic>> execute({
    dynamic client,
    List<String>? signatures,
    Map<String, dynamic>? options,
  }) async {
    final activeClient = _resolveClient(client);
    final txBytes = buildBase64();

    if (signatures == null && options == null) {
      return _rpcCall(activeClient, 'sui_executeTransactionBlock', [txBytes]);
    }
    return _rpcCall(
      activeClient,
      'sui_executeTransactionBlock',
      [txBytes, signatures ?? <String>[], options ?? <String, dynamic>{}],
    );
  }

  Future<Map<String, dynamic>> inspectAll({dynamic client, String? sender}) {
    final activeClient = _resolveClient(client);
    final activeSender = sender ?? data.sender;
    return _rpcCall(activeClient, 'sui_devInspectTransactionBlock', [activeSender, buildBase64()]);
  }

  Future<Map<String, int>> inspectForCost({dynamic client, String? sender}) async {
    final result = await inspectAll(client: client, sender: sender);
    final effects = (result['result'] as Map<String, dynamic>?)?['effects'] as Map<String, dynamic>?;
    final gasUsed = effects?['gasUsed'] as Map<String, dynamic>?;

    final computation = int.parse('${gasUsed?['computationCost'] ?? 0}');
    final storage = int.parse('${gasUsed?['storageCost'] ?? 0}');
    final rebate = int.parse('${gasUsed?['storageRebate'] ?? 0}');

    return {
      'computation_cost': computation,
      'storage_cost': storage,
      'storage_rebate': rebate,
      'total_cost': computation + storage - rebate,
    };
  }

  static Transaction fromSerialized(String serialized) {
    final raw = serialized.startsWith('{') ? utf8.encode(serialized) : base64Decode(serialized);
    final payload = jsonDecode(utf8.decode(raw)) as Map<String, dynamic>;

    final tx = Transaction();
    tx.data.sender = '${payload['Sender'] ?? ''}';
    tx.data.expiration = payload['Expiration'];
    tx.data.gasData.addAll((payload['GasData'] as Map?)?.cast<String, dynamic>() ?? <String, dynamic>{});
    tx.data.inputs.addAll(((payload['Inputs'] as List?) ?? const <dynamic>[])
        .map((e) => Map<String, dynamic>.from(e as Map))
        .toList());
    tx.data.commands.addAll(((payload['Commands'] as List?) ?? const <dynamic>[])
        .map((e) => Map<String, dynamic>.from(e as Map))
        .toList());
    return tx;
  }

  dynamic _resolveClient(dynamic injected) {
    final active = injected ?? client;
    if (active == null) {
      throw StateError('client is required for execution and inspection');
    }
    return active;
  }

  Future<Map<String, dynamic>> _rpcCall(dynamic c, String method, List<dynamic> params) async {
    try {
      final out = await c.call(method, params);
      return Map<String, dynamic>.from(out as Map);
    } catch (_) {
      final out = await c.execute(method, params);
      return Map<String, dynamic>.from(out as Map);
    }
  }

  static List<int> _u64Bytes(int value) {
    if (value < 0) {
      throw ArgumentError.value(value, 'value', 'u64 value must be non-negative');
    }
    final max = (BigInt.one << 64) - BigInt.one;
    if (BigInt.from(value) > max) {
      throw RangeError.value(value, 'value', 'u64 value out of range');
    }
    final out = ByteData(8)..setUint64(0, value, Endian.little);
    return out.buffer.asUint8List();
  }

  static List<int> _optionU64Bytes(int? value) {
    if (value == null) {
      return const [0];
    }
    return <int>[1, ..._u64Bytes(value)];
  }
}

class ResolveContext {
  ResolveContext({required this.transaction});

  final Transaction transaction;
  final List<Map<String, dynamic>> unresolvedInputs = <Map<String, dynamic>>[];
}

typedef ResolvePlugin = Future<void> Function(ResolveContext context);

class ResolverPluginError implements Exception {
  ResolverPluginError({required this.index, required this.pluginName, required this.cause});

  final int index;
  final String pluginName;
  final Object cause;

  Map<String, dynamic> toJson() {
    return {
      'error_type': 'ResolverPluginError',
      'index': index,
      'plugin_name': pluginName,
      'cause_type': cause.runtimeType.toString(),
      'cause_message': cause.toString(),
      'message': toString(),
    };
  }

  @override
  String toString() {
    return 'resolver plugin failed at index $index ($pluginName): $cause';
  }
}

class Resolver {
  final List<ResolvePlugin> _plugins = <ResolvePlugin>[];

  void addPlugin(ResolvePlugin plugin) {
    _plugins.add(plugin);
  }

  Future<ResolveContext> resolve(Transaction tx) async {
    final context = ResolveContext(transaction: tx);
    for (final input in tx.data.inputs) {
      if (input['\$kind'] == 'UnresolvedObject') {
        context.unresolvedInputs.add(input);
      }
    }

    for (var i = 0; i < _plugins.length; i++) {
      final plugin = _plugins[i];
      final pluginName = '$plugin';
      try {
        await plugin(context);
      } catch (e) {
        throw ResolverPluginError(index: i, pluginName: pluginName, cause: e);
      }
    }

    return context;
  }
}

class CachingExecutor {
  CachingExecutor(this.client);

  final dynamic client;
  final Map<String, Map<String, dynamic>> _cache = <String, Map<String, dynamic>>{};

  Future<Map<String, dynamic>> executeTransaction(Transaction tx) async {
    final key = tx.buildBase64();
    final cached = _cache[key];
    if (cached != null) {
      return cached;
    }

    Map<String, dynamic> result;
    try {
      final out = await client.call('sui_executeTransactionBlock', [key]);
      result = Map<String, dynamic>.from(out as Map);
    } catch (_) {
      final out = await client.execute('sui_executeTransactionBlock', [key]);
      result = Map<String, dynamic>.from(out as Map);
    }
    _cache[key] = result;
    return result;
  }
}

class SerialExecutor {
  SerialExecutor(this.executor);

  final CachingExecutor executor;

  Future<List<Map<String, dynamic>>> execute(List<Transaction> txs) async {
    final out = <Map<String, dynamic>>[];
    for (final tx in txs) {
      out.add(await executor.executeTransaction(tx));
    }
    return out;
  }
}

class ParallelExecutor {
  ParallelExecutor(this.executor, {this.maxWorkers = 4});

  final CachingExecutor executor;
  final int maxWorkers;

  Future<List<Map<String, dynamic>>> execute(List<Transaction> txs) async {
    final out = List<Map<String, dynamic>?>.filled(txs.length, null, growable: false);
    var next = 0;

    Future<void> worker() async {
      while (true) {
        final i = next;
        next += 1;
        if (i >= txs.length) {
          return;
        }
        out[i] = await executor.executeTransaction(txs[i]);
      }
    }

    await Future.wait(List.generate(maxWorkers, (_) => worker()));
    return out.whereType<Map<String, dynamic>>().toList();
  }
}
