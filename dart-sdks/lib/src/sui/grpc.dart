import 'dart:convert';

import 'package:grpc/grpc.dart' as grpc;
import 'pagination.dart';

class GrpcRequest {
  const GrpcRequest({required this.method, this.params = const <dynamic>[], this.metadata});

  final String method;
  final List<dynamic> params;
  final Map<String, String>? metadata;
}

class GrpcResponse {
  const GrpcResponse({this.result, this.error, this.raw});

  final dynamic result;
  final dynamic error;
  final Map<String, dynamic>? raw;
}

abstract class GrpcTransport {
  Future<GrpcResponse> unary(GrpcRequest request);
}

typedef StubInvoker = Future<Map<String, dynamic>> Function({
  required String method,
  required List<dynamic> params,
  Map<String, String>? metadata,
});

typedef GrpcMethodMapper = String Function(String method);

String identityGrpcMethodMapper(String method) => method;

String defaultJsonRpcToGrpcMethodMapper(String method) {
  switch (method) {
    case 'rpc.discover':
      return 'Discover';
    case 'sui_getLatestCheckpointSequenceNumber':
      return 'GetLatestCheckpointSequenceNumber';
    case 'suix_getCoins':
      return 'GetCoins';
    case 'suix_getAllCoins':
      return 'GetAllCoins';
    case 'suix_getBalance':
      return 'GetBalance';
    case 'suix_getAllBalances':
      return 'GetAllBalances';
    case 'suix_getCoinMetadata':
      return 'GetCoinMetadata';
    case 'suix_getTotalSupply':
      return 'GetTotalSupply';
    case 'sui_getObject':
      return 'GetObject';
    case 'suix_getOwnedObjects':
      return 'GetOwnedObjects';
    case 'suix_getDynamicFields':
      return 'GetDynamicFields';
    case 'suix_getDynamicFieldObject':
      return 'GetDynamicFieldObject';
    case 'sui_getCheckpoints':
      return 'GetCheckpoints';
    case 'sui_getCheckpoint':
      return 'GetCheckpoint';
    case 'suix_getReferenceGasPrice':
      return 'GetReferenceGasPrice';
    case 'suix_getLatestSuiSystemState':
      return 'GetLatestSuiSystemState';
    case 'suix_getValidatorsApy':
      return 'GetValidatorsApy';
    case 'suix_getCommitteeInfo':
      return 'GetCommitteeInfo';
    case 'sui_getProtocolConfig':
      return 'GetProtocolConfig';
    case 'sui_getChainIdentifier':
      return 'GetChainIdentifier';
    case 'suix_getStakes':
      return 'GetStakes';
    case 'suix_getStakesByIds':
      return 'GetStakesByIds';
    case 'sui_tryGetPastObject':
      return 'TryGetPastObject';
    case 'sui_tryMultiGetPastObjects':
      return 'TryMultiGetPastObjects';
    case 'sui_getNormalizedMoveModulesByPackage':
      return 'GetNormalizedMoveModulesByPackage';
    case 'sui_getNormalizedMoveModule':
      return 'GetNormalizedMoveModule';
    case 'sui_getNormalizedMoveFunction':
      return 'GetNormalizedMoveFunction';
    case 'sui_getMoveFunctionArgTypes':
      return 'GetMoveFunctionArgTypes';
    case 'sui_getNormalizedMoveStruct':
      return 'GetNormalizedMoveStruct';
    case 'suix_resolveNameServiceAddress':
      return 'ResolveNameServiceAddress';
    case 'suix_resolveNameServiceNames':
      return 'ResolveNameServiceNames';
    case 'suix_queryEvents':
      return 'QueryEvents';
    case 'sui_multiGetObjects':
      return 'GetObjects';
    case 'sui_multiGetTransactionBlocks':
      return 'GetTransactions';
    case 'sui_getEvents':
      return 'GetEvents';
    case 'sui_getTransactionBlock':
      return 'GetTransaction';
    case 'suix_queryTransactionBlocks':
      return 'QueryTransactionBlocks';
    case 'sui_dryRunTransactionBlock':
      return 'DryRunTransactionBlock';
    case 'sui_executeTransactionBlock':
      return 'ExecuteTransactionBlock';
    default:
      return method;
  }
}

class StubGrpcTransport implements GrpcTransport {
  const StubGrpcTransport({
    required this.invoker,
    this.methodMapper = identityGrpcMethodMapper,
  });

  final StubInvoker invoker;
  final GrpcMethodMapper methodMapper;

  @override
  Future<GrpcResponse> unary(GrpcRequest request) async {
    final mappedMethod = methodMapper(request.method);
    final parsed = await invoker(
      method: mappedMethod,
      params: request.params,
      metadata: request.metadata,
    );

    if (parsed.containsKey('error')) {
      return GrpcResponse(error: parsed['error'], raw: parsed);
    }
    if (parsed.containsKey('result')) {
      return GrpcResponse(result: parsed['result'], raw: parsed);
    }
    return GrpcResponse(result: parsed, raw: parsed);
  }
}

class OfficialGrpcTransport implements GrpcTransport {
  OfficialGrpcTransport({
    required this.host,
    required this.port,
    this.service = 'sui.rpc.v2.Service',
    this.methodMapper = identityGrpcMethodMapper,
    this.options = const grpc.ChannelOptions(credentials: grpc.ChannelCredentials.insecure()),
    this.timeout = const Duration(seconds: 30),
    grpc.ClientChannel? channel,
  }) : _channel = channel ?? grpc.ClientChannel(host, port: port, options: options);

  final String host;
  final int port;
  final String service;
  final GrpcMethodMapper methodMapper;
  final grpc.ChannelOptions options;
  final Duration timeout;
  final grpc.ClientChannel _channel;

  @override
  Future<GrpcResponse> unary(GrpcRequest request) async {
    final mappedMethod = methodMapper(request.method);
    final path = _methodPath(mappedMethod);
    final method = grpc.ClientMethod<List<int>, List<int>>(
      path,
      (List<int> value) => value,
      (List<int> value) => value,
    );

    final req = utf8.encode(jsonEncode(<String, dynamic>{
      'method': mappedMethod,
      'params': request.params,
    }));
    final call = _channel.createCall(
      method,
      Stream<List<int>>.value(req),
      grpc.CallOptions(
        timeout: timeout,
        metadata: request.metadata,
      ),
    );

    try {
      final responseBytes = await call.response.single;
      final decoded = jsonDecode(utf8.decode(responseBytes));
      final parsed = decoded is Map<String, dynamic>
          ? decoded
          : <String, dynamic>{'result': decoded};
      if (parsed.containsKey('error')) {
        return GrpcResponse(error: parsed['error'], raw: parsed);
      }
      if (parsed.containsKey('result')) {
        return GrpcResponse(result: parsed['result'], raw: parsed);
      }
      return GrpcResponse(result: parsed, raw: parsed);
    } catch (e) {
      return GrpcResponse(error: {'message': e.toString()});
    }
  }

  String _methodPath(String method) {
    if (method.startsWith('/')) {
      return method;
    }
    return '/$service/$method';
  }

  Future<void> shutdown() => _channel.shutdown();
}

class SuiGrpcClient {
  SuiGrpcClient({required GrpcTransport transport}) : _transport = transport;

  final GrpcTransport _transport;

  factory SuiGrpcClient.fromStubInvoker(
    StubInvoker invoker, {
    GrpcMethodMapper methodMapper = identityGrpcMethodMapper,
  }) {
    return SuiGrpcClient(
      transport: StubGrpcTransport(
        invoker: invoker,
        methodMapper: methodMapper,
      ),
    );
  }

  factory SuiGrpcClient.fromEndpoint({
    required String host,
    int port = 443,
    String service = 'sui.rpc.v2.Service',
    GrpcMethodMapper methodMapper = identityGrpcMethodMapper,
    Duration timeout = const Duration(seconds: 30),
    grpc.ChannelOptions options = const grpc.ChannelOptions(credentials: grpc.ChannelCredentials.insecure()),
  }) {
    return SuiGrpcClient(
      transport: OfficialGrpcTransport(
        host: host,
        port: port,
        service: service,
        methodMapper: methodMapper,
        timeout: timeout,
        options: options,
      ),
    );
  }

  Future<GrpcResponse> unary(GrpcRequest request) async {
    final response = await _transport.unary(request);
    if (response.error != null) {
      throw StateError('grpc transport error: ${response.error}');
    }
    return response;
  }

  Future<Map<String, dynamic>> call(String method, [List<dynamic>? params]) async {
    final response = await unary(GrpcRequest(method: method, params: params ?? const <dynamic>[]));
    if (response.raw != null) {
      return response.raw!;
    }
    return <String, dynamic>{'result': response.result};
  }

  Future<Map<String, dynamic>> execute(String method, [List<dynamic>? params]) {
    return call(method, params);
  }

  Future<Map<String, dynamic>> getLatestCheckpointSequenceNumber() {
    return call('sui_getLatestCheckpointSequenceNumber');
  }

  Future<Map<String, dynamic>> dryRun(String txBytesB64) {
    return call('sui_dryRunTransactionBlock', [txBytesB64]);
  }

  Future<Map<String, dynamic>> getCoins({
    required String owner,
    String coinType = '0x2::sui::SUI',
    String? cursor,
    int? limit,
  }) {
    return call('suix_getCoins', [owner, coinType, cursor, limit]);
  }

  Future<Map<String, dynamic>> getGas({
    required String owner,
    String coinType = '0x2::sui::SUI',
    String? cursor,
    int? limit,
  }) {
    return getCoins(owner: owner, coinType: coinType, cursor: cursor, limit: limit);
  }

  Future<Map<String, dynamic>> getAllCoins({
    required String owner,
    String? cursor,
    int? limit,
  }) {
    return call('suix_getAllCoins', [owner, cursor, limit]);
  }

  Stream<Map<String, dynamic>> iterAllCoins({
    required String owner,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async {
        final page = await getAllCoins(owner: owner, cursor: c, limit: limit);
        final result = page['result'];
        return result is Map<String, dynamic> ? result : page;
      },
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getBalance({
    required String owner,
    String coinType = '0x2::sui::SUI',
  }) {
    return call('suix_getBalance', [owner, coinType]);
  }

  Future<Map<String, dynamic>> getAllBalances({required String owner}) {
    return call('suix_getAllBalances', [owner]);
  }

  Future<Map<String, dynamic>> getCoinMetadata(String coinType) {
    return call('suix_getCoinMetadata', [coinType]);
  }

  Future<Map<String, dynamic>> getTotalSupply(String coinType) {
    return call('suix_getTotalSupply', [coinType]);
  }

  Future<Map<String, dynamic>> discoverRpcApi() {
    return call('rpc.discover');
  }

  Future<Map<String, dynamic>> getObject(String objectId, [Map<String, dynamic>? options]) {
    return call('sui_getObject', [objectId, options ?? const <String, dynamic>{}]);
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

  Future<Map<String, dynamic>> getOwnedObjects({
    required String owner,
    Map<String, dynamic>? query,
    String? cursor,
    int? limit,
  }) {
    return call('suix_getOwnedObjects', [owner, query ?? const <String, dynamic>{}, cursor, limit]);
  }

  Stream<Map<String, dynamic>> iterOwnedObjects({
    required String owner,
    Map<String, dynamic>? query,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async {
        final page = await getOwnedObjects(owner: owner, query: query, cursor: c, limit: limit);
        final result = page['result'];
        return result is Map<String, dynamic> ? result : page;
      },
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getDynamicFields(String parentObjectId, {String? cursor, int? limit}) {
    return call('suix_getDynamicFields', [parentObjectId, cursor, limit]);
  }

  Future<Map<String, dynamic>> getDynamicFieldObject(
    String parentObjectId,
    Map<String, dynamic> name,
  ) {
    return call('suix_getDynamicFieldObject', [parentObjectId, name]);
  }

  Stream<Map<String, dynamic>> iterDynamicFields({
    required String parentObjectId,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async {
        final page = await getDynamicFields(parentObjectId, cursor: c, limit: limit);
        final result = page['result'];
        return result is Map<String, dynamic> ? result : page;
      },
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> multiGetObjects(List<String> objectIds, [Map<String, dynamic>? options]) {
    return call('sui_multiGetObjects', [objectIds, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getTransactionBlock(String digest, [Map<String, dynamic>? options]) {
    return call('sui_getTransactionBlock', [digest, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> multiGetTransactionBlocks(
    List<String> digests, [
    Map<String, dynamic>? options,
  ]) {
    return call('sui_multiGetTransactionBlocks', [digests, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getEventsByTransaction(String transactionDigest) {
    return call('sui_getEvents', [transactionDigest]);
  }

  Future<Map<String, dynamic>> queryTransactionBlocks({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return call('suix_queryTransactionBlocks', [query, cursor, limit, descendingOrder]);
  }

  Stream<Map<String, dynamic>> iterTransactionBlocks({
    required Map<String, dynamic> query,
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) async {
        final page =
            await queryTransactionBlocks(query: query, cursor: c, limit: limit, descendingOrder: descendingOrder);
        final result = page['result'];
        return result is Map<String, dynamic> ? result : page;
      },
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> queryEvents({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return call('suix_queryEvents', [query, cursor, limit, descendingOrder]);
  }

  Future<Map<String, dynamic>> getEvents({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return queryEvents(query: query, cursor: cursor, limit: limit, descendingOrder: descendingOrder);
  }

  Stream<Map<String, dynamic>> iterEvents({
    required Map<String, dynamic> query,
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) async {
        final page = await queryEvents(query: query, cursor: c, limit: limit, descendingOrder: descendingOrder);
        final result = page['result'];
        return result is Map<String, dynamic> ? result : page;
      },
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getCheckpoints({
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return call('sui_getCheckpoints', [cursor, limit, descendingOrder]);
  }

  Future<Map<String, dynamic>> getCheckpoint(String checkpointId) {
    return call('sui_getCheckpoint', [checkpointId]);
  }

  Stream<Map<String, dynamic>> iterCheckpoints({
    String? cursor,
    int limit = 100,
    bool descendingOrder = false,
    int? maxItems,
  }) {
    return paginate(
      (c) async {
        final page = await getCheckpoints(cursor: c, limit: limit, descendingOrder: descendingOrder);
        final result = page['result'];
        return result is Map<String, dynamic> ? result : page;
      },
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getReferenceGasPrice() {
    return call('suix_getReferenceGasPrice');
  }

  Future<Map<String, dynamic>> getLatestSuiSystemState() {
    return call('suix_getLatestSuiSystemState');
  }

  Future<Map<String, dynamic>> getValidatorsApy() {
    return call('suix_getValidatorsApy');
  }

  Future<Map<String, dynamic>> getCommitteeInfo([String? epoch]) {
    return call('suix_getCommitteeInfo', [epoch]);
  }

  Future<Map<String, dynamic>> getProtocolConfig([String? version]) {
    return call('sui_getProtocolConfig', [version]);
  }

  Future<Map<String, dynamic>> getChainIdentifier() {
    return call('sui_getChainIdentifier');
  }

  Future<Map<String, dynamic>> getStakes(String owner) {
    return call('suix_getStakes', [owner]);
  }

  Future<Map<String, dynamic>> getStakesByIds(List<String> stakedSuiIds) {
    return call('suix_getStakesByIds', [stakedSuiIds]);
  }

  Future<Map<String, dynamic>> tryGetPastObject(String objectId, int version, [Map<String, dynamic>? options]) {
    return call('sui_tryGetPastObject', [objectId, version, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> tryMultiGetPastObjects(
    List<Map<String, dynamic>> pastObjects, [
    Map<String, dynamic>? options,
  ]) {
    return call('sui_tryMultiGetPastObjects', [pastObjects, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveModulesByPackage(String packageId) {
    return call('sui_getNormalizedMoveModulesByPackage', [packageId]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveModule(String packageId, String moduleName) {
    return call('sui_getNormalizedMoveModule', [packageId, moduleName]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveFunction(
    String packageId,
    String moduleName,
    String functionName,
  ) {
    return call('sui_getNormalizedMoveFunction', [packageId, moduleName, functionName]);
  }

  Future<Map<String, dynamic>> getMoveFunctionArgTypes(
    String packageId,
    String moduleName,
    String functionName,
  ) {
    return call('sui_getMoveFunctionArgTypes', [packageId, moduleName, functionName]);
  }

  Future<Map<String, dynamic>> getNormalizedMoveStruct(
    String packageId,
    String moduleName,
    String structName,
  ) {
    return call('sui_getNormalizedMoveStruct', [packageId, moduleName, structName]);
  }

  Future<Map<String, dynamic>> resolveNameServiceAddress(String name) {
    return call('suix_resolveNameServiceAddress', [name]);
  }

  Future<Map<String, dynamic>> resolveNameServiceNames({
    required String address,
    String? cursor,
    int? limit,
  }) {
    return call('suix_resolveNameServiceNames', [address, cursor, limit]);
  }

  Future<Map<String, dynamic>> executeTransactionBlock(
    String txBytesB64,
    List<String> signatures, {
    Map<String, dynamic>? options,
  }) {
    return call('sui_executeTransactionBlock', [txBytesB64, signatures, options ?? const <String, dynamic>{}]);
  }

  Future<List<GrpcResponse>> batch(List<GrpcRequest> requests) {
    return Future.wait(requests.map(unary));
  }

  Future<void> close() async {
    if (_transport is OfficialGrpcTransport) {
      await (_transport as OfficialGrpcTransport).shutdown();
    }
  }
}

typedef GrpcCoreClient = SuiGrpcClient;
