import 'package:grpc/grpc.dart' as grpc;
import 'package:protobuf/wkt.dart';

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
    case 'sui_getObject':
      return 'GetObject';
    case 'suix_getOwnedObjects':
      return 'GetOwnedObjects';
    case 'suix_getDynamicFields':
      return 'GetDynamicFields';
    case 'sui_getCheckpoints':
      return 'GetCheckpoints';
    case 'suix_getReferenceGasPrice':
      return 'GetReferenceGasPrice';
    case 'suix_getLatestSuiSystemState':
      return 'GetLatestSuiSystemState';
    case 'suix_resolveNameServiceAddress':
      return 'ResolveNameServiceAddress';
    case 'suix_resolveNameServiceNames':
      return 'ResolveNameServiceNames';
    case 'suix_queryEvents':
      return 'QueryEvents';
    case 'sui_multiGetObjects':
      return 'GetObjects';
    case 'sui_getTransactionBlock':
      return 'GetTransaction';
    case 'suix_queryTransactionBlocks':
      return 'QueryTransactionBlocks';
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
    final method = grpc.ClientMethod<Struct, Struct>(
      path,
      (Struct value) => value.writeToBuffer(),
      (List<int> value) => Struct.fromBuffer(value),
    );

    final req = Struct()..mergeFromProto3Json(<String, dynamic>{'method': mappedMethod, 'params': request.params});
    final call = _channel.createUnaryCall(
      method,
      req,
      options: grpc.CallOptions(
        timeout: timeout,
        metadata: request.metadata,
      ),
    );

    try {
      final response = await call;
      final parsed = response.toProto3Json() as Map<String, dynamic>;
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

  Future<Map<String, dynamic>> getLatestCheckpointSequenceNumber() {
    return call('sui_getLatestCheckpointSequenceNumber');
  }

  Future<Map<String, dynamic>> getCoins({
    required String owner,
    String coinType = '0x2::sui::SUI',
    String? cursor,
    int? limit,
  }) {
    return call('suix_getCoins', [owner, coinType, cursor, limit]);
  }

  Future<Map<String, dynamic>> getAllCoins({
    required String owner,
    String? cursor,
    int? limit,
  }) {
    return call('suix_getAllCoins', [owner, cursor, limit]);
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

  Future<Map<String, dynamic>> discoverRpcApi() {
    return call('rpc.discover');
  }

  Future<Map<String, dynamic>> getObject(String objectId, [Map<String, dynamic>? options]) {
    return call('sui_getObject', [objectId, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getOwnedObjects({
    required String owner,
    Map<String, dynamic>? query,
    String? cursor,
    int? limit,
  }) {
    return call('suix_getOwnedObjects', [owner, query ?? const <String, dynamic>{}, cursor, limit]);
  }

  Future<Map<String, dynamic>> getDynamicFields(String parentObjectId, {String? cursor, int? limit}) {
    return call('suix_getDynamicFields', [parentObjectId, cursor, limit]);
  }

  Future<Map<String, dynamic>> multiGetObjects(List<String> objectIds, [Map<String, dynamic>? options]) {
    return call('sui_multiGetObjects', [objectIds, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> getTransactionBlock(String digest, [Map<String, dynamic>? options]) {
    return call('sui_getTransactionBlock', [digest, options ?? const <String, dynamic>{}]);
  }

  Future<Map<String, dynamic>> queryTransactionBlocks({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return call('suix_queryTransactionBlocks', [query, cursor, limit, descendingOrder]);
  }

  Future<Map<String, dynamic>> queryEvents({
    required Map<String, dynamic> query,
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return call('suix_queryEvents', [query, cursor, limit, descendingOrder]);
  }

  Future<Map<String, dynamic>> getCheckpoints({
    String? cursor,
    int? limit,
    bool descendingOrder = false,
  }) {
    return call('sui_getCheckpoints', [cursor, limit, descendingOrder]);
  }

  Future<Map<String, dynamic>> getReferenceGasPrice() {
    return call('suix_getReferenceGasPrice');
  }

  Future<Map<String, dynamic>> getLatestSuiSystemState() {
    return call('suix_getLatestSuiSystemState');
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
}

typedef GrpcCoreClient = SuiGrpcClient;
