import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'jsonrpc.dart';
import 'pagination.dart';
import 'transactions.dart';

const String defaultCoinType = '0x2::sui::SUI';

class SuiClient {
  SuiClient({
    required this.endpoint,
    this.timeout = const Duration(seconds: 30),
    JsonRpcClient? rpcClient,
  }) : _rpcClient = rpcClient;

  final String endpoint;
  final Duration timeout;
  JsonRpcClient? _rpcClient;

  static SuiClient fromNetwork({
    String network = 'testnet',
    Duration timeout = const Duration(seconds: 30),
    JsonRpcTransport? transport,
  }) {
    final rpc = JsonRpcClient.fromNetwork(
        network: network, timeout: timeout, transport: transport);
    return SuiClient(endpoint: rpc.endpoint, timeout: timeout, rpcClient: rpc);
  }

  JsonRpcClient get rpc {
    _rpcClient ??= JsonRpcClient(endpoint: endpoint, timeout: timeout);
    return _rpcClient!;
  }

  Future<Map<String, dynamic>> execute(String method, [List<dynamic>? params]) {
    return rpc.call(method, params ?? const <dynamic>[]);
  }

  Future<Map<String, dynamic>> call(String method, [List<dynamic>? params]) {
    return execute(method, params);
  }

  Future<Map<String, dynamic>> discoverRpcApi() => execute('rpc.discover');

  Future<String?> getRpcApiVersion() async {
    final spec = await discoverRpcApi();
    final info = spec['info'];
    if (info is Map<String, dynamic>) {
      final version = info['version'];
      if (version is String) {
        return version;
      }
    }
    return null;
  }

  Future<Map<String, dynamic>> dryRun(String txBytesB64) =>
      execute('sui_dryRunTransactionBlock', [txBytesB64]);

  Future<Map<String, dynamic>> dryRunTransactionBlock(String txBytesB64) {
    return dryRun(txBytesB64);
  }

  Future<Map<String, dynamic>> devInspectTransactionBlock({
    required String sender,
    required String txBytesB64,
    String? gasPrice,
    String? epoch,
    Map<String, dynamic>? additionalArgs,
  }) {
    return execute('sui_devInspectTransactionBlock',
        [sender, txBytesB64, gasPrice, epoch, additionalArgs]);
  }

  Future<Map<String, dynamic>> getObject(String objectId,
      [Map<String, dynamic>? options]) {
    return execute('sui_getObject', [objectId, options ?? <String, dynamic>{}]);
  }

  Future<List<Map<String, dynamic>>> getObjects(List<String> objectIds,
      [Map<String, dynamic>? options]) async {
    final out = <Map<String, dynamic>>[];
    for (final objectId in objectIds) {
      out.add(await getObject(objectId, options));
    }
    return out;
  }

  Future<Map<String, dynamic>> multiGetObjects(List<String> objectIds,
      [Map<String, dynamic>? options]) {
    return execute(
        'sui_multiGetObjects', [objectIds, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getEvents({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return execute('suix_queryEvents', [query, cursor, limit, descendingOrder]);
  }

  Stream<Map<String, dynamic>> iterEvents({
    required Map<String, dynamic> query,
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) => getEvents(
          query: query,
          cursor: c,
          limit: limit,
          descendingOrder: descendingOrder),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getPackage(String packageId) {
    return getObject(packageId, {
      'showType': true,
      'showOwner': true,
      'showPreviousTransaction': true,
      'showDisplay': false,
      'showContent': true,
      'showBcs': true,
      'showStorageRebate': true,
    });
  }

  Future<Map<String, dynamic>> getGas({
    required String owner,
    String coinType = defaultCoinType,
    String? cursor,
    int? limit,
  }) {
    return getCoins(
      owner: owner,
      coinType: coinType,
      cursor: cursor,
      limit: limit,
    );
  }

  Future<Map<String, dynamic>> getCoins({
    required String owner,
    String coinType = defaultCoinType,
    String? cursor,
    int? limit,
  }) {
    return execute('suix_getCoins', [owner, coinType, cursor, limit]);
  }

  Future<Map<String, dynamic>> getAllCoins(
      {required String owner, String? cursor, int? limit}) {
    return execute('suix_getAllCoins', [owner, cursor, limit]);
  }

  Future<Map<String, dynamic>> listCoins({
    required String owner,
    String coinType = defaultCoinType,
    String? cursor,
    int? limit,
  }) {
    return getCoins(
      owner: owner,
      coinType: coinType,
      cursor: cursor,
      limit: limit,
    );
  }

  Stream<Map<String, dynamic>> iterAllCoins(
      {required String owner, String? cursor, int limit = 100, int? maxItems}) {
    return paginate(
      (c) => getAllCoins(owner: owner, cursor: c, limit: limit),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getBalance(
      {required String owner, String coinType = defaultCoinType}) {
    return execute('suix_getBalance', [owner, coinType]);
  }

  Future<Map<String, dynamic>> getAllBalances({required String owner}) {
    return execute('suix_getAllBalances', [owner]);
  }

  Future<Map<String, dynamic>> listBalances({required String owner}) {
    return getAllBalances(owner: owner);
  }

  Future<Map<String, dynamic>> getCoinMetadata(String coinType) {
    return execute('suix_getCoinMetadata', [coinType]);
  }

  Future<Map<String, dynamic>> getTotalSupply(String coinType) {
    return execute('suix_getTotalSupply', [coinType]);
  }

  Future<Map<String, dynamic>> getOwnedObjects({
    required String owner,
    Map<String, dynamic>? query,
    String? cursor,
    int? limit,
  }) {
    return execute('suix_getOwnedObjects',
        [owner, query ?? <String, dynamic>{}, cursor, limit]);
  }

  Future<Map<String, dynamic>> listOwnedObjects({
    required String owner,
    Map<String, dynamic>? query,
    String? cursor,
    int? limit,
  }) {
    return getOwnedObjects(
      owner: owner,
      query: query,
      cursor: cursor,
      limit: limit,
    );
  }

  Stream<Map<String, dynamic>> iterOwnedObjects({
    required String owner,
    Map<String, dynamic>? query,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) =>
          getOwnedObjects(owner: owner, query: query, cursor: c, limit: limit),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getDynamicFields(String parentObjectId,
      [String? cursor, int? limit]) {
    return execute('suix_getDynamicFields', [parentObjectId, cursor, limit]);
  }

  Future<Map<String, dynamic>> listDynamicFields(
    String parentObjectId, [
    String? cursor,
    int? limit,
  ]) {
    return getDynamicFields(parentObjectId, cursor, limit);
  }

  Stream<Map<String, dynamic>> iterDynamicFields({
    required String parentObjectId,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) => getDynamicFields(parentObjectId, c, limit),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getDynamicFieldObject(
      String parentObjectId, Map<String, dynamic> name) {
    return execute('suix_getDynamicFieldObject', [parentObjectId, name]);
  }

  Future<Map<String, dynamic>> getDynamicField(
    String parentObjectId,
    Map<String, dynamic> name,
  ) {
    return getDynamicFieldObject(parentObjectId, name);
  }

  Future<Map<String, dynamic>> getLatestSuiSystemState() =>
      execute('suix_getLatestSuiSystemState');

  Future<Map<String, dynamic>> getCurrentSystemState() =>
      getLatestSuiSystemState();

  Future<Map<String, dynamic>> getReferenceGasPrice() =>
      execute('suix_getReferenceGasPrice');

  Future<Map<String, dynamic>> getLatestCheckpointSequenceNumber() =>
      execute('sui_getLatestCheckpointSequenceNumber');

  Future<Map<String, dynamic>> queryTransactionBlocks({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return execute(
        'suix_queryTransactionBlocks', [query, cursor, limit, descendingOrder]);
  }

  Stream<Map<String, dynamic>> iterTransactionBlocks({
    required Map<String, dynamic> query,
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) => queryTransactionBlocks(
          query: query,
          cursor: c,
          limit: limit,
          descendingOrder: descendingOrder),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getTransactionBlock(String digest,
      [Map<String, dynamic>? options]) {
    return execute(
        'sui_getTransactionBlock', [digest, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getTransaction(String digest,
      [Map<String, dynamic>? options]) {
    return getTransactionBlock(digest, options);
  }

  Future<Map<String, dynamic>> multiGetTransactionBlocks(List<String> digests,
      [Map<String, dynamic>? options]) {
    return execute('sui_multiGetTransactionBlocks',
        [digests, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getTransactions(List<String> digests,
      [Map<String, dynamic>? options]) {
    return multiGetTransactionBlocks(digests, options);
  }

  Future<Map<String, dynamic>> executeTransactionBlock({
    required String transactionBlock,
    required List<String> signatures,
    Map<String, dynamic>? options,
    String? requestType,
  }) {
    return execute('sui_executeTransactionBlock', [
      transactionBlock,
      signatures,
      options ?? <String, dynamic>{},
      requestType,
    ]);
  }

  Future<Map<String, dynamic>> executeTransaction({
    required String transactionBlock,
    required List<String> signatures,
    Map<String, dynamic>? options,
    String? requestType,
  }) {
    return executeTransactionBlock(
      transactionBlock: transactionBlock,
      signatures: signatures,
      options: options,
      requestType: requestType,
    );
  }

  Future<Map<String, dynamic>> simulateTransaction(String txBytesB64) {
    return dryRunTransactionBlock(txBytesB64);
  }

  Future<Map<String, dynamic>> signAndExecuteTransaction({
    required Object transaction,
    required String sender,
    required Future<Map<String, dynamic>> Function(Uint8List txBytes)
        signTransaction,
    Map<String, dynamic>? options,
    String? requestType,
  }) async {
    final Uint8List txBytes;
    if (transaction is Uint8List) {
      txBytes = transaction;
    } else if (transaction is List<int>) {
      txBytes = Uint8List.fromList(transaction);
    } else if (transaction is Transaction) {
      transaction.setSenderIfNotSet(sender);
      txBytes = transaction.build();
    } else {
      throw ArgumentError.value(transaction, 'transaction',
          'must be Uint8List, List<int>, or Transaction');
    }

    final signed = await signTransaction(txBytes);
    final txB64 = signed['bytes'] as String? ?? base64Encode(txBytes);
    final sig = signed['signature'];
    final sigs = signed['signatures'];

    final signatures = sigs is List
        ? sigs.map((e) => e.toString()).toList()
        : sig != null
            ? <String>[sig.toString()]
            : const <String>[];
    if (signatures.isEmpty) {
      throw StateError('signTransaction must return signature or signatures');
    }

    return executeTransactionBlock(
      transactionBlock: txB64,
      signatures: signatures,
      options: options,
      requestType: requestType,
    );
  }

  Future<Map<String, dynamic>> waitForTransaction({
    required String digest,
    Map<String, dynamic>? options,
    Duration timeout = const Duration(seconds: 20),
    Duration pollInterval = const Duration(seconds: 1),
  }) async {
    final deadline = DateTime.now().add(timeout);
    Object? lastError;
    while (DateTime.now().isBefore(deadline)) {
      try {
        final result = await getTransactionBlock(digest, options);
        if (result.isNotEmpty) {
          return result;
        }
      } catch (err) {
        lastError = err;
      }
      if (DateTime.now().isBefore(deadline)) {
        await Future<void>.delayed(pollInterval);
      }
    }
    throw TimeoutException(
      'transaction $digest was not found before timeout'
      '${lastError != null ? ' (last error: $lastError)' : ''}',
      timeout,
    );
  }

  Future<Map<String, dynamic>> getEventsByTransaction(
      String transactionDigest) {
    return execute('sui_getEvents', [transactionDigest]);
  }

  Future<Map<String, dynamic>> getCheckpoint(String checkpointId) {
    return execute('sui_getCheckpoint', [checkpointId]);
  }

  Future<Map<String, dynamic>> getCheckpoints(
      {String? cursor, int? limit, bool descendingOrder = false}) {
    return execute('sui_getCheckpoints', [cursor, limit, descendingOrder]);
  }

  Stream<Map<String, dynamic>> iterCheckpoints({
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) => getCheckpoints(
          cursor: c, limit: limit, descendingOrder: descendingOrder),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getCommitteeInfo([String? epoch]) =>
      execute('suix_getCommitteeInfo', [epoch]);

  Future<Map<String, dynamic>> getProtocolConfig([String? version]) =>
      execute('sui_getProtocolConfig', [version]);

  Future<Map<String, dynamic>> getChainIdentifier() =>
      execute('sui_getChainIdentifier');

  Future<Map<String, dynamic>> getTotalTransactionBlocks() =>
      execute('sui_getTotalTransactionBlocks');

  Future<Map<String, dynamic>> resolveNameServiceAddress(String name) {
    return execute('suix_resolveNameServiceAddress', [name]);
  }

  Future<Map<String, dynamic>> resolveNameServiceNames(String address,
      {String? cursor, int? limit}) {
    return execute('suix_resolveNameServiceNames', [address, cursor, limit]);
  }

  Future<Map<String, dynamic>> defaultNameServiceName(String address) {
    return resolveNameServiceNames(address, limit: 1);
  }

  Future<Map<String, dynamic>> getValidatorsApy() =>
      execute('suix_getValidatorsApy');

  Future<Map<String, dynamic>> getNetworkMetrics() =>
      execute('suix_getNetworkMetrics');

  Future<Map<String, dynamic>> getAddressMetrics() =>
      execute('suix_getLatestAddressMetrics');

  Future<Map<String, dynamic>> getEpochMetrics({String? cursor, int? limit}) {
    return execute('suix_getEpochMetrics', [cursor, limit]);
  }

  Future<Map<String, dynamic>> getAllEpochAddressMetrics(
      {String? desc, String? cursor, int? limit}) {
    return execute('suix_getAllEpochAddressMetrics', [desc, cursor, limit]);
  }

  Future<Map<String, dynamic>> getMoveCallMetrics() =>
      execute('suix_getMoveCallMetrics');

  Future<Map<String, dynamic>> getCurrentEpoch() =>
      execute('suix_getCurrentEpoch');

  Future<Map<String, dynamic>> getEpochs(
      {String? cursor, int? limit, bool descendingOrder = false}) {
    return execute('suix_getEpochs', [cursor, limit, descendingOrder]);
  }

  Future<Map<String, dynamic>> getStakes(String owner) =>
      execute('suix_getStakes', [owner]);

  Future<Map<String, dynamic>> getStakesByIds(List<String> stakedSuiIds) {
    return execute('suix_getStakesByIds', [stakedSuiIds]);
  }

  Future<Map<String, dynamic>> tryGetPastObject(String objectId, int version,
      [Map<String, dynamic>? options]) {
    return execute('sui_tryGetPastObject',
        [objectId, version, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> tryMultiGetPastObjects(
      List<Map<String, dynamic>> pastObjects,
      [Map<String, dynamic>? options]) {
    return execute('sui_tryMultiGetPastObjects',
        [pastObjects, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveModulesByPackage(
      String packageId) {
    return execute('sui_getNormalizedMoveModulesByPackage', [packageId]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveModule(
      String packageId, String moduleName) {
    return execute('sui_getNormalizedMoveModule', [packageId, moduleName]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveFunction(
      String packageId, String moduleName, String functionName) {
    return execute(
        'sui_getNormalizedMoveFunction', [packageId, moduleName, functionName]);
  }

  Future<Map<String, dynamic>> getMoveFunctionArgTypes(
      String packageId, String moduleName, String functionName) {
    return execute(
        'sui_getMoveFunctionArgTypes', [packageId, moduleName, functionName]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveStruct(
      String packageId, String moduleName, String structName) {
    return execute(
        'sui_getNormalizedMoveStruct', [packageId, moduleName, structName]);
  }

  Future<Map<String, dynamic>> verifyZkLoginSignature(
      Map<String, dynamic> bytes,
      String signature,
      String intentScope,
      String author) {
    return execute(
        'sui_verifyZkLoginSignature', [bytes, signature, intentScope, author]);
  }

  Future<void> close() async {}
}
