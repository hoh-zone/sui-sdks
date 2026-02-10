import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockClient {
  int calls = 0;

  Future<Map<String, dynamic>> call(String method, [List<dynamic>? params]) async {
    calls += 1;
    if (method == 'sui_devInspectTransactionBlock') {
      return {
        'result': {
          'effects': {
            'gasUsed': {
              'computationCost': '10',
              'storageCost': '20',
              'storageRebate': '5',
            }
          }
        }
      };
    }
    return {'method': method, 'params': params ?? <dynamic>[], 'calls': calls};
  }
}

void main() {
  test('build and restore', () {
    final tx = Transaction();
    tx.setSender('0x1');
    tx.setGasBudgetIfNotSet(100);
    tx.moveCall('0x2::foo::bar', [tx.object('0xabc')], ['0x2::sui::SUI']);

    final b64 = tx.buildBase64();
    final restored = Transaction.fromSerialized(b64);
    expect(restored.data.sender, '0x1');
    expect(restored.data.commands.length, 1);
  });

  test('resolver plugin pipeline', () async {
    final tx = Transaction();
    tx.object('0x123');

    var seen = 0;
    final resolver = Resolver()
      ..addPlugin((ctx) async {
        seen = ctx.unresolvedInputs.length;
      });

    await resolver.resolve(tx);
    expect(seen, 1);
  });

  test('caching executor', () async {
    final client = _MockClient();
    final executor = CachingExecutor(client);

    final tx = Transaction();
    tx.moveCall('0x2::foo::bar', [], []);

    final a = await executor.executeTransaction(tx);
    final b = await executor.executeTransaction(tx);
    expect(a, b);
    expect(client.calls, 1);
  });

  test('serial and parallel executor', () async {
    final client = _MockClient();
    final cacheExecutor = CachingExecutor(client);
    final serial = SerialExecutor(cacheExecutor);
    final parallel = ParallelExecutor(cacheExecutor, maxWorkers: 2);

    final tx1 = Transaction()..moveCall('0x2::m::f1', [], []);
    final tx2 = Transaction()..moveCall('0x2::m::f2', [], []);

    final out1 = await serial.execute([tx1, tx2]);
    expect(out1.length, 2);

    final out2 = await parallel.execute([tx1, tx2]);
    expect(out2.length, 2);
  });

  test('transaction execute and inspect', () async {
    final client = _MockClient();
    final tx = Transaction(client: client);
    tx.setSender('0x1');
    tx.moveCall('0x2::m::f1', [], []);

    final data = tx.getTransactionData();
    expect(data['Sender'], '0x1');

    final deferred = tx.deferredExecution();
    expect(deferred['sender'], '0x1');
    expect(deferred['tx_bytes'], isA<String>());

    final out = await tx.execute();
    expect(out['method'], 'sui_executeTransactionBlock');

    final out2 = await tx.execute(signatures: ['sig1'], options: {'showEffects': true});
    expect(out2['method'], 'sui_executeTransactionBlock');

    await tx.inspectAll();
    final cost = await tx.inspectForCost();
    expect(cost['computation_cost'], 10);
    expect(cost['storage_cost'], 20);
    expect(cost['storage_rebate'], 5);
    expect(cost['total_cost'], 25);
  });

  test('split coin equal validation', () {
    final tx = Transaction();
    expect(() => tx.splitCoinEqual(tx.gas(), splitCount: 0, amountPerSplit: 1), throwsArgumentError);
  });

  test('stake and unstake baseline', () {
    final tx = Transaction();
    tx.stakeCoin(coins: ['0x11', '0x12'], validatorAddress: '0xvalidator', amount: 9);
    tx.unstakeCoin(stakedCoin: '0xstaked');

    expect(tx.data.commands[0]['\$kind'], 'MakeMoveVec');
    expect(tx.data.commands[1]['\$kind'], 'MoveCall');
    expect(tx.data.commands[1]['MoveCall']['function'], 'request_add_stake');
    expect(tx.data.commands[2]['\$kind'], 'MoveCall');
    expect(tx.data.commands[2]['MoveCall']['function'], 'request_withdraw_stake');
  });
}
