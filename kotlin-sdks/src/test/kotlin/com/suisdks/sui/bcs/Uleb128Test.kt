package com.suisdks.sui.bcs

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class Uleb128Test {
    @Test
    fun encodeDecodeRoundtrip() {
        val values = listOf(0L, 1L, 127L, 128L, 255L, 16_384L, MAX_ULEB128_VALUE)
        for (v in values) {
            val encoded = encodeUleb128(v)
            val (decoded, consumed) = decodeUleb128(encoded)
            assertEquals(v, decoded)
            assertEquals(encoded.size, consumed)
        }
    }

    @Test
    fun decodeRejectsNonCanonicalAndOverflow() {
        assertFailsWith<IllegalArgumentException> {
            decodeUleb128(byteArrayOf(0x80.toByte(), 0x00))
        }
        assertFailsWith<IllegalArgumentException> {
            decodeUleb128(byteArrayOf(0x80.toByte()))
        }
        assertFailsWith<IllegalArgumentException> {
            decodeUleb128(byteArrayOf(0xFF.toByte(), 0xFF.toByte(), 0xFF.toByte(), 0xFF.toByte(), 0x10))
        }
    }
}
