package com.suisdks.sui.utils

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue
import kotlin.test.assertFailsWith

class FormatAndSuinsTest {
    @Test
    fun formatHelpers() {
        assertEquals("0x000000...000abc", formatAddress("0xabc"))
        assertEquals("123456...7890", formatDigest("1234567890"))
        assertEquals("short", formatDigest("short"))
    }

    @Test
    fun suinsHelpers() {
        assertTrue(isValidSuiNSName("alice.sui"))
        assertFalse(isValidSuiNSName("Alice Sui"))
        assertEquals("@alice.sui", normalizeSuiNSName("alice.sui", "at"))
        assertEquals("alice.sui", normalizeSuiNSName("@alice.sui", "dot"))
        assertFailsWith<IllegalArgumentException> {
            normalizeSuiNSName("invalid name", "at")
        }
    }
}
