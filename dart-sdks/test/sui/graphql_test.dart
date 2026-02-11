import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockGraphQlTransport implements GraphQlTransport {
  _MockGraphQlTransport(this.responses);

  final List<Map<String, dynamic>> responses;
  final List<(String, Map<String, dynamic>?)> calls = <(String, Map<String, dynamic>?)>[];

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
  test('execute success', () async {
    final client = GraphQlClient(
      endpoint: 'https://example.invalid',
      transport: _MockGraphQlTransport([
        {'data': {'ok': true}}
      ]),
    );

    final out = await client.execute('query { ok }');
    expect(out.containsKey('data'), isTrue);
  });

  test('execute error surface', () async {
    final transport = _MockGraphQlTransport([
      {'errors': ['bad']}
    ]);

    final client = GraphQlClient(endpoint: 'https://example.invalid', transport: transport);
    await expectLater(client.execute('query { bad }'), throwsStateError);
  });

  test('helper methods build graphql queries and variables', () async {
    final transport = _MockGraphQlTransport([
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
      {'data': {'ok': true}},
    ]);
    final client = GraphQlClient(endpoint: 'https://example.invalid', transport: transport);

    await client.getBalance(owner: '0xabc');
    await client.getCoins(owner: '0xabc', cursor: 'c1', limit: 10);
    await client.getOwnedObjects(owner: '0xabc', cursor: 'c2', limit: 8);
    await client.getDynamicFields(parentId: '0xparent', cursor: 'd1', limit: 3);
    await client.getTransactionBlock(digest: 'tx1');
    await client.executeTransactionBlock(transactionDataBcs: 'AA==', signatures: const ['sig']);
    await client.getChainIdentifier();
    await client.getReferenceGasPrice();
    await client.getAllBalances(owner: '0xabc', cursor: 'b1', limit: 4);
    await client.getCoinMetadata(coinType: '0x2::sui::SUI');
    await client.getDefaultSuinsName(address: '0xabc');
    await client.getObject(objectId: '0xobj');
    await client.multiGetObjects(objectIds: const ['0x1', '0x2']);
    await client.getEventsByTransaction(digest: 'tx1');
    await client.getMoveFunction(packageId: '0x2', moduleName: 'coin', functionName: 'balance');
    await client.queryTransactionBlocks(filter: const {'kind': 'ProgrammableTransaction'}, cursor: 't1', limit: 7);
    await client.queryEvents(filter: const {'sender': '0xabc'}, cursor: 'e1', limit: 9);
    await client.simulateTransaction(transaction: const {'tx': 'json'}, includeCommandResults: true);
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

    expect(transport.calls[0].$1.contains('query getBalance'), isTrue);
    expect(transport.calls[0].$2, {'owner': '0xabc', 'coinType': '0x2::sui::SUI'});
    expect(transport.calls[1].$1.contains('query getCoins'), isTrue);
    expect(transport.calls[1].$2?['owner'], '0xabc');
    expect(transport.calls[1].$2?['cursor'], 'c1');
    expect(transport.calls[1].$2?['first'], 10);
    expect(transport.calls[2].$1.contains('query getOwnedObjects'), isTrue);
    expect(transport.calls[3].$1.contains('query getDynamicFields'), isTrue);
    expect(transport.calls[4].$1.contains('query getTransactionBlock'), isTrue);
    expect(transport.calls[4].$2, {'digest': 'tx1'});
    expect(transport.calls[5].$1.contains('mutation executeTransaction'), isTrue);
    expect(transport.calls[5].$2, {
      'transactionDataBcs': 'AA==',
      'signatures': ['sig'],
    });
    expect(transport.calls[6].$1.contains('query getChainIdentifier'), isTrue);
    expect(transport.calls[7].$1.contains('query getReferenceGasPrice'), isTrue);
    expect(transport.calls[8].$1.contains('query getAllBalances'), isTrue);
    expect(transport.calls[8].$2, {'owner': '0xabc', 'limit': 4, 'cursor': 'b1'});
    expect(transport.calls[9].$1.contains('query getCoinMetadata'), isTrue);
    expect(transport.calls[9].$2, {'coinType': '0x2::sui::SUI'});
    expect(transport.calls[10].$1.contains('query defaultSuinsName'), isTrue);
    expect(transport.calls[10].$2, {'address': '0xabc'});
    expect(transport.calls[11].$1.contains('query getObject'), isTrue);
    expect(transport.calls[11].$2, {'objectId': '0xobj'});
    expect(transport.calls[12].$1.contains('query multiGetObjects'), isTrue);
    expect(transport.calls[12].$2, {
      'objectKeys': [
        {'address': '0x1'},
        {'address': '0x2'}
      ]
    });
    expect(transport.calls[13].$1.contains('query getEventsByTransaction'), isTrue);
    expect(transport.calls[13].$2, {'digest': 'tx1', 'first': 50});
    expect(transport.calls[14].$1.contains('query getMoveFunction'), isTrue);
    expect(transport.calls[14].$2, {
      'package': '0x2',
      'module': 'coin',
      'function': 'balance',
    });
    expect(transport.calls[15].$1.contains('query queryTransactionBlocks'), isTrue);
    expect(transport.calls[15].$2, {
      'first': 7,
      'cursor': 't1',
      'filter': {'kind': 'ProgrammableTransaction'}
    });
    expect(transport.calls[16].$1.contains('query queryEvents'), isTrue);
    expect(transport.calls[16].$2, {
      'first': 9,
      'cursor': 'e1',
      'filter': {'sender': '0xabc'}
    });
    expect(transport.calls[17].$1.contains('query simulateTransaction'), isTrue);
    expect(transport.calls[17].$2, {
      'transaction': {'tx': 'json'},
      'doGasSelection': false,
      'checksEnabled': true,
      'includeCommandResults': true,
    });
    expect(transport.calls[18].$1.contains('query resolveTransaction'), isTrue);
    expect(transport.calls[18].$2, {
      'transaction': {'tx': 'json'},
      'doGasSelection': true,
    });
    expect(transport.calls[19].$1.contains('query getCheckpoint'), isTrue);
    expect(transport.calls[19].$2, {'sequenceNumber': 7});
    expect(transport.calls[20].$1.contains('query getCheckpoints'), isTrue);
    expect(transport.calls[20].$2, {'first': 5, 'cursor': 'k1', 'filter': null});
    expect(transport.calls[21].$1.contains('query getLatestCheckpointSequenceNumber'), isTrue);
    expect(transport.calls[22].$1.contains('query getProtocolConfig'), isTrue);
    expect(transport.calls[22].$2, {'version': 6});
    expect(transport.calls[23].$1.contains('query getCurrentSystemState'), isTrue);
    expect(transport.calls[24].$1.contains('query getCurrentSystemState'), isTrue);
    expect(transport.calls[25].$1.contains('query getTotalSupply'), isTrue);
    expect(transport.calls[25].$2, {'coinType': '0x2::sui::SUI'});
    expect(transport.calls[26].$1.contains('query resolveNameServiceAddress'), isTrue);
    expect(transport.calls[26].$2, {'name': 'alice.sui'});
    expect(transport.calls[27].$1.contains('query resolveNameServiceNames'), isTrue);
    expect(transport.calls[27].$2, {'address': '0xabc'});
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
    final client = GraphQlClient(endpoint: 'https://example.invalid', transport: transport);

    final out = await client.iterOwnedObjects(owner: '0xabc', limit: 2).toList();
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
    final client = GraphQlClient(endpoint: 'https://example.invalid', transport: transport);

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
    final client = GraphQlClient(endpoint: 'https://example.invalid', transport: transport);

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
    final client = GraphQlClient(endpoint: 'https://example.invalid', transport: transport);

    final out = await client.iterCheckpoints(limit: 1).toList();
    expect(out.map((e) => e['sequenceNumber']).toList(), [1, 2]);
    expect(transport.calls[0].$2?['cursor'], isNull);
    expect(transport.calls[1].$2?['cursor'], 'k-next');
  });
}
