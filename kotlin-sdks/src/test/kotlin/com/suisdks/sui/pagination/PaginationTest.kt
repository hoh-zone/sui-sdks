package com.suisdks.sui.pagination

import kotlin.test.Test
import kotlin.test.assertEquals

class PaginationTest {
    @Test
    fun iteratesAcrossPagesAndRespectsMaxItems() {
        val pages = mapOf(
            null to mapOf(
                "data" to listOf(mapOf("id" to 1), mapOf("id" to 2)),
                "hasNextPage" to true,
                "nextCursor" to "c1",
            ),
            "c1" to mapOf(
                "data" to listOf(mapOf("id" to 3), mapOf("id" to 4)),
                "hasNextPage" to false,
                "nextCursor" to null,
            ),
        )

        val out = iterPaginatedItems(fetchPage = { c -> pages[c] ?: emptyMap() }, maxItems = 3).toList()
        assertEquals(listOf(1, 2, 3), out.map { it["id"] })
    }
}
