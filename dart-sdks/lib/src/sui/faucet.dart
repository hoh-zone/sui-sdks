import 'dart:convert';
import 'dart:io';

const Map<String, String> defaultFaucetHosts = {
  'testnet': 'https://faucet.testnet.sui.io',
  'devnet': 'https://faucet.devnet.sui.io',
  'localnet': 'http://127.0.0.1:9123',
};
const String defaultFaucetPathV2 = '/v2/gas';

String getFaucetHost({String network = 'testnet'}) {
  final host = defaultFaucetHosts[network];
  if (host == null) {
    throw ArgumentError.value(network, 'network', 'unsupported faucet network');
  }
  return host;
}

class FaucetRateLimitError extends StateError {
  FaucetRateLimitError(super.message);
}

abstract class FaucetTransport {
  Future<Map<String, dynamic>> post(
    String endpoint,
    Map<String, dynamic> payload, {
    Duration timeout = const Duration(seconds: 30),
  });
}

class HttpFaucetTransport implements FaucetTransport {
  @override
  Future<Map<String, dynamic>> post(
    String endpoint,
    Map<String, dynamic> payload, {
    Duration timeout = const Duration(seconds: 30),
  }) async {
    final client = HttpClient();
    try {
      final req = await client.postUrl(Uri.parse(endpoint)).timeout(timeout);
      req.headers.contentType = ContentType.json;
      req.add(utf8.encode(jsonEncode(payload)));
      final resp = await req.close().timeout(timeout);
      final body = await utf8.decoder.bind(resp).join();

      if (resp.statusCode == 429) {
        throw FaucetRateLimitError('HTTP Error 429');
      }
      final parsed = jsonDecode(body);
      if (parsed is! Map<String, dynamic>) {
        throw StateError('faucet response is not an object');
      }
      return parsed;
    } finally {
      client.close(force: true);
    }
  }
}

class FaucetClient {
  FaucetClient({
    required this.endpoint,
    this.timeout = const Duration(seconds: 30),
    FaucetTransport? transport,
  }) : _transport = transport ?? HttpFaucetTransport();

  final String endpoint;
  final Duration timeout;
  final FaucetTransport _transport;

  static FaucetClient fromNetwork({
    String network = 'testnet',
    Duration timeout = const Duration(seconds: 30),
    FaucetTransport? transport,
  }) {
    return FaucetClient(
      endpoint: getFaucetHost(network: network),
      timeout: timeout,
      transport: transport,
    );
  }

  Future<Map<String, dynamic>> requestSuiFromFaucetV2(
    String recipient, {
    int? fixedAmount,
  }) async {
    final payload = <String, dynamic>{
      'FixedAmountRequest': <String, dynamic>{
        'recipient': recipient,
        if (fixedAmount != null) 'amount': fixedAmount,
      },
    };

    try {
      final endpointUri = Uri.parse(endpoint);
      final target = endpointUri.path.endsWith(defaultFaucetPathV2)
          ? endpointUri.toString()
          : endpointUri.resolve(defaultFaucetPathV2).toString();
      return await _transport.post(target, payload, timeout: timeout);
    } catch (e) {
      if (e is FaucetRateLimitError || e.toString().contains('429')) {
        throw FaucetRateLimitError(e.toString());
      }
      rethrow;
    }
  }
}

Future<Map<String, dynamic>> requestSuiFromFaucetV2({
  required String host,
  required String recipient,
  int? fixedAmount,
  Duration timeout = const Duration(seconds: 30),
  FaucetTransport? transport,
}) {
  final client = FaucetClient(
    endpoint: host,
    timeout: timeout,
    transport: transport,
  );
  return client.requestSuiFromFaucetV2(
    recipient,
    fixedAmount: fixedAmount,
  );
}
