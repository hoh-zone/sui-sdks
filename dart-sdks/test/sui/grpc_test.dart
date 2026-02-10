import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockGrpcTransport implements GrpcTransport {
  _MockGrpcTransport(this.responses);

  final List<GrpcResponse> responses;

  @override
  Future<GrpcResponse> unary(GrpcRequest request) async {
    return responses.removeAt(0);
  }
}

void main() {
  test('grpc call success', () async {
    final client = SuiGrpcClient(
      transport: _MockGrpcTransport([
        const GrpcResponse(raw: {'result': {'ok': true}}),
      ]),
    );

    final out = await client.call('sui_getLatestCheckpointSequenceNumber');
    expect(out['result'], isNotNull);
  });

  test('grpc error mapped', () async {
    final client = SuiGrpcClient(
      transport: _MockGrpcTransport([
        const GrpcResponse(error: {'code': -1}),
      ]),
    );

    await expectLater(client.call('bad_method'), throwsStateError);
  });

  test('grpc stub invoker adapter', () async {
    final client = SuiGrpcClient.fromStubInvoker(
      ({required String method, required List<dynamic> params, Map<String, String>? metadata}) async {
        return {
          'result': {
            'method': method,
            'params': params,
            'metadata': metadata ?? <String, String>{},
          }
        };
      },
    );

    final out = await client.call('sui_getObject', ['0x1']);
    expect(out['result']['method'], 'sui_getObject');
    expect((out['result']['params'] as List).first, '0x1');
  });

  test('grpc batch', () async {
    final client = SuiGrpcClient.fromStubInvoker(
      ({required String method, required List<dynamic> params, Map<String, String>? metadata}) async {
        return {'result': {'method': method}};
      },
    );

    final out = await client.batch(const [
      GrpcRequest(method: 'm1'),
      GrpcRequest(method: 'm2'),
    ]);
    expect(out.length, 2);
    expect(out[0].result['method'], 'm1');
    expect(out[1].result['method'], 'm2');
  });

  test('grpc helper methods pack params', () async {
    final seen = <String, List<dynamic>>{};
    final client = SuiGrpcClient.fromStubInvoker(
      ({required String method, required List<dynamic> params, Map<String, String>? metadata}) async {
        seen[method] = params;
        return {'result': {'ok': true}};
      },
    );

    await client.discoverRpcApi();
    await client.getCoins(owner: '0xabc');
    await client.getAllCoins(owner: '0xabc', cursor: 'n1', limit: 10);
    await client.getBalance(owner: '0xabc');
    await client.getAllBalances(owner: '0xabc');
    await client.getObject('0x1', {'showType': true});
    await client.getOwnedObjects(owner: '0xabc', query: {'MatchAll': []}, cursor: 'c0', limit: 5);
    await client.getDynamicFields('0xparent', cursor: 'd1', limit: 7);
    await client.multiGetObjects(['0x1', '0x2'], {'showContent': true});
    await client.getTransactionBlock('digest', {'showEffects': true});
    await client.queryTransactionBlocks(
      query: {'FromAddress': '0xabc'},
      cursor: 'c1',
      limit: 20,
      descendingOrder: true,
    );
    await client.queryEvents(query: {'All': []}, cursor: 'e1', limit: 8, descendingOrder: true);
    await client.getCheckpoints(cursor: 'k1', limit: 3, descendingOrder: true);
    await client.getReferenceGasPrice();
    await client.getLatestSuiSystemState();
    await client.resolveNameServiceAddress('alice.sui');
    await client.resolveNameServiceNames(address: '0xabc', cursor: 'ns1', limit: 2);

    expect(seen['rpc.discover'], isEmpty);
    expect(seen['suix_getCoins'], ['0xabc', '0x2::sui::SUI', null, null]);
    expect(seen['suix_getAllCoins'], ['0xabc', 'n1', 10]);
    expect(seen['suix_getBalance'], ['0xabc', '0x2::sui::SUI']);
    expect(seen['suix_getAllBalances'], ['0xabc']);
    expect(seen['sui_getObject'], ['0x1', {'showType': true}]);
    expect(seen['suix_getOwnedObjects'], ['0xabc', {'MatchAll': []}, 'c0', 5]);
    expect(seen['suix_getDynamicFields'], ['0xparent', 'd1', 7]);
    expect(seen['sui_multiGetObjects'], [
      ['0x1', '0x2'],
      {'showContent': true}
    ]);
    expect(seen['sui_getTransactionBlock'], ['digest', {'showEffects': true}]);
    expect(seen['suix_queryTransactionBlocks'], [
      {'FromAddress': '0xabc'},
      'c1',
      20,
      true
    ]);
    expect(seen['suix_queryEvents'], [
      {'All': []},
      'e1',
      8,
      true
    ]);
    expect(seen['sui_getCheckpoints'], ['k1', 3, true]);
    expect(seen['suix_getReferenceGasPrice'], isEmpty);
    expect(seen['suix_getLatestSuiSystemState'], isEmpty);
    expect(seen['suix_resolveNameServiceAddress'], ['alice.sui']);
    expect(seen['suix_resolveNameServiceNames'], ['0xabc', 'ns1', 2]);
  });

  test('grpc method mapper remaps jsonrpc to grpc-style names', () async {
    final seenMethods = <String>[];
    final client = SuiGrpcClient.fromStubInvoker(
      ({required String method, required List<dynamic> params, Map<String, String>? metadata}) async {
        seenMethods.add(method);
        return {'result': {'ok': true}};
      },
      methodMapper: defaultJsonRpcToGrpcMethodMapper,
    );

    await client.discoverRpcApi();
    await client.getCoins(owner: '0x1');
    await client.getBalance(owner: '0x1');
    await client.getObject('0x1');
    await client.getOwnedObjects(owner: '0x1');
    await client.getDynamicFields('0xparent');
    await client.multiGetObjects(['0x1']);
    await client.getTransactionBlock('d');
    await client.queryTransactionBlocks(query: {'All': []});
    await client.queryEvents(query: {'All': []});
    await client.getCheckpoints();
    await client.getReferenceGasPrice();
    await client.getLatestSuiSystemState();
    await client.resolveNameServiceAddress('alice.sui');
    await client.resolveNameServiceNames(address: '0x1');
    await client.executeTransactionBlock('AA==', const <String>[]);

    expect(
      seenMethods,
      [
        'Discover',
        'GetCoins',
        'GetBalance',
        'GetObject',
        'GetOwnedObjects',
        'GetDynamicFields',
        'GetObjects',
        'GetTransaction',
        'QueryTransactionBlocks',
        'QueryEvents',
        'GetCheckpoints',
        'GetReferenceGasPrice',
        'GetLatestSuiSystemState',
        'ResolveNameServiceAddress',
        'ResolveNameServiceNames',
        'ExecuteTransactionBlock',
      ],
    );
  });
}
