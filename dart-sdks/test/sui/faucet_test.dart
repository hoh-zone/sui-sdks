import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockFaucetTransport implements FaucetTransport {
  _MockFaucetTransport(this.handler);

  final Future<Map<String, dynamic>> Function(String endpoint, Map<String, dynamic> payload) handler;

  @override
  Future<Map<String, dynamic>> post(
    String endpoint,
    Map<String, dynamic> payload, {
    Duration timeout = const Duration(seconds: 30),
  }) {
    return handler(endpoint, payload);
  }
}

void main() {
  test('get host', () {
    expect(getFaucetHost(network: 'testnet'), contains('testnet'));
    expect(getFaucetHost(network: 'localnet'), 'http://127.0.0.1:9123');
    expect(() => getFaucetHost(network: 'mainnet'), throwsArgumentError);
  });

  test('request success', () async {
    final client = FaucetClient.fromNetwork(
      network: 'testnet',
      transport: _MockFaucetTransport((_, __) async => {'transferredGasObjects': <dynamic>[]}),
    );

    final out = await client.requestSuiFromFaucetV2('0x1');
    expect(out.containsKey('transferredGasObjects'), isTrue);
  });

  test('request targets v2 gas path', () async {
    String? endpoint;
    final client = FaucetClient.fromNetwork(
      network: 'testnet',
      transport: _MockFaucetTransport((e, __) async {
        endpoint = e;
        return {'ok': true};
      }),
    );

    await client.requestSuiFromFaucetV2('0x1');
    expect(endpoint, 'https://faucet.testnet.sui.io/v2/gas');
  });

  test('top-level request helper', () async {
    String? endpoint;
    final out = await requestSuiFromFaucetV2(
      host: 'http://127.0.0.1:9123',
      recipient: '0x1',
      transport: _MockFaucetTransport((e, payload) async {
        endpoint = e;
        expect(payload['FixedAmountRequest']['recipient'], '0x1');
        return {'ok': true};
      }),
    );

    expect(endpoint, 'http://127.0.0.1:9123/v2/gas');
    expect(out['ok'], isTrue);
  });

  test('request 429 mapped', () async {
    final client = FaucetClient.fromNetwork(
      network: 'testnet',
      transport: _MockFaucetTransport((_, __) async => throw Exception('HTTP Error 429')),
    );

    await expectLater(client.requestSuiFromFaucetV2('0x1'), throwsA(isA<FaucetRateLimitError>()));
  });
}
