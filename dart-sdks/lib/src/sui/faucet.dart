import 'dart:convert';
import 'dart:io';

const Map<String, String> defaultFaucetHosts = {
  'testnet': 'https://faucet.testnet.sui.io/v2/gas',
  'devnet': 'https://faucet.devnet.sui.io/v2/gas',
};

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
      return await _transport.post(endpoint, payload, timeout: timeout);
    } catch (e) {
      if (e is FaucetRateLimitError || e.toString().contains('429')) {
        throw FaucetRateLimitError(e.toString());
      }
      rethrow;
    }
  }
}
