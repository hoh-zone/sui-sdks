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

  test('request 429 mapped', () async {
    final client = FaucetClient.fromNetwork(
      network: 'testnet',
      transport: _MockFaucetTransport((_, __) async => throw Exception('HTTP Error 429')),
    );

    await expectLater(client.requestSuiFromFaucetV2('0x1'), throwsA(isA<FaucetRateLimitError>()));
  });
}
