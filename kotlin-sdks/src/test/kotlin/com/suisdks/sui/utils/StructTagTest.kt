package com.suisdks.sui.utils

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class StructTagTest {
    @Test
    fun parseAndNormalizeStructTag() {
        val parsed = parseStructTag("0x2::coin::Coin<0x2::sui::SUI>")
        assertEquals("0x0000000000000000000000000000000000000000000000000000000000000002", parsed.address)
        assertEquals("coin", parsed.module)
        assertEquals("Coin", parsed.name)
        assertEquals(1, parsed.typeParams.size)

        val normalized = normalizeStructTag("0x2::coin::Coin<0x2::sui::SUI>")
        assertEquals(
            "0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>",
            normalized,
        )
    }

    @Test
    fun normalizeNestedGenericParams() {
        val normalized = normalizeStructTag(
            "0x2::vec::Wrapper<0x2::coin::Coin<0x2::sui::SUI>, vector<u8>>",
        )
        assertTrue(normalized.contains("::vec::Wrapper<"))
        assertTrue(normalized.contains("::coin::Coin<"))
        assertTrue(normalized.contains("vector<u8>"))
    }
}
