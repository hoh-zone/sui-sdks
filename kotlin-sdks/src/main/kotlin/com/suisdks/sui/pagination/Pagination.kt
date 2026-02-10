package com.suisdks.sui.pagination

fun iterPaginatedItems(
    fetchPage: (String?) -> Map<String, Any?>,
    startCursor: String? = null,
    maxItems: Int? = null,
): Sequence<Map<String, Any?>> = sequence {
    var cursor = startCursor
    var emitted = 0

    while (true) {
        val page = fetchPage(cursor)
        @Suppress("UNCHECKED_CAST")
        val data = page["data"] as? List<Map<String, Any?>> ?: emptyList()

        for (item in data) {
            yield(item)
            emitted += 1
            if (maxItems != null && emitted >= maxItems) {
                return@sequence
            }
        }

        val hasNextPage = page["hasNextPage"] as? Boolean ?: false
        if (!hasNextPage) {
            return@sequence
        }
        val nextCursor = page["nextCursor"]?.toString()
        if (nextCursor == null || nextCursor == cursor) {
            return@sequence
        }
        cursor = nextCursor
    }
}
