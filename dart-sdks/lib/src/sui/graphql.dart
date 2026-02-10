import 'package:gql/language.dart';
import 'package:graphql/client.dart' as gql;

abstract class GraphQlTransport {
  Future<Map<String, dynamic>> execute(
    String query, {
    Map<String, dynamic>? variables,
  });
}

class OfficialGraphQlTransport implements GraphQlTransport {
  OfficialGraphQlTransport({
    required String endpoint,
    Duration timeout = const Duration(seconds: 30),
  }) : _client = gql.GraphQLClient(
          link: gql.HttpLink(endpoint),
          cache: gql.GraphQLCache(store: gql.InMemoryStore()),
        ),
        _timeout = timeout;

  final gql.GraphQLClient _client;
  final Duration _timeout;

  @override
  Future<Map<String, dynamic>> execute(
    String query, {
    Map<String, dynamic>? variables,
  }) async {
    final result = await _client
        .query(
          gql.QueryOptions(
            document: parseString(query),
            variables: variables ?? const <String, dynamic>{},
            fetchPolicy: gql.FetchPolicy.networkOnly,
          ),
        )
        .timeout(_timeout);

    if (result.hasException) {
      throw StateError('graphql error: ${result.exception}');
    }

    return <String, dynamic>{
      'data': result.data ?? <String, dynamic>{},
      if (result.source != null) 'source': result.source.toString(),
    };
  }
}

class GraphQlClient {
  GraphQlClient({
    required this.endpoint,
    this.timeout = const Duration(seconds: 30),
    GraphQlTransport? transport,
  }) : _transport = transport ?? OfficialGraphQlTransport(endpoint: endpoint, timeout: timeout);

  final String endpoint;
  final Duration timeout;
  final GraphQlTransport _transport;

  Future<Map<String, dynamic>> execute(String query, {Map<String, dynamic>? variables}) {
    return _executeChecked(query, variables: variables);
  }

  Future<Map<String, dynamic>> _executeChecked(String query, {Map<String, dynamic>? variables}) async {
    final out = await _transport.execute(query, variables: variables);
    final errors = out['errors'];
    if (errors is List && errors.isNotEmpty) {
      throw StateError('graphql error: $errors');
    }
    return out;
  }
}
