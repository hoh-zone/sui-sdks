package com.suisdks.sui.batch

import kotlin.test.Test
import kotlin.test.assertEquals

class BatchTest {
    @Test
    fun mapSyncAndMapAsync() {
        val outSync = mapSync(listOf(1, 2, 3)) { it * 2 }
        assertEquals(listOf(2, 4, 6), outSync)

        val outAsync = mapAsync(listOf("a", "b", "c")) { it.uppercase() }.get()
        assertEquals(listOf("A", "B", "C"), outAsync)
    }
}
