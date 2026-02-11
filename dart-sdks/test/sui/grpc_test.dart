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
  test('grpc fromNetwork picks endpoint and validates network', () {
    expect(
        defaultGrpcEndpoints['mainnet'], 'https://fullnode.mainnet.sui.io:443');
    expect(
        defaultGrpcEndpoints['testnet'], 'https://fullnode.testnet.sui.io:443');
    expect(SuiGrpcClient.fromNetwork(network: 'mainnet'), isA<SuiGrpcClient>());
    expect(() => SuiGrpcClient.fromNetwork(network: 'localnet'),
        throwsArgumentError);
  });

  test('grpc call success', () async {
    final client = SuiGrpcClient(
      transport: _MockGrpcTransport([
        const GrpcResponse(raw: {
          'result': {'ok': true}
        }),
      ]),
    );

    final out = await client.call('sui_getLatestCheckpointSequenceNumber');
    expect(out['result'], isNotNull);
  });

  test('grpc execute alias', () async {
    final client = SuiGrpcClient(
      transport: _MockGrpcTransport([
        const GrpcResponse(raw: {
          'result': {'ok': true}
        }),
      ]),
    );

    final out = await client.execute('sui_getChainIdentifier');
    expect(out['result']['ok'], isTrue);
  });

  test('grpc getRpcApiVersion parses discovery result', () async {
    final client = SuiGrpcClient(
      transport: _MockGrpcTransport([
        const GrpcResponse(raw: {
          'info': {'version': '1.2.3'}
        }),
      ]),
    );

    final version = await client.getRpcApiVersion();
    expect(version, '1.2.3');
  });

  test('grpc alias helpers route to canonical rpc methods', () async {
    final calls = <Map<String, dynamic>>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        calls.add({'method': method, 'params': params});
        return {
          'result': {'ok': true}
        };
      },
    );

    await client.dryRunTransactionBlock('AA==');
    await client.getCurrentSystemState();

    expect(calls[0], {
      'method': 'sui_dryRunTransactionBlock',
      'params': ['AA==']
    });
    expect(calls[1],
        {'method': 'suix_getLatestSuiSystemState', 'params': <dynamic>[]});
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
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
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
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        return {
          'result': {'method': method}
        };
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
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        seen[method] = params;
        return {
          'result': {'ok': true}
        };
      },
    );

    await client.discoverRpcApi();
    await client.getCoins(owner: '0xabc');
    await client.getAllCoins(owner: '0xabc', cursor: 'n1', limit: 10);
    await client.getBalance(owner: '0xabc');
    await client.getAllBalances(owner: '0xabc');
    await client.getCoinMetadata('0x2::sui::SUI');
    await client.getTotalSupply('0x2::sui::SUI');
    await client.getObject('0x1', {'showType': true});
    await client.getOwnedObjects(
        owner: '0xabc', query: {'MatchAll': []}, cursor: 'c0', limit: 5);
    await client.getDynamicFields('0xparent', cursor: 'd1', limit: 7);
    await client.multiGetObjects(['0x1', '0x2'], {'showContent': true});
    await client.getTransactionBlock('digest', {'showEffects': true});
    await client.queryTransactionBlocks(
      query: {'FromAddress': '0xabc'},
      cursor: 'c1',
      limit: 20,
      descendingOrder: true,
    );
    await client.queryEvents(
        query: {'All': []}, cursor: 'e1', limit: 8, descendingOrder: true);
    await client.getCheckpoints(cursor: 'k1', limit: 3, descendingOrder: true);
    await client.getReferenceGasPrice();
    await client.getLatestSuiSystemState();
    await client.getCommitteeInfo('3');
    await client.getProtocolConfig('6');
    await client.getChainIdentifier();
    await client.getStakes('0xabc');
    await client.getStakesByIds(['0xstake1', '0xstake2']);
    await client.tryGetPastObject('0x1', 12, {'showContent': true});
    await client.tryMultiGetPastObjects(
      [
        {'objectId': '0x1', 'version': 12},
        {'objectId': '0x2', 'version': 99},
      ],
      {'showType': true},
    );
    await client.getNormalizedMoveModulesByPackage('0x2');
    await client.getNormalizedMoveModule('0x2', 'coin');
    await client.getNormalizedMoveFunction('0x2', 'coin', 'balance');
    await client.getMoveFunctionArgTypes('0x2', 'coin', 'balance');
    await client.getNormalizedMoveStruct('0x2', 'coin', 'Coin');
    await client.resolveNameServiceAddress('alice.sui');
    await client.resolveNameServiceNames(
        address: '0xabc', cursor: 'ns1', limit: 2);

    expect(seen['rpc.discover'], isEmpty);
    expect(seen['suix_getCoins'], ['0xabc', '0x2::sui::SUI', null, null]);
    expect(seen['suix_getAllCoins'], ['0xabc', 'n1', 10]);
    expect(seen['suix_getBalance'], ['0xabc', '0x2::sui::SUI']);
    expect(seen['suix_getAllBalances'], ['0xabc']);
    expect(seen['suix_getCoinMetadata'], ['0x2::sui::SUI']);
    expect(seen['suix_getTotalSupply'], ['0x2::sui::SUI']);
    expect(seen['sui_getObject'], [
      '0x1',
      {'showType': true}
    ]);
    expect(seen['suix_getOwnedObjects'], [
      '0xabc',
      {'MatchAll': []},
      'c0',
      5
    ]);
    expect(seen['suix_getDynamicFields'], ['0xparent', 'd1', 7]);
    expect(seen['sui_multiGetObjects'], [
      ['0x1', '0x2'],
      {'showContent': true}
    ]);
    expect(seen['sui_getTransactionBlock'], [
      'digest',
      {'showEffects': true}
    ]);
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
    expect(seen['suix_getCommitteeInfo'], ['3']);
    expect(seen['sui_getProtocolConfig'], ['6']);
    expect(seen['sui_getChainIdentifier'], isEmpty);
    expect(seen['suix_getStakes'], ['0xabc']);
    expect(seen['suix_getStakesByIds'], [
      ['0xstake1', '0xstake2']
    ]);
    expect(seen['sui_tryGetPastObject'], [
      '0x1',
      12,
      {'showContent': true}
    ]);
    expect(seen['sui_tryMultiGetPastObjects'], [
      [
        {'objectId': '0x1', 'version': 12},
        {'objectId': '0x2', 'version': 99},
      ],
      {'showType': true}
    ]);
    expect(seen['sui_getNormalizedMoveModulesByPackage'], ['0x2']);
    expect(seen['sui_getNormalizedMoveModule'], ['0x2', 'coin']);
    expect(seen['sui_getNormalizedMoveFunction'], ['0x2', 'coin', 'balance']);
    expect(seen['sui_getMoveFunctionArgTypes'], ['0x2', 'coin', 'balance']);
    expect(seen['sui_getNormalizedMoveStruct'], ['0x2', 'coin', 'Coin']);
    expect(seen['suix_resolveNameServiceAddress'], ['alice.sui']);
    expect(seen['suix_resolveNameServiceNames'], ['0xabc', 'ns1', 2]);
  });

  test('grpc helper aliases and extra methods pack params', () async {
    final calls = <Map<String, dynamic>>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        calls.add({'method': method, 'params': params});
        return {
          'result': {'ok': true}
        };
      },
    );

    await client.dryRun('AA==');
    await client.getGas(owner: '0xabc', cursor: 'g1', limit: 9);
    await client.getPackage('0x2');
    await client.getDynamicFieldObject(
        '0xparent', const {'type': '0x1::t::T', 'value': 'x'});
    await client
        .multiGetTransactionBlocks(['d1', 'd2'], const {'showEffects': true});
    await client.getEventsByTransaction('d1');
    await client.getEvents(
        query: const {'All': []},
        cursor: 'e2',
        limit: 2,
        descendingOrder: false);
    await client.getCheckpoint('100');
    await client.getValidatorsApy();
    await client.getNetworkMetrics();
    await client.getAddressMetrics();
    await client.getEpochMetrics(cursor: 'ep1', limit: 2);
    await client.getAllEpochAddressMetrics(
        desc: 'true', cursor: 'ep2', limit: 3);
    await client.getMoveCallMetrics();
    await client.getCurrentEpoch();
    await client.getEpochs(cursor: 'ep3', limit: 4, descendingOrder: true);
    await client.getTotalTransactionBlocks();
    await client.devInspectTransactionBlock(
        sender: '0xabc', txBytesB64: 'AA==');
    await client.executeTransactionBlock('AA==', const ['sig'],
        requestType: 'WaitForLocalExecution');
    await client
        .verifyZkLoginSignature({'bytes': 'AA=='}, 'sig', 'scope', '0xabc');

    expect(calls[0], {
      'method': 'sui_dryRunTransactionBlock',
      'params': ['AA==']
    });
    expect(calls[1], {
      'method': 'suix_getCoins',
      'params': ['0xabc', '0x2::sui::SUI', 'g1', 9]
    });
    expect(calls[2], {
      'method': 'sui_getObject',
      'params': [
        '0x2',
        {
          'showType': true,
          'showOwner': true,
          'showPreviousTransaction': true,
          'showDisplay': false,
          'showContent': true,
          'showBcs': true,
          'showStorageRebate': true,
        }
      ]
    });
    expect(calls[3], {
      'method': 'suix_getDynamicFieldObject',
      'params': [
        '0xparent',
        {'type': '0x1::t::T', 'value': 'x'}
      ]
    });
    expect(calls[4], {
      'method': 'sui_multiGetTransactionBlocks',
      'params': [
        ['d1', 'd2'],
        {'showEffects': true}
      ]
    });
    expect(calls[5], {
      'method': 'sui_getEvents',
      'params': ['d1']
    });
    expect(calls[6], {
      'method': 'suix_queryEvents',
      'params': [
        {'All': []},
        'e2',
        2,
        false
      ]
    });
    expect(calls[7], {
      'method': 'sui_getCheckpoint',
      'params': ['100']
    });
    expect(
        calls[8], {'method': 'suix_getValidatorsApy', 'params': <dynamic>[]});
    expect(
        calls[9], {'method': 'suix_getNetworkMetrics', 'params': <dynamic>[]});
    expect(calls[10],
        {'method': 'suix_getLatestAddressMetrics', 'params': <dynamic>[]});
    expect(calls[11], {
      'method': 'suix_getEpochMetrics',
      'params': ['ep1', 2]
    });
    expect(calls[12], {
      'method': 'suix_getAllEpochAddressMetrics',
      'params': ['true', 'ep2', 3]
    });
    expect(calls[13],
        {'method': 'suix_getMoveCallMetrics', 'params': <dynamic>[]});
    expect(
        calls[14], {'method': 'suix_getCurrentEpoch', 'params': <dynamic>[]});
    expect(calls[15], {
      'method': 'suix_getEpochs',
      'params': ['ep3', 4, true]
    });
    expect(calls[16],
        {'method': 'sui_getTotalTransactionBlocks', 'params': <dynamic>[]});
    expect(calls[17], {
      'method': 'sui_devInspectTransactionBlock',
      'params': ['0xabc', 'AA==', null, null, null]
    });
    expect(calls[18], {
      'method': 'sui_executeTransactionBlock',
      'params': [
        'AA==',
        ['sig'],
        <String, dynamic>{},
        'WaitForLocalExecution'
      ]
    });
    expect(calls[19], {
      'method': 'sui_verifyZkLoginSignature',
      'params': [
        {'bytes': 'AA=='},
        'sig',
        'scope',
        '0xabc'
      ]
    });
  });

  test('grpc pagination helpers iterate across pages', () async {
    final seen = <List<dynamic>>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        if (method != 'suix_getAllCoins') {
          return {
            'result': {
              'data': const <Map<String, dynamic>>[],
              'hasNextPage': false,
              'nextCursor': null
            }
          };
        }

        seen.add(params);
        final cursor = params[1] as String?;
        if (cursor == null) {
          return {
            'result': {
              'data': const [
                {'id': 'c1'},
                {'id': 'c2'},
              ],
              'hasNextPage': true,
              'nextCursor': 'p2',
            }
          };
        }

        return {
          'result': {
            'data': const [
              {'id': 'c3'},
            ],
            'hasNextPage': false,
            'nextCursor': null,
          }
        };
      },
    );

    final items = await client.iterAllCoins(owner: '0xabc', limit: 2).toList();
    expect(
      items.map((e) => e['id']).toList(),
      ['c1', 'c2', 'c3'],
    );
    expect(seen, [
      ['0xabc', null, 2],
      ['0xabc', 'p2', 2],
    ]);
  });

  test('grpc pagination helpers respect maxItems', () async {
    final seen = <List<dynamic>>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        if (method != 'suix_queryEvents') {
          return {
            'result': {
              'data': const <Map<String, dynamic>>[],
              'hasNextPage': false,
              'nextCursor': null
            }
          };
        }

        seen.add(params);
        return {
          'result': {
            'data': const [
              {'id': 'e1'},
              {'id': 'e2'},
              {'id': 'e3'},
            ],
            'hasNextPage': true,
            'nextCursor': 'ignored',
          }
        };
      },
    );

    final items = await client.iterEvents(
        query: const {'All': []},
        limit: 50,
        descendingOrder: true,
        maxItems: 2).toList();
    expect(
      items.map((e) => e['id']).toList(),
      ['e1', 'e2'],
    );
    expect(seen, [
      [
        {'All': []},
        null,
        50,
        true
      ]
    ]);
  });

  test('grpc waitForTransaction retries until found', () async {
    var attempts = 0;
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        if (method == 'sui_getTransactionBlock') {
          attempts += 1;
          if (attempts == 1) {
            throw StateError('temporary missing');
          }
          return {
            'result': {'digest': params[0]}
          };
        }
        return {'result': <String, dynamic>{}};
      },
    );

    final out = await client.waitForTransaction(
      digest: '0xtx',
      pollInterval: const Duration(milliseconds: 1),
      timeout: const Duration(seconds: 1),
    );
    expect((out['result'] as Map<String, dynamic>)['digest'], '0xtx');
    expect(attempts, 2);
  });

  test('grpc signAndExecuteTransaction forwards signed payload', () async {
    final calls = <Map<String, dynamic>>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        calls.add({'method': method, 'params': params});
        return {
          'result': {'ok': true}
        };
      },
    );

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
      requestType: 'WaitForEffectsCert',
    );

    expect(calls.single, {
      'method': 'sui_executeTransactionBlock',
      'params': [
        'AQID',
        ['sig1'],
        <String, dynamic>{},
        'WaitForEffectsCert'
      ]
    });
  });

  test('grpc list/getDynamicField aliases route to canonical methods',
      () async {
    final calls = <Map<String, dynamic>>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        calls.add({'method': method, 'params': params});
        return {
          'result': {'ok': true}
        };
      },
    );

    await client.listCoins(owner: '0xabc', cursor: 'c1', limit: 7);
    await client.listBalances(owner: '0xabc');
    await client.listOwnedObjects(owner: '0xabc', cursor: 'o1', limit: 3);
    await client.listDynamicFields('0xparent', cursor: 'd1', limit: 2);
    await client.getDynamicField('0xparent', const {'type': '0x1::t::T'});
    await client.getObjects(const ['0x1', '0x2']);
    await client.defaultNameServiceName('0xabc');

    expect(calls[0], {
      'method': 'suix_getCoins',
      'params': ['0xabc', '0x2::sui::SUI', 'c1', 7]
    });
    expect(calls[1], {
      'method': 'suix_getAllBalances',
      'params': ['0xabc']
    });
    expect(calls[2], {
      'method': 'suix_getOwnedObjects',
      'params': ['0xabc', <String, dynamic>{}, 'o1', 3]
    });
    expect(calls[3], {
      'method': 'suix_getDynamicFields',
      'params': ['0xparent', 'd1', 2]
    });
    expect(calls[4], {
      'method': 'suix_getDynamicFieldObject',
      'params': [
        '0xparent',
        {'type': '0x1::t::T'}
      ]
    });
    expect(calls[5], {
      'method': 'sui_multiGetObjects',
      'params': [
        ['0x1', '0x2'],
        <String, dynamic>{}
      ]
    });
    expect(calls[6], {
      'method': 'suix_resolveNameServiceNames',
      'params': ['0xabc', null, 1]
    });
  });

  test('grpc transaction aliases route to canonical methods', () async {
    final calls = <Map<String, dynamic>>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        calls.add({'method': method, 'params': params});
        return {
          'result': {'ok': true}
        };
      },
    );

    await client.getTransaction('0xtx', const {'showEffects': true});
    await client.getTransactions(const ['0xt1', '0xt2']);
    await client.executeTransaction(
      'AA==',
      const ['sig'],
      requestType: 'WaitForEffectsCert',
    );
    await client.simulateTransaction('BB==');

    expect(calls[0], {
      'method': 'sui_getTransactionBlock',
      'params': [
        '0xtx',
        {'showEffects': true}
      ]
    });
    expect(calls[1], {
      'method': 'sui_multiGetTransactionBlocks',
      'params': [
        ['0xt1', '0xt2'],
        <String, dynamic>{}
      ]
    });
    expect(calls[2], {
      'method': 'sui_executeTransactionBlock',
      'params': [
        'AA==',
        ['sig'],
        <String, dynamic>{},
        'WaitForEffectsCert'
      ]
    });
    expect(calls[3], {
      'method': 'sui_dryRunTransactionBlock',
      'params': ['BB==']
    });
  });

  test('grpc method mapper remaps jsonrpc to grpc-style names', () async {
    final seenMethods = <String>[];
    final client = SuiGrpcClient.fromStubInvoker(
      (
          {required String method,
          required List<dynamic> params,
          Map<String, String>? metadata}) async {
        seenMethods.add(method);
        return {
          'result': {'ok': true}
        };
      },
      methodMapper: defaultJsonRpcToGrpcMethodMapper,
    );

    await client.discoverRpcApi();
    await client.getCoins(owner: '0x1');
    await client.getBalance(owner: '0x1');
    await client.getCoinMetadata('0x2::sui::SUI');
    await client.getTotalSupply('0x2::sui::SUI');
    await client.getObject('0x1');
    await client.getOwnedObjects(owner: '0x1');
    await client.getDynamicFields('0xparent');
    await client.getDynamicFieldObject(
        '0xparent', const {'type': '0x1::t::T', 'value': 'x'});
    await client.multiGetObjects(['0x1']);
    await client.multiGetTransactionBlocks(const ['d1']);
    await client.getEventsByTransaction('d1');
    await client.getTransactionBlock('d');
    await client.queryTransactionBlocks(query: {'All': []});
    await client.queryEvents(query: {'All': []});
    await client.dryRun('AA==');
    await client.getCheckpoints();
    await client.getCheckpoint('1');
    await client.getReferenceGasPrice();
    await client.getLatestSuiSystemState();
    await client.getValidatorsApy();
    await client.getNetworkMetrics();
    await client.getAddressMetrics();
    await client.getEpochMetrics();
    await client.getAllEpochAddressMetrics();
    await client.getMoveCallMetrics();
    await client.getCurrentEpoch();
    await client.getEpochs();
    await client.getCommitteeInfo();
    await client.getProtocolConfig();
    await client.getChainIdentifier();
    await client.getTotalTransactionBlocks();
    await client.getStakes('0x1');
    await client.getStakesByIds(const ['0xstake1']);
    await client.tryGetPastObject('0x1', 1);
    await client.tryMultiGetPastObjects(const [
      {'objectId': '0x1', 'version': 1}
    ]);
    await client.getNormalizedMoveModulesByPackage('0x2');
    await client.getNormalizedMoveModule('0x2', 'coin');
    await client.getNormalizedMoveFunction('0x2', 'coin', 'balance');
    await client.getMoveFunctionArgTypes('0x2', 'coin', 'balance');
    await client.getNormalizedMoveStruct('0x2', 'coin', 'Coin');
    await client.resolveNameServiceAddress('alice.sui');
    await client.resolveNameServiceNames(address: '0x1');
    await client.devInspectTransactionBlock(sender: '0x1', txBytesB64: 'AA==');
    await client.verifyZkLoginSignature(
        const <String, dynamic>{}, 'sig', 'scope', '0x1');
    await client.executeTransactionBlock('AA==', const <String>[]);

    expect(
      seenMethods,
      [
        'Discover',
        'GetCoins',
        'GetBalance',
        'GetCoinMetadata',
        'GetTotalSupply',
        'GetObject',
        'GetOwnedObjects',
        'GetDynamicFields',
        'GetDynamicFieldObject',
        'GetObjects',
        'GetTransactions',
        'GetEvents',
        'GetTransaction',
        'QueryTransactionBlocks',
        'QueryEvents',
        'DryRunTransactionBlock',
        'GetCheckpoints',
        'GetCheckpoint',
        'GetReferenceGasPrice',
        'GetLatestSuiSystemState',
        'GetValidatorsApy',
        'GetNetworkMetrics',
        'GetLatestAddressMetrics',
        'GetEpochMetrics',
        'GetAllEpochAddressMetrics',
        'GetMoveCallMetrics',
        'GetCurrentEpoch',
        'GetEpochs',
        'GetCommitteeInfo',
        'GetProtocolConfig',
        'GetChainIdentifier',
        'GetTotalTransactionBlocks',
        'GetStakes',
        'GetStakesByIds',
        'TryGetPastObject',
        'TryMultiGetPastObjects',
        'GetNormalizedMoveModulesByPackage',
        'GetNormalizedMoveModule',
        'GetNormalizedMoveFunction',
        'GetMoveFunctionArgTypes',
        'GetNormalizedMoveStruct',
        'ResolveNameServiceAddress',
        'ResolveNameServiceNames',
        'DevInspectTransactionBlock',
        'VerifyZkLoginSignature',
        'ExecuteTransactionBlock',
      ],
    );
  });
}
