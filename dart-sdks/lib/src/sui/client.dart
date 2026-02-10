import 'jsonrpc.dart';
import 'pagination.dart';

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
    final rpc = JsonRpcClient.fromNetwork(network: network, timeout: timeout, transport: transport);
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

  Future<Map<String, dynamic>> dryRun(String txBytesB64) => execute('sui_dryRunTransactionBlock', [txBytesB64]);

  Future<Map<String, dynamic>> getObject(String objectId, [Map<String, dynamic>? options]) {
    return execute('sui_getObject', [objectId, options ?? <String, dynamic>{}]);
  }

  Future<List<Map<String, dynamic>>> getObjects(List<String> objectIds, [Map<String, dynamic>? options]) async {
    final out = <Map<String, dynamic>>[];
    for (final objectId in objectIds) {
      out.add(await getObject(objectId, options));
    }
    return out;
  }

  Future<Map<String, dynamic>> multiGetObjects(List<String> objectIds, [Map<String, dynamic>? options]) {
    return execute('sui_multiGetObjects', [objectIds, options ?? <String, dynamic>{}]);
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
      (c) => getEvents(query: query, cursor: c, limit: limit, descendingOrder: descendingOrder),
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
    return execute('suix_getCoins', [owner, coinType, cursor, limit]);
  }

  Future<Map<String, dynamic>> getAllCoins({required String owner, String? cursor, int? limit}) {
    return execute('suix_getAllCoins', [owner, cursor, limit]);
  }

  Stream<Map<String, dynamic>> iterAllCoins({required String owner, String? cursor, int limit = 100, int? maxItems}) {
    return paginate(
      (c) => getAllCoins(owner: owner, cursor: c, limit: limit),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getBalance({required String owner, String coinType = defaultCoinType}) {
    return execute('suix_getBalance', [owner, coinType]);
  }

  Future<Map<String, dynamic>> getAllBalances({required String owner}) {
    return execute('suix_getAllBalances', [owner]);
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
    return execute('suix_getOwnedObjects', [owner, query ?? <String, dynamic>{}, cursor, limit]);
  }

  Stream<Map<String, dynamic>> iterOwnedObjects({
    required String owner,
    Map<String, dynamic>? query,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) => getOwnedObjects(owner: owner, query: query, cursor: c, limit: limit),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getDynamicFields(String parentObjectId, [String? cursor, int? limit]) {
    return execute('suix_getDynamicFields', [parentObjectId, cursor, limit]);
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

  Future<Map<String, dynamic>> getDynamicFieldObject(String parentObjectId, Map<String, dynamic> name) {
    return execute('suix_getDynamicFieldObject', [parentObjectId, name]);
  }

  Future<Map<String, dynamic>> getLatestSuiSystemState() => execute('suix_getLatestSuiSystemState');

  Future<Map<String, dynamic>> getReferenceGasPrice() => execute('suix_getReferenceGasPrice');

  Future<Map<String, dynamic>> getLatestCheckpointSequenceNumber() => execute('sui_getLatestCheckpointSequenceNumber');

  Future<Map<String, dynamic>> queryTransactionBlocks({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return execute('suix_queryTransactionBlocks', [query, cursor, limit, descendingOrder]);
  }

  Stream<Map<String, dynamic>> iterTransactionBlocks({
    required Map<String, dynamic> query,
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) => queryTransactionBlocks(query: query, cursor: c, limit: limit, descendingOrder: descendingOrder),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getTransactionBlock(String digest, [Map<String, dynamic>? options]) {
    return execute('sui_getTransactionBlock', [digest, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> multiGetTransactionBlocks(List<String> digests, [Map<String, dynamic>? options]) {
    return execute('sui_multiGetTransactionBlocks', [digests, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getEventsByTransaction(String transactionDigest) {
    return execute('sui_getEvents', [transactionDigest]);
  }

  Future<Map<String, dynamic>> getCheckpoint(String checkpointId) {
    return execute('sui_getCheckpoint', [checkpointId]);
  }

  Future<Map<String, dynamic>> getCheckpoints({String? cursor, int? limit, bool descendingOrder = false}) {
    return execute('sui_getCheckpoints', [cursor, limit, descendingOrder]);
  }

  Stream<Map<String, dynamic>> iterCheckpoints({
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) => getCheckpoints(cursor: c, limit: limit, descendingOrder: descendingOrder),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getCommitteeInfo([String? epoch]) => execute('suix_getCommitteeInfo', [epoch]);

  Future<Map<String, dynamic>> getProtocolConfig([String? version]) => execute('sui_getProtocolConfig', [version]);

  Future<Map<String, dynamic>> getChainIdentifier() => execute('sui_getChainIdentifier');

  Future<Map<String, dynamic>> resolveNameServiceAddress(String name) {
    return execute('suix_resolveNameServiceAddress', [name]);
  }

  Future<Map<String, dynamic>> resolveNameServiceNames(String address, {String? cursor, int? limit}) {
    return execute('suix_resolveNameServiceNames', [address, cursor, limit]);
  }

  Future<Map<String, dynamic>> getValidatorsApy() => execute('suix_getValidatorsApy');

  Future<Map<String, dynamic>> getStakes(String owner) => execute('suix_getStakes', [owner]);

  Future<Map<String, dynamic>> getStakesByIds(List<String> stakedSuiIds) {
    return execute('suix_getStakesByIds', [stakedSuiIds]);
  }

  Future<Map<String, dynamic>> tryGetPastObject(String objectId, int version, [Map<String, dynamic>? options]) {
    return execute('sui_tryGetPastObject', [objectId, version, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> tryMultiGetPastObjects(List<Map<String, dynamic>> pastObjects,
      [Map<String, dynamic>? options]) {
    return execute('sui_tryMultiGetPastObjects', [pastObjects, options ?? <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveModulesByPackage(String packageId) {
    return execute('sui_getNormalizedMoveModulesByPackage', [packageId]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveModule(String packageId, String moduleName) {
    return execute('sui_getNormalizedMoveModule', [packageId, moduleName]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveFunction(String packageId, String moduleName, String functionName) {
    return execute('sui_getNormalizedMoveFunction', [packageId, moduleName, functionName]);
  }

  Future<Map<String, dynamic>> getMoveFunctionArgTypes(String packageId, String moduleName, String functionName) {
    return execute('sui_getMoveFunctionArgTypes', [packageId, moduleName, functionName]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveStruct(String packageId, String moduleName, String structName) {
    return execute('sui_getNormalizedMoveStruct', [packageId, moduleName, structName]);
  }

  Future<void> close() async {}
}
