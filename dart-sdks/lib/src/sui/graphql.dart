import 'package:gql/language.dart';
import 'package:graphql/client.dart' as gql;
import 'pagination.dart';

const String defaultGraphQlCoinType = '0x2::coin::Coin<0x2::sui::SUI>';

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

  Future<Map<String, dynamic>> query(String query, {Map<String, dynamic>? variables}) {
    return execute(query, variables: variables);
  }

  Future<Map<String, dynamic>> mutation(String query, {Map<String, dynamic>? variables}) {
    return execute(query, variables: variables);
  }

  Future<Map<String, dynamic>> getBalance({
    required String owner,
    String coinType = '0x2::sui::SUI',
  }) {
    return query(
      r'''
query getBalance($owner: SuiAddress!, $coinType: String = "0x2::sui::SUI") {
  address(address: $owner) {
    balance(coinType: $coinType) {
      coinType { repr }
      totalBalance
      addressBalance
    }
  }
}
''',
      variables: <String, dynamic>{'owner': owner, 'coinType': coinType},
    );
  }

  Future<Map<String, dynamic>> getCoins({
    required String owner,
    String coinType = defaultGraphQlCoinType,
    String? cursor,
    int? limit,
  }) {
    return query(
      r'''
query getCoins($owner: SuiAddress!, $first: Int, $cursor: String, $type: String = "0x2::coin::Coin<0x2::sui::SUI>") {
  address(address: $owner) {
    address
    objects(first: $first, after: $cursor, filter: { type: $type }) {
      pageInfo { hasNextPage endCursor }
      nodes {
        address
        version
        digest
        owner { __typename }
        contents { json type { repr } }
      }
    }
  }
}
''',
      variables: <String, dynamic>{
        'owner': owner,
        'first': limit,
        'cursor': cursor,
        'type': coinType,
      },
    );
  }

  Stream<Map<String, dynamic>> iterCoins({
    required String owner,
    String coinType = defaultGraphQlCoinType,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async => _toPage(
        await getCoins(owner: owner, coinType: coinType, cursor: c, limit: limit),
        const ['address', 'objects'],
      ),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getAllBalances({
    required String owner,
    String? cursor,
    int? limit,
  }) {
    return query(
      r'''
query getAllBalances($owner: SuiAddress!, $limit: Int, $cursor: String) {
  address(address: $owner) {
    balances(first: $limit, after: $cursor) {
      pageInfo { hasNextPage endCursor }
      nodes {
        coinType { repr }
        totalBalance
        addressBalance
      }
    }
  }
}
''',
      variables: <String, dynamic>{'owner': owner, 'limit': limit, 'cursor': cursor},
    );
  }

  Stream<Map<String, dynamic>> iterAllBalances({
    required String owner,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async => _toPage(
        await getAllBalances(owner: owner, cursor: c, limit: limit),
        const ['address', 'balances'],
      ),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getCoinMetadata({required String coinType}) {
    return query(
      r'''
query getCoinMetadata($coinType: String!) {
  coinMetadata(coinType: $coinType) {
    address
    decimals
    name
    symbol
    description
    iconUrl
  }
}
''',
      variables: <String, dynamic>{'coinType': coinType},
    );
  }

  Future<Map<String, dynamic>> getTotalSupply({required String coinType}) {
    return query(
      r'''
query getTotalSupply($coinType: String!) {
  coinMetadata(coinType: $coinType) {
    supply
    supplyState
  }
}
''',
      variables: <String, dynamic>{'coinType': coinType},
    );
  }

  Future<Map<String, dynamic>> getDefaultSuinsName({required String address}) {
    return query(
      r'''
query defaultSuinsName($address: SuiAddress!) {
  address(address: $address) {
    defaultNameRecord { domain }
  }
}
''',
      variables: <String, dynamic>{'address': address},
    );
  }

  Future<Map<String, dynamic>> resolveNameServiceAddress({required String name}) {
    return query(
      r'''
query resolveNameServiceAddress($name: String!) {
  nameRecord(name: $name) {
    domain
    target {
      address
    }
  }
}
''',
      variables: <String, dynamic>{'name': name},
    );
  }

  Future<Map<String, dynamic>> resolveNameServiceNames({
    required String address,
    String? cursor,
    int? limit,
  }) {
    final _ = (cursor, limit);
    return query(
      r'''
query resolveNameServiceNames($address: SuiAddress!) {
  address(address: $address) {
    defaultNameRecord {
      domain
    }
  }
}
''',
      variables: <String, dynamic>{'address': address},
    );
  }

  Future<Map<String, dynamic>> getObject({
    required String objectId,
  }) {
    return query(
      r'''
query getObject($objectId: SuiAddress!) {
  object(address: $objectId) {
    address
    digest
    version
    owner { __typename }
  }
}
''',
      variables: <String, dynamic>{'objectId': objectId},
    );
  }

  Future<Map<String, dynamic>> multiGetObjects({
    required List<String> objectIds,
  }) {
    return query(
      r'''
query multiGetObjects($objectKeys: [ObjectKey!]!) {
  multiGetObjects(keys: $objectKeys) {
    address
    digest
    version
  }
}
''',
      variables: <String, dynamic>{
        'objectKeys': objectIds.map((id) => <String, dynamic>{'address': id}).toList(),
      },
    );
  }

  Future<Map<String, dynamic>> getEventsByTransaction({
    required String digest,
    int limit = 50,
  }) {
    return query(
      r'''
query getEventsByTransaction($digest: String!, $first: Int = 50) {
  transaction(digest: $digest) {
    effects {
      events(first: $first) {
        pageInfo { hasNextPage endCursor }
        nodes {
          sender { address }
          transactionModule { name package { address } }
          contents { bcs type { repr } }
        }
      }
    }
  }
}
''',
      variables: <String, dynamic>{'digest': digest, 'first': limit},
    );
  }

  Future<Map<String, dynamic>> getMoveFunction({
    required String packageId,
    required String moduleName,
    required String functionName,
  }) {
    return query(
      r'''
query getMoveFunction($package: SuiAddress!, $module: String!, $function: String!) {
  package(address: $package) {
    module(name: $module) {
      function(name: $function) {
        name
        visibility
        isEntry
        typeParameters { constraints }
        parameters { signature }
        return { signature }
      }
    }
  }
}
''',
      variables: <String, dynamic>{
        'package': packageId,
        'module': moduleName,
        'function': functionName,
      },
    );
  }

  Future<Map<String, dynamic>> queryTransactionBlocks({
    Map<String, dynamic>? filter,
    String? cursor,
    int? limit,
  }) {
    return query(
      r'''
query queryTransactionBlocks($first: Int, $cursor: String, $filter: TransactionFilter) {
  transactions(first: $first, after: $cursor, filter: $filter) {
    pageInfo { hasNextPage endCursor }
    nodes {
      digest
      signatures { signatureBytes }
    }
  }
}
''',
      variables: <String, dynamic>{'first': limit, 'cursor': cursor, 'filter': filter},
    );
  }

  Stream<Map<String, dynamic>> iterTransactionBlocks({
    Map<String, dynamic>? filter,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async => _toPage(
        await queryTransactionBlocks(filter: filter, cursor: c, limit: limit),
        const ['transactions'],
      ),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> queryEvents({
    Map<String, dynamic>? filter,
    String? cursor,
    int? limit,
  }) {
    return query(
      r'''
query queryEvents($first: Int, $cursor: String, $filter: EventFilter) {
  events(first: $first, after: $cursor, filter: $filter) {
    pageInfo { hasNextPage endCursor }
    nodes {
      sender { address }
      transactionModule { name package { address } }
      contents { bcs type { repr } }
    }
  }
}
''',
      variables: <String, dynamic>{'first': limit, 'cursor': cursor, 'filter': filter},
    );
  }

  Stream<Map<String, dynamic>> iterEvents({
    Map<String, dynamic>? filter,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async => _toPage(
        await queryEvents(filter: filter, cursor: c, limit: limit),
        const ['events'],
      ),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> simulateTransaction({
    required Map<String, dynamic> transaction,
    bool checksEnabled = true,
    bool doGasSelection = false,
    bool includeCommandResults = false,
  }) {
    return query(
      r'''
query simulateTransaction(
  $transaction: JSON!
  $doGasSelection: Boolean = false
  $checksEnabled: Boolean = true
  $includeCommandResults: Boolean = false
) {
  simulateTransaction(transaction: $transaction, doGasSelection: $doGasSelection, checksEnabled: $checksEnabled) {
    error
    effects { transaction { digest } }
    outputs @include(if: $includeCommandResults) {
      returnValues { value { bcs } }
      mutatedReferences { value { bcs } }
    }
  }
}
''',
      variables: <String, dynamic>{
        'transaction': transaction,
        'doGasSelection': doGasSelection,
        'checksEnabled': checksEnabled,
        'includeCommandResults': includeCommandResults,
      },
    );
  }

  Future<Map<String, dynamic>> resolveTransaction({
    required Map<String, dynamic> transaction,
    bool doGasSelection = true,
  }) {
    return query(
      r'''
query resolveTransaction($transaction: JSON!, $doGasSelection: Boolean = true) {
  simulateTransaction(transaction: $transaction, doGasSelection: $doGasSelection) {
    error
    effects { transaction { transactionBcs } }
  }
}
''',
      variables: <String, dynamic>{'transaction': transaction, 'doGasSelection': doGasSelection},
    );
  }

  Future<Map<String, dynamic>> getOwnedObjects({
    required String owner,
    Map<String, dynamic>? filter,
    String? cursor,
    int? limit,
  }) {
    return query(
      r'''
query getOwnedObjects($owner: SuiAddress!, $limit: Int, $cursor: String, $filter: ObjectFilter) {
  address(address: $owner) {
    objects(first: $limit, after: $cursor, filter: $filter) {
      pageInfo { hasNextPage endCursor }
      nodes { address digest version }
    }
  }
}
''',
      variables: <String, dynamic>{
        'owner': owner,
        'limit': limit,
        'cursor': cursor,
        'filter': filter,
      },
    );
  }

  Stream<Map<String, dynamic>> iterOwnedObjects({
    required String owner,
    Map<String, dynamic>? filter,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async => _toPage(
        await getOwnedObjects(owner: owner, filter: filter, cursor: c, limit: limit),
        const ['address', 'objects'],
      ),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getDynamicFields({
    required String parentId,
    String? cursor,
    int? limit,
  }) {
    return query(
      r'''
query getDynamicFields($parentId: SuiAddress!, $first: Int, $cursor: String) {
  address(address: $parentId) {
    dynamicFields(first: $first, after: $cursor) {
      pageInfo { hasNextPage endCursor }
      nodes {
        name { bcs type { repr } }
        value { __typename }
      }
    }
  }
}
''',
      variables: <String, dynamic>{'parentId': parentId, 'first': limit, 'cursor': cursor},
    );
  }

  Stream<Map<String, dynamic>> iterDynamicFields({
    required String parentId,
    String? cursor,
    int limit = 100,
    int? maxItems,
  }) {
    return paginate(
      (c) async => _toPage(
        await getDynamicFields(parentId: parentId, cursor: c, limit: limit),
        const ['address', 'dynamicFields'],
      ),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getTransactionBlock({required String digest}) {
    return query(
      r'''
query getTransactionBlock($digest: String!) {
  transaction(digest: $digest) {
    digest
    signatures { signatureBytes }
  }
}
''',
      variables: <String, dynamic>{'digest': digest},
    );
  }

  Future<Map<String, dynamic>> executeTransactionBlock({
    required String transactionDataBcs,
    required List<String> signatures,
  }) {
    return mutation(
      r'''
mutation executeTransaction($transactionDataBcs: Base64!, $signatures: [Base64!]!) {
  executeTransaction(transactionDataBcs: $transactionDataBcs, signatures: $signatures) {
    errors
    effects { transaction { digest } }
  }
}
''',
      variables: <String, dynamic>{
        'transactionDataBcs': transactionDataBcs,
        'signatures': signatures,
      },
    );
  }

  Future<Map<String, dynamic>> getChainIdentifier() {
    return query(
      r'''
query getChainIdentifier {
  checkpoint(sequenceNumber: 0) { digest }
}
''',
    );
  }

  Future<Map<String, dynamic>> getCheckpoint({int? sequenceNumber}) {
    return query(
      r'''
query getCheckpoint($sequenceNumber: UInt53) {
  checkpoint(sequenceNumber: $sequenceNumber) {
    sequenceNumber
    digest
    timestamp
  }
}
''',
      variables: <String, dynamic>{'sequenceNumber': sequenceNumber},
    );
  }

  Future<Map<String, dynamic>> getCheckpoints({
    String? cursor,
    int? limit,
    Map<String, dynamic>? filter,
  }) {
    return query(
      r'''
query getCheckpoints($first: Int, $cursor: String, $filter: CheckpointFilter) {
  checkpoints(first: $first, after: $cursor, filter: $filter) {
    pageInfo { hasNextPage endCursor }
    nodes {
      sequenceNumber
      digest
      timestamp
    }
  }
}
''',
      variables: <String, dynamic>{'first': limit, 'cursor': cursor, 'filter': filter},
    );
  }

  Stream<Map<String, dynamic>> iterCheckpoints({
    String? cursor,
    int limit = 100,
    Map<String, dynamic>? filter,
    int? maxItems,
  }) {
    return paginate(
      (c) async => _toPage(
        await getCheckpoints(cursor: c, limit: limit, filter: filter),
        const ['checkpoints'],
      ),
      startCursor: cursor,
      maxItems: maxItems,
    );
  }

  Future<Map<String, dynamic>> getLatestCheckpointSequenceNumber() {
    return query(
      r'''
query getLatestCheckpointSequenceNumber {
  checkpoint {
    sequenceNumber
  }
}
''',
    );
  }

  Future<Map<String, dynamic>> getReferenceGasPrice() {
    return query(
      r'''
query getReferenceGasPrice {
  epoch { referenceGasPrice }
}
''',
    );
  }

  Future<Map<String, dynamic>> getProtocolConfig({int? version}) {
    return query(
      r'''
query getProtocolConfig($version: UInt53) {
  protocolConfigs(version: $version) {
    protocolVersion
    configs { key value }
    featureFlags { key value }
  }
}
''',
      variables: <String, dynamic>{'version': version},
    );
  }

  Future<Map<String, dynamic>> getCurrentSystemState() {
    return query(
      r'''
query getCurrentSystemState {
  epoch {
    epochId
    referenceGasPrice
    startTimestamp
    protocolConfigs { protocolVersion }
    systemState { json }
  }
}
''',
    );
  }

  Future<Map<String, dynamic>> getLatestSuiSystemState() {
    return getCurrentSystemState();
  }

  Future<Map<String, dynamic>> _executeChecked(String query, {Map<String, dynamic>? variables}) async {
    final out = await _transport.execute(query, variables: variables);
    final errors = out['errors'];
    if (errors is List && errors.isNotEmpty) {
      throw StateError('graphql error: $errors');
    }
    return out;
  }

  Map<String, dynamic> _toPage(Map<String, dynamic> out, List<String> path) {
    dynamic cur = out['data'];
    for (final key in path) {
      if (cur is Map<String, dynamic>) {
        cur = cur[key];
      } else {
        cur = null;
        break;
      }
    }

    final conn = cur is Map<String, dynamic> ? cur : const <String, dynamic>{};
    final nodes = conn['nodes'];
    final pageInfo = conn['pageInfo'];
    final hasNext = pageInfo is Map<String, dynamic> && pageInfo['hasNextPage'] == true;
    final endCursor = pageInfo is Map<String, dynamic> ? pageInfo['endCursor'] : null;

    return <String, dynamic>{
      'data': nodes is List ? nodes.cast<Map<String, dynamic>>() : const <Map<String, dynamic>>[],
      'hasNextPage': hasNext,
      'nextCursor': endCursor,
    };
  }
}
