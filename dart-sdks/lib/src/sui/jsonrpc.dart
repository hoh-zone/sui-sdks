import 'dart:convert';
import 'dart:io';

const Map<String, String> defaultFullnodeEndpoints = {
  'mainnet': 'https://fullnode.mainnet.sui.io:443',
  'testnet': 'https://fullnode.testnet.sui.io:443',
  'devnet': 'https://fullnode.devnet.sui.io:443',
};

abstract class JsonRpcTransport {
  Future<Map<String, dynamic>> send(
    String endpoint,
    Map<String, dynamic> payload, {
    Duration timeout = const Duration(seconds: 30),
  });
}

class HttpJsonRpcTransport implements JsonRpcTransport {
  @override
  Future<Map<String, dynamic>> send(
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
      final parsed = jsonDecode(body);
      if (parsed is! Map<String, dynamic>) {
        throw StateError('jsonrpc response is not an object');
      }
      return parsed;
    } finally {
      client.close(force: true);
    }
  }
}

class JsonRpcClient {
  JsonRpcClient({
    required this.endpoint,
    this.timeout = const Duration(seconds: 30),
    JsonRpcTransport? transport,
  }) : _transport = transport ?? HttpJsonRpcTransport();

  final String endpoint;
  final Duration timeout;
  final JsonRpcTransport _transport;

  static JsonRpcClient fromNetwork({
    String network = 'testnet',
    Duration timeout = const Duration(seconds: 30),
    JsonRpcTransport? transport,
  }) {
    final endpoint = defaultFullnodeEndpoints[network];
    if (endpoint == null) {
      throw ArgumentError.value(network, 'network', 'unsupported network');
    }
    return JsonRpcClient(endpoint: endpoint, timeout: timeout, transport: transport);
  }

  Future<Map<String, dynamic>> call(String method, [List<dynamic>? params]) async {
    final payload = <String, dynamic>{
      'jsonrpc': '2.0',
      'id': 1,
      'method': method,
      'params': params ?? <dynamic>[],
    };

    final parsed = await _transport.send(endpoint, payload, timeout: timeout);
    if (parsed.containsKey('error')) {
      throw StateError('jsonrpc error: ${parsed['error']}');
    }
    return parsed;
  }
}
