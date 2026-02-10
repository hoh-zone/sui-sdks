import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockGraphQlTransport implements GraphQlTransport {
  _MockGraphQlTransport(this.responses);

  final List<Map<String, dynamic>> responses;

  @override
  Future<Map<String, dynamic>> execute(
    String query, {
    Map<String, dynamic>? variables,
  }) async {
    return responses.removeAt(0);
  }
}

void main() {
  test('execute success', () async {
    final client = GraphQlClient(
      endpoint: 'https://example.invalid',
      transport: _MockGraphQlTransport([
        {'data': {'ok': true}}
      ]),
    );

    final out = await client.execute('query { ok }');
    expect(out.containsKey('data'), isTrue);
  });

  test('execute error surface', () async {
    final transport = _MockGraphQlTransport([
      {'errors': ['bad']}
    ]);

    final client = GraphQlClient(endpoint: 'https://example.invalid', transport: transport);
    await expectLater(client.execute('query { bad }'), throwsStateError);
  });
}
