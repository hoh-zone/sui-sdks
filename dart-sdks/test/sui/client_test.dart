import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockJsonRpcClient extends JsonRpcClient {
  _MockJsonRpcClient() : super(endpoint: 'https://example.invalid');

  final List<(String, List<dynamic>)> calls = <(String, List<dynamic>)>[];

  @override
  Future<Map<String, dynamic>> call(String method, [List<dynamic>? params]) async {
    final p = params ?? <dynamic>[];
    calls.add((method, p));

    if (method == 'sui_getCheckpoints') {
      final cursor = p[0] as String?;
      if (cursor == null) {
        return {
          'data': [
            {'sequenceNumber': '1'}
          ],
          'hasNextPage': true,
          'nextCursor': '1',
        };
      }
      return {
        'data': [
          {'sequenceNumber': '2'}
        ],
        'hasNextPage': false,
        'nextCursor': null,
      };
    }

    return {
      'jsonrpc': '2.0',
      'id': 1,
      'result': {'method': method, 'params': p}
    };
  }
}

void main() {
  test('discover and dry run', () async {
    final mock = _MockJsonRpcClient();
    final client = SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    await client.discoverRpcApi();
    await client.dryRun('AA==');

    expect(mock.calls[0].$1, 'rpc.discover');
    expect(mock.calls[1].$1, 'sui_dryRunTransactionBlock');
    expect(mock.calls[1].$2, ['AA==']);
  });

  test('object and coin helpers', () async {
    final mock = _MockJsonRpcClient();
    final client = SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    await client.getObject('0x1', {'showContent': true});
    await client.getGas(owner: '0xabc');
    await client.getBalance(owner: '0xabc');

    expect(mock.calls[0].$1, 'sui_getObject');
    expect(mock.calls[1].$1, 'suix_getCoins');
    expect(mock.calls[2].$1, 'suix_getBalance');
  });

  test('iter checkpoints', () async {
    final mock = _MockJsonRpcClient();
    final client = SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    final items = await client.iterCheckpoints(limit: 1).toList();
    expect(items.map((x) => x['sequenceNumber']), ['1', '2']);
  });
}
