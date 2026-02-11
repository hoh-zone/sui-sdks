import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockGraphQlTransport implements GraphQlTransport {
  _MockGraphQlTransport(this.responses);

  final List<Map<String, dynamic>> responses;
  final List<(String, Map<String, dynamic>?)> calls =
      <(String, Map<String, dynamic>?)>[];

  @override
  Future<Map<String, dynamic>> execute(
    String query, {
    Map<String, dynamic>? variables,
  }) async {
    calls.add((query, variables));
    return responses.removeAt(0);
  }
}

void main() {
  test('fromNetwork picks graphql endpoint', () {
    expect(GraphQlClient.fromNetwork(network: 'mainnet').endpoint,
        defaultGraphQlEndpoints['mainnet']);
    expect(GraphQlClient.fromNetwork(network: 'testnet').endpoint,
        defaultGraphQlEndpoints['testnet']);
    expect(() => GraphQlClient.fromNetwork(network: 'localnet'),
        throwsArgumentError);
  });

  test('execute success', () async {
    final client = GraphQlClient(
      endpoint: 'https://example.invalid',
      transport: _MockGraphQlTransport([
        {
          'data': {'ok': true}
        }
      ]),
    );

    final out = await client.execute('query { ok }');
    expect(out.containsKey('data'), isTrue);
  });

  test('execute error surface', () async {
    final transport = _MockGraphQlTransport([
      {
        'errors': ['bad']
      }
    ]);

    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);
    await expectLater(client.execute('query { bad }'), throwsStateError);
  });

  test('dryRunTransactionBlock alias builds dry-run query', () async {
    final transport = _MockGraphQlTransport([
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    await client.dryRunTransactionBlock(
      txBytesB64: 'AA==',
      includeCommandResults: true,
    );

    expect(transport.calls[0].$1.contains('query simulateTransaction'), isTrue);
    expect(transport.calls[0].$2, {
      'transaction': {
        'bcs': {'value': 'AA=='}
      },
      'doGasSelection': false,
      'checksEnabled': true,
      'includeCommandResults': true,
    });
  });

  test('defaultNameServiceName alias routes to default suins query', () async {
    final transport = _MockGraphQlTransport([
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    await client.defaultNameServiceName(address: '0xabc');

    expect(transport.calls[0].$1.contains('query defaultSuinsName'), isTrue);
    expect(transport.calls[0].$2, {'address': '0xabc'});
  });

  test('graphql list/getObjects aliases reuse canonical queries', () async {
    final transport = _MockGraphQlTransport([
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    await client.listCoins(owner: '0xabc', cursor: 'c1', limit: 3);
    await client.listBalances(owner: '0xabc', cursor: 'b1', limit: 2);
    await client.listOwnedObjects(owner: '0xabc', cursor: 'o1', limit: 4);
    await client.listDynamicFields(
        parentId: '0xparent', cursor: 'd1', limit: 5);
    await client.getObjects(objectIds: const ['0x1', '0x2']);

    expect(transport.calls[0].$1.contains('query getCoins'), isTrue);
    expect(transport.calls[0].$2?['owner'], '0xabc');
    expect(transport.calls[1].$1.contains('query getAllBalances'), isTrue);
    expect(transport.calls[1].$2?['owner'], '0xabc');
    expect(transport.calls[2].$1.contains('query getOwnedObjects'), isTrue);
    expect(transport.calls[2].$2?['owner'], '0xabc');
    expect(transport.calls[3].$1.contains('query getDynamicFields'), isTrue);
    expect(transport.calls[3].$2?['parentId'], '0xparent');
    expect(transport.calls[4].$1.contains('query multiGetObjects'), isTrue);
    expect(transport.calls[4].$2, {
      'objectKeys': [
        {'address': '0x1'},
        {'address': '0x2'}
      ]
    });
  });

  test('graphql transaction aliases reuse canonical queries', () async {
    final transport = _MockGraphQlTransport([
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
      const <String, dynamic>{
        'data': <String, dynamic>{'ok': true}
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    await client.getTransaction(digest: '0xtx');
    await client.getTransactions(digests: const ['0xt1', '0xt2']);
    await client.executeTransaction(
      transactionDataBcs: 'AA==',
      signatures: const ['sig'],
    );
    await client.simulateTransactionBlock(txBytesB64: 'BB==');

    expect(transport.calls[0].$1.contains('query getTransactionBlock'), isTrue);
    expect(transport.calls[0].$2, {'digest': '0xtx'});
    expect(transport.calls[1].$1.contains('query multiGetTransactionBlocks'),
        isTrue);
    expect(transport.calls[1].$2, {
      'keys': ['0xt1', '0xt2']
    });
    expect(
        transport.calls[2].$1.contains('mutation executeTransaction'), isTrue);
    expect(transport.calls[2].$2, {
      'transactionDataBcs': 'AA==',
      'signatures': ['sig'],
    });
    expect(transport.calls[3].$1.contains('query simulateTransaction'), isTrue);
    expect(transport.calls[3].$2, {
      'transaction': {
        'bcs': {'value': 'BB=='}
      },
      'doGasSelection': false,
      'checksEnabled': true,
      'includeCommandResults': false,
    });
  });

  test('helper methods build graphql queries and variables', () async {
    final transport = _MockGraphQlTransport([
      ...List<Map<String, dynamic>>.generate(
          36,
          (_) => <String, dynamic>{
                'data': <String, dynamic>{'ok': true}
              }),
      <String, dynamic>{
        'data': <String, dynamic>{
          'address': <String, dynamic>{
            'objects': <String, dynamic>{
              'nodes': <Map<String, dynamic>>[
                <String, dynamic>{'id': 'a1'},
                <String, dynamic>{'id': 'a2'},
              ],
              'pageInfo': <String, dynamic>{
                'hasNextPage': false,
                'endCursor': null
              },
            }
          }
        }
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    await client.getBalance(owner: '0xabc');
    await client.getCoins(owner: '0xabc', cursor: 'c1', limit: 10);
    await client.getGas(owner: '0xabc', cursor: 'g1', limit: 12);
    await client.getOwnedObjects(owner: '0xabc', cursor: 'c2', limit: 8);
    await client.getDynamicFields(parentId: '0xparent', cursor: 'd1', limit: 3);
    await client.getTransactionBlock(digest: 'tx1');
    await client.executeTransactionBlock(
        transactionDataBcs: 'AA==', signatures: const ['sig']);
    await client.getChainIdentifier();
    await client.getReferenceGasPrice();
    await client.getAllBalances(owner: '0xabc', cursor: 'b1', limit: 4);
    await client.getCoinMetadata(coinType: '0x2::sui::SUI');
    await client.getDefaultSuinsName(address: '0xabc');
    await client.getObject(objectId: '0xobj');
    await client.multiGetObjects(objectIds: const ['0x1', '0x2']);
    await client.getEventsByTransaction(digest: 'tx1');
    await client.getMoveFunction(
        packageId: '0x2', moduleName: 'coin', functionName: 'balance');
    await client.queryTransactionBlocks(
        filter: const {'kind': 'ProgrammableTransaction'},
        cursor: 't1',
        limit: 7);
    await client
        .queryEvents(filter: const {'sender': '0xabc'}, cursor: 'e1', limit: 9);
    await client.simulateTransaction(
        transaction: const {'tx': 'json'}, includeCommandResults: true);
    await client.resolveTransaction(transaction: const {'tx': 'json'});
    await client.getCheckpoint(sequenceNumber: 7);
    await client.getCheckpoints(cursor: 'k1', limit: 5);
    await client.getLatestCheckpointSequenceNumber();
    await client.getProtocolConfig(version: 6);
    await client.getLatestSuiSystemState();
    await client.getCurrentSystemState();
    await client.getTotalSupply(coinType: '0x2::sui::SUI');
    await client.resolveNameServiceAddress(name: 'alice.sui');
    await client.resolveNameServiceNames(address: '0xabc');
    await client.getCommitteeInfo(epoch: 3, cursor: 'v1', limit: 11);
    await client.getStakes(owner: '0xabc', cursor: 's1', limit: 6);
    await client.getStakesByIds(stakedSuiIds: const ['0xstake1', '0xstake2']);
    await client.discoverRpcApi();
    await client.dryRun(txBytesB64: 'AA==', includeCommandResults: true);
    await client.getAllCoins(owner: '0xabc', cursor: 'c3', limit: 4);
    await client.multiGetTransactionBlocks(digests: const ['t1', 't2']);
    final allCoins =
        await client.iterAllCoins(owner: '0xabc', limit: 2).toList();

    expect(transport.calls[0].$1.contains('query getBalance'), isTrue);
    expect(
        transport.calls[0].$2, {'owner': '0xabc', 'coinType': '0x2::sui::SUI'});
    expect(transport.calls[1].$1.contains('query getCoins'), isTrue);
    expect(transport.calls[1].$2?['owner'], '0xabc');
    expect(transport.calls[1].$2?['cursor'], 'c1');
    expect(transport.calls[1].$2?['first'], 10);
    expect(transport.calls[2].$1.contains('query getCoins'), isTrue);
    expect(transport.calls[2].$2?['owner'], '0xabc');
    expect(transport.calls[2].$2?['cursor'], 'g1');
    expect(transport.calls[2].$2?['first'], 12);
    expect(transport.calls[3].$1.contains('query getOwnedObjects'), isTrue);
    expect(transport.calls[4].$1.contains('query getDynamicFields'), isTrue);
    expect(transport.calls[5].$1.contains('query getTransactionBlock'), isTrue);
    expect(transport.calls[5].$2, {'digest': 'tx1'});
    expect(
        transport.calls[6].$1.contains('mutation executeTransaction'), isTrue);
    expect(transport.calls[6].$2, {
      'transactionDataBcs': 'AA==',
      'signatures': ['sig'],
    });
    expect(transport.calls[7].$1.contains('query getChainIdentifier'), isTrue);
    expect(
        transport.calls[8].$1.contains('query getReferenceGasPrice'), isTrue);
    expect(transport.calls[9].$1.contains('query getAllBalances'), isTrue);
    expect(
        transport.calls[9].$2, {'owner': '0xabc', 'limit': 4, 'cursor': 'b1'});
    expect(transport.calls[10].$1.contains('query getCoinMetadata'), isTrue);
    expect(transport.calls[10].$2, {'coinType': '0x2::sui::SUI'});
    expect(transport.calls[11].$1.contains('query defaultSuinsName'), isTrue);
    expect(transport.calls[11].$2, {'address': '0xabc'});
    expect(transport.calls[12].$1.contains('query getObject'), isTrue);
    expect(transport.calls[12].$2, {'objectId': '0xobj'});
    expect(transport.calls[13].$1.contains('query multiGetObjects'), isTrue);
    expect(transport.calls[13].$2, {
      'objectKeys': [
        {'address': '0x1'},
        {'address': '0x2'}
      ]
    });
    expect(transport.calls[14].$1.contains('query getEventsByTransaction'),
        isTrue);
    expect(transport.calls[14].$2, {'digest': 'tx1', 'first': 50});
    expect(transport.calls[15].$1.contains('query getMoveFunction'), isTrue);
    expect(transport.calls[15].$2, {
      'package': '0x2',
      'module': 'coin',
      'function': 'balance',
    });
    expect(transport.calls[16].$1.contains('query queryTransactionBlocks'),
        isTrue);
    expect(transport.calls[16].$2, {
      'first': 7,
      'cursor': 't1',
      'filter': {'kind': 'ProgrammableTransaction'}
    });
    expect(transport.calls[17].$1.contains('query queryEvents'), isTrue);
    expect(transport.calls[17].$2, {
      'first': 9,
      'cursor': 'e1',
      'filter': {'sender': '0xabc'}
    });
    expect(
        transport.calls[18].$1.contains('query simulateTransaction'), isTrue);
    expect(transport.calls[18].$2, {
      'transaction': {'tx': 'json'},
      'doGasSelection': false,
      'checksEnabled': true,
      'includeCommandResults': true,
    });
    expect(transport.calls[19].$1.contains('query resolveTransaction'), isTrue);
    expect(transport.calls[19].$2, {
      'transaction': {'tx': 'json'},
      'doGasSelection': true,
    });
    expect(transport.calls[20].$1.contains('query getCheckpoint'), isTrue);
    expect(transport.calls[20].$2, {'sequenceNumber': 7});
    expect(transport.calls[21].$1.contains('query getCheckpoints'), isTrue);
    expect(
        transport.calls[21].$2, {'first': 5, 'cursor': 'k1', 'filter': null});
    expect(
        transport.calls[22].$1
            .contains('query getLatestCheckpointSequenceNumber'),
        isTrue);
    expect(transport.calls[23].$1.contains('query getProtocolConfig'), isTrue);
    expect(transport.calls[23].$2, {'version': 6});
    expect(
        transport.calls[24].$1.contains('query getCurrentSystemState'), isTrue);
    expect(
        transport.calls[25].$1.contains('query getCurrentSystemState'), isTrue);
    expect(transport.calls[26].$1.contains('query getTotalSupply'), isTrue);
    expect(transport.calls[26].$2, {'coinType': '0x2::sui::SUI'});
    expect(transport.calls[27].$1.contains('query resolveNameServiceAddress'),
        isTrue);
    expect(transport.calls[27].$2, {'name': 'alice.sui'});
    expect(transport.calls[28].$1.contains('query resolveNameServiceNames'),
        isTrue);
    expect(transport.calls[28].$2, {'address': '0xabc'});
    expect(transport.calls[29].$1.contains('query getCommitteeInfo'), isTrue);
    expect(transport.calls[29].$2, {'epochId': 3, 'first': 11, 'cursor': 'v1'});
    expect(transport.calls[30].$1.contains('query getOwnedObjects'), isTrue);
    expect(transport.calls[30].$2, {
      'owner': '0xabc',
      'limit': 6,
      'cursor': 's1',
      'filter': {'type': '0x3::staking_pool::StakedSui'}
    });
    expect(transport.calls[31].$1.contains('query multiGetObjects'), isTrue);
    expect(transport.calls[31].$2, {
      'objectKeys': [
        {'address': '0xstake1'},
        {'address': '0xstake2'}
      ]
    });
    expect(transport.calls[32].$1.contains('query discoverRpcApi'), isTrue);
    expect(
        transport.calls[33].$1.contains('query simulateTransaction'), isTrue);
    expect(transport.calls[33].$2, {
      'transaction': {
        'bcs': {'value': 'AA=='}
      },
      'doGasSelection': false,
      'checksEnabled': true,
      'includeCommandResults': true,
    });
    expect(transport.calls[34].$1.contains('query getCoins'), isTrue);
    expect(transport.calls[34].$2?['owner'], '0xabc');
    expect(transport.calls[34].$2?['cursor'], 'c3');
    expect(transport.calls[34].$2?['first'], 4);
    expect(transport.calls[35].$1.contains('query multiGetTransactionBlocks'),
        isTrue);
    expect(transport.calls[35].$2, {
      'keys': ['t1', 't2']
    });
    expect(allCoins.map((e) => e['id']).toList(), ['a1', 'a2']);
    expect(transport.calls[36].$2?['owner'], '0xabc');
    expect(transport.calls[36].$2?['first'], 2);
  });

  test('iter helpers paginate graph connections', () async {
    final transport = _MockGraphQlTransport([
      {
        'data': {
          'address': {
            'objects': {
              'nodes': [
                {'id': 'o1'},
                {'id': 'o2'},
              ],
              'pageInfo': {'hasNextPage': true, 'endCursor': 'next-1'}
            }
          }
        }
      },
      {
        'data': {
          'address': {
            'objects': {
              'nodes': [
                {'id': 'o3'},
              ],
              'pageInfo': {'hasNextPage': false, 'endCursor': null}
            }
          }
        }
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    final out =
        await client.iterOwnedObjects(owner: '0xabc', limit: 2).toList();
    expect(out.map((e) => e['id']).toList(), ['o1', 'o2', 'o3']);
    expect(transport.calls[0].$2?['cursor'], isNull);
    expect(transport.calls[1].$2?['cursor'], 'next-1');
  });

  test('iter all balances paginates balance connection', () async {
    final transport = _MockGraphQlTransport([
      {
        'data': {
          'address': {
            'balances': {
              'nodes': [
                {'coin': 'SUI'},
              ],
              'pageInfo': {'hasNextPage': true, 'endCursor': 'b-next'}
            }
          }
        }
      },
      {
        'data': {
          'address': {
            'balances': {
              'nodes': [
                {'coin': 'USDC'},
              ],
              'pageInfo': {'hasNextPage': false, 'endCursor': null}
            }
          }
        }
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    final out = await client.iterAllBalances(owner: '0xabc', limit: 1).toList();
    expect(out.map((e) => e['coin']).toList(), ['SUI', 'USDC']);
    expect(transport.calls[0].$2?['cursor'], isNull);
    expect(transport.calls[1].$2?['cursor'], 'b-next');
  });

  test('iter transactions and events paginate root connections', () async {
    final transport = _MockGraphQlTransport([
      {
        'data': {
          'transactions': {
            'nodes': [
              {'digest': 't1'},
            ],
            'pageInfo': {'hasNextPage': true, 'endCursor': 't-next'}
          }
        }
      },
      {
        'data': {
          'transactions': {
            'nodes': [
              {'digest': 't2'},
            ],
            'pageInfo': {'hasNextPage': false, 'endCursor': null}
          }
        }
      },
      {
        'data': {
          'events': {
            'nodes': [
              {'id': 'e1'},
            ],
            'pageInfo': {'hasNextPage': false, 'endCursor': null}
          }
        }
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    final txs = await client.iterTransactionBlocks(limit: 1).toList();
    expect(txs.map((e) => e['digest']).toList(), ['t1', 't2']);
    expect(transport.calls[0].$2?['cursor'], isNull);
    expect(transport.calls[1].$2?['cursor'], 't-next');

    final events = await client.iterEvents(limit: 1).toList();
    expect(events.map((e) => e['id']).toList(), ['e1']);
    expect(transport.calls[2].$2?['cursor'], isNull);
  });

  test('iter checkpoints paginates root checkpoint connection', () async {
    final transport = _MockGraphQlTransport([
      {
        'data': {
          'checkpoints': {
            'nodes': [
              {'sequenceNumber': 1},
            ],
            'pageInfo': {'hasNextPage': true, 'endCursor': 'k-next'}
          }
        }
      },
      {
        'data': {
          'checkpoints': {
            'nodes': [
              {'sequenceNumber': 2},
            ],
            'pageInfo': {'hasNextPage': false, 'endCursor': null}
          }
        }
      },
    ]);
    final client = GraphQlClient(
        endpoint: 'https://example.invalid', transport: transport);

    final out = await client.iterCheckpoints(limit: 1).toList();
    expect(out.map((e) => e['sequenceNumber']).toList(), [1, 2]);
    expect(transport.calls[0].$2?['cursor'], isNull);
    expect(transport.calls[1].$2?['cursor'], 'k-next');
  });
}
