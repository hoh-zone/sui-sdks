package com.suisdks.sui.utils

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class SuiTypesTest {
    @Test
    fun normalizeAndValidateAddress() {
        val normalized = normalizeSuiAddress("0xabc", forceAdd0x = true)
        assertTrue(normalized.startsWith("0x"))
        assertEquals(66, normalized.length)
        assertTrue(isValidSuiAddress("0xabc"))
        assertTrue(isValidSuiObjectId("abc"))
        assertFalse(isValidSuiAddress("0xzz"))
    }

    @Test
    fun transactionDigestValidation() {
        assertTrue(isValidTransactionDigest("3vQB7B6MrGQZaxCuFg4oh"))
        assertFalse(isValidTransactionDigest("0x123"))
        assertFalse(isValidTransactionDigest(""))
    }
}
