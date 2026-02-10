import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockDryRunClient {
  @override
  Future<Map<String, dynamic>> call(String method, [List<dynamic>? params]) async {
    final one = base64Encode(encodeU64(100));
    final two = base64Encode(encodeU64(200));
    final three = base64Encode(encodeU64(300));
    return {
      'commandResults': [
        {
          'returnValues': [
            {'bcs': one},
            {'bcs': two},
            {'bcs': three},
          ]
        },
        {
          'returnValues': [
            {'bcs': one}
          ]
        }
      ]
    };
  }
}

class _MockBadClient {
  @override
  Future<Map<String, dynamic>> call(String method, [List<dynamic>? params]) async {
    return <String, dynamic>{};
  }
}

void main() {
  DeepBookClient buildClient() {
    final cfg = DeepBookConfig(
      address: '0x1',
      balanceManagers: const {'m1': BalanceManager(address: '0x2')},
      marginManagers: const {'mm1': MarginManager(address: '0x3', poolKey: 'DEEP_SUI')},
    );
    return DeepBookClient(client: _MockDryRunClient(), config: cfg);
  }

  test('quantity methods', () async {
    final c = buildClient();
    final out = await c.getQuoteQuantityOut('DEEP_SUI', 1);
    final out2 = await c.getBaseQuantityOut('DEEP_SUI', 1);
    final out3 = await c.getQuantityOut('DEEP_SUI', 1, 0);
    expect(out.containsKey('deepRequired'), isTrue);
    expect(out2.containsKey('deepRequired'), isTrue);
    expect(out3.containsKey('deepRequired'), isTrue);
  });

  test('mid price and order paths', () async {
    final c = buildClient();
    expect(await c.midPrice('DEEP_SUI'), greaterThan(0));
    expect(await c.getOrder('DEEP_SUI', '1'), isNotEmpty);
    expect(await c.getMarginAccountOrderDetails('mm1'), isNotEmpty);
  });

  test('check manager balance', () async {
    final c = buildClient();
    final out = await c.checkManagerBalance('m1', 'SUI');
    expect(out.containsKey('coinType'), isTrue);
  });

  test('error path', () async {
    final cfg = DeepBookConfig(
      address: '0x1',
      balanceManagers: const {'m1': BalanceManager(address: '0x2')},
    );
    final c = DeepBookClient(client: _MockBadClient(), config: cfg);
    await expectLater(c.checkManagerBalance('m1', 'SUI'), throwsStateError);
  });
}
