typedef PaginatedFetcher = Future<Map<String, dynamic>> Function(String? cursor);

Stream<Map<String, dynamic>> paginate(
  PaginatedFetcher fetcher, {
  String? startCursor,
  int? maxItems,
}) async* {
  var emitted = 0;
  String? cursor = startCursor;

  while (true) {
    final page = await fetcher(cursor);
    final data = page['data'];
    final items = data is List ? data.cast<Map<String, dynamic>>() : const <Map<String, dynamic>>[];

    for (final item in items) {
      if (maxItems != null && emitted >= maxItems) {
        return;
      }
      yield item;
      emitted += 1;
    }

    final hasNextPage = page['hasNextPage'] == true;
    if (!hasNextPage) {
      return;
    }

    final nextCursor = page['nextCursor'];
    cursor = nextCursor is String ? nextCursor : null;
    if (cursor == null) {
      return;
    }
  }
}
