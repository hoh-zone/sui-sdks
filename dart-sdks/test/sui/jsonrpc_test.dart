import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockTransport implements JsonRpcTransport {
  _MockTransport(this.responses);

  final List<Map<String, dynamic>> responses;
  final List<Map<String, dynamic>> payloads = <Map<String, dynamic>>[];

  @override
  Future<Map<String, dynamic>> send(
    String endpoint,
    Map<String, dynamic> payload, {
    Duration timeout = const Duration(seconds: 30),
  }) async {
    payloads.add(payload);
    return responses.removeAt(0);
  }
}

void main() {
  test('fromNetwork picks endpoint', () {
    final client = JsonRpcClient.fromNetwork(network: 'testnet');
    expect(client.endpoint, contains('testnet'));
  });

  test('call success', () async {
    final transport = _MockTransport([
      {'jsonrpc': '2.0', 'result': {'ok': true}}
    ]);
    final client = JsonRpcClient(endpoint: 'https://example.invalid', transport: transport);

    final out = await client.call('sui_getLatestCheckpointSequenceNumber');
    expect(out.containsKey('result'), isTrue);
    expect(transport.payloads.first['method'], 'sui_getLatestCheckpointSequenceNumber');
  });

  test('call error throws', () async {
    final transport = _MockTransport([
      {'error': {'code': -1}}
    ]);
    final client = JsonRpcClient(endpoint: 'https://example.invalid', transport: transport);

    await expectLater(client.call('bad_method'), throwsStateError);
  });
}
