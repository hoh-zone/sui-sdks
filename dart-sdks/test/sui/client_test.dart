import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockJsonRpcClient extends JsonRpcClient {
  _MockJsonRpcClient() : super(endpoint: 'https://example.invalid');

  final List<(String, List<dynamic>)> calls = <(String, List<dynamic>)>[];

  @override
  Future<Map<String, dynamic>> call(String method,
      [List<dynamic>? params]) async {
    final p = params ?? <dynamic>[];
    calls.add((method, p));

    if (method == 'rpc.discover') {
      return {
        'info': {'version': '1.2.3'}
      };
    }

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

class _PollingMockJsonRpcClient extends JsonRpcClient {
  _PollingMockJsonRpcClient() : super(endpoint: 'https://example.invalid');

  int attempts = 0;

  @override
  Future<Map<String, dynamic>> call(String method,
      [List<dynamic>? params]) async {
    if (method == 'sui_getTransactionBlock') {
      attempts += 1;
      if (attempts == 1) {
        throw StateError('temporary missing');
      }
      return {'digest': params![0]};
    }
    return <String, dynamic>{};
  }
}

void main() {
  test('discover and dry run', () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    await client.discoverRpcApi();
    await client.dryRun('AA==');
    await client.dryRunTransactionBlock('BB==');

    expect(mock.calls[0].$1, 'rpc.discover');
    expect(mock.calls[1].$1, 'sui_dryRunTransactionBlock');
    expect(mock.calls[1].$2, ['AA==']);
    expect(mock.calls[2].$1, 'sui_dryRunTransactionBlock');
    expect(mock.calls[2].$2, ['BB==']);
  });

  test('object and coin helpers', () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    await client.getObject('0x1', {'showContent': true});
    await client.getCoins(owner: '0xabc', cursor: 'c1', limit: 5);
    await client.getGas(owner: '0xabc');
    await client.getBalance(owner: '0xabc');

    expect(mock.calls[0].$1, 'sui_getObject');
    expect(mock.calls[1].$1, 'suix_getCoins');
    expect(mock.calls[2].$1, 'suix_getCoins');
    expect(mock.calls[3].$1, 'suix_getBalance');
  });

  test('iter checkpoints', () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    final items = await client.iterCheckpoints(limit: 1).toList();
    expect(items.map((x) => x['sequenceNumber']), ['1', '2']);
  });

  test('extended json-rpc wrappers', () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    expect(await client.getRpcApiVersion(), '1.2.3');
    await client.devInspectTransactionBlock(
        sender: '0xabc', txBytesB64: 'AA==');
    await client.executeTransactionBlock(
        transactionBlock: 'AA==', signatures: const ['sig']);
    await client.getTotalTransactionBlocks();
    await client.getNetworkMetrics();
    await client.getAddressMetrics();
    await client.getEpochMetrics(cursor: 'e1', limit: 5);
    await client.getAllEpochAddressMetrics(
        desc: 'true', cursor: 'a1', limit: 3);
    await client.getMoveCallMetrics();
    await client.getCurrentEpoch();
    await client.getEpochs(cursor: 'c1', limit: 2, descendingOrder: true);
    await client.getCurrentSystemState();
    await client.defaultNameServiceName('0xabc');
    await client
        .verifyZkLoginSignature({'bytes': 'AA=='}, 'sig', 'scope', '0xabc');

    expect(mock.calls[1].$1, 'sui_devInspectTransactionBlock');
    expect(mock.calls[2].$1, 'sui_executeTransactionBlock');
    expect(mock.calls[3].$1, 'sui_getTotalTransactionBlocks');
    expect(mock.calls[4].$1, 'suix_getNetworkMetrics');
    expect(mock.calls[5].$1, 'suix_getLatestAddressMetrics');
    expect(mock.calls[6].$1, 'suix_getEpochMetrics');
    expect(mock.calls[7].$1, 'suix_getAllEpochAddressMetrics');
    expect(mock.calls[8].$1, 'suix_getMoveCallMetrics');
    expect(mock.calls[9].$1, 'suix_getCurrentEpoch');
    expect(mock.calls[10].$1, 'suix_getEpochs');
    expect(mock.calls[11].$1, 'suix_getLatestSuiSystemState');
    expect(mock.calls[12].$1, 'suix_resolveNameServiceNames');
    expect(mock.calls[12].$2, ['0xabc', null, 1]);
    expect(mock.calls[13].$1, 'sui_verifyZkLoginSignature');
  });

  test('signAndExecuteTransaction with raw bytes', () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    await client.signAndExecuteTransaction(
      transaction: const [1, 2, 3],
      sender: '0xabc',
      signTransaction: (txBytes) async {
        expect(txBytes, [1, 2, 3]);
        return {
          'bytes': 'AQID',
          'signature': 'sig1',
        };
      },
    );

    expect(mock.calls[0].$1, 'sui_executeTransactionBlock');
    expect(mock.calls[0].$2, [
      'AQID',
      ['sig1'],
      <String, dynamic>{},
      null,
    ]);
  });

  test('signAndExecuteTransaction with Transaction and signatures[]', () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);
    final tx = Transaction();

    await client.signAndExecuteTransaction(
      transaction: tx,
      sender: '0xabc',
      signTransaction: (txBytes) async {
        expect(tx.data.sender, '0xabc');
        expect(txBytes.isNotEmpty, isTrue);
        return {
          'signatures': ['sigA', 'sigB'],
        };
      },
      requestType: 'WaitForEffectsCert',
    );

    expect(mock.calls[0].$1, 'sui_executeTransactionBlock');
    expect(mock.calls[0].$2[1], ['sigA', 'sigB']);
    expect(mock.calls[0].$2[3], 'WaitForEffectsCert');
  });

  test('waitForTransaction retries until found', () async {
    final mock = _PollingMockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    final tx = await client.waitForTransaction(
      digest: '0xtx',
      pollInterval: const Duration(milliseconds: 1),
      timeout: const Duration(seconds: 1),
    );

    expect(tx['digest'], '0xtx');
    expect(mock.attempts, 2);
  });

  test('list/getDynamicField aliases map to canonical json-rpc methods',
      () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    await client.listCoins(owner: '0xabc', cursor: 'c1', limit: 7);
    await client.listBalances(owner: '0xabc');
    await client.listOwnedObjects(owner: '0xabc', cursor: 'o1', limit: 3);
    await client.listDynamicFields('0xparent', 'd1', 2);
    await client.getDynamicField('0xparent', const {'type': '0x1::t::T'});

    expect(mock.calls[0].$1, 'suix_getCoins');
    expect(mock.calls[0].$2, ['0xabc', '0x2::sui::SUI', 'c1', 7]);
    expect(mock.calls[1].$1, 'suix_getAllBalances');
    expect(mock.calls[1].$2, ['0xabc']);
    expect(mock.calls[2].$1, 'suix_getOwnedObjects');
    expect(mock.calls[2].$2, ['0xabc', <String, dynamic>{}, 'o1', 3]);
    expect(mock.calls[3].$1, 'suix_getDynamicFields');
    expect(mock.calls[3].$2, ['0xparent', 'd1', 2]);
    expect(mock.calls[4].$1, 'suix_getDynamicFieldObject');
    expect(mock.calls[4].$2, [
      '0xparent',
      {'type': '0x1::t::T'}
    ]);
  });

  test('transaction aliases map to canonical json-rpc methods', () async {
    final mock = _MockJsonRpcClient();
    final client =
        SuiClient(endpoint: 'https://example.invalid', rpcClient: mock);

    await client.getTransaction('0xtx', const {'showEffects': true});
    await client.getTransactions(const ['0xt1', '0xt2']);
    await client.executeTransaction(
      transactionBlock: 'AA==',
      signatures: const ['sig'],
      requestType: 'WaitForEffectsCert',
    );
    await client.simulateTransaction('BB==');

    expect(mock.calls[0].$1, 'sui_getTransactionBlock');
    expect(mock.calls[0].$2, [
      '0xtx',
      {'showEffects': true}
    ]);
    expect(mock.calls[1].$1, 'sui_multiGetTransactionBlocks');
    expect(mock.calls[1].$2, [
      ['0xt1', '0xt2'],
      <String, dynamic>{}
    ]);
    expect(mock.calls[2].$1, 'sui_executeTransactionBlock');
    expect(mock.calls[2].$2, [
      'AA==',
      ['sig'],
      <String, dynamic>{},
      'WaitForEffectsCert',
    ]);
    expect(mock.calls[3].$1, 'sui_dryRunTransactionBlock');
    expect(mock.calls[3].$2, ['BB==']);
  });
}
