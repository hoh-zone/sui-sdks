package com.suisdks.sui.cryptography

import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertFailsWith

class EncryptionTest {
    @Test
    fun encryptDecryptRoundtrip() {
        val message = "secret-data".toByteArray()
        val aad = "ctx".toByteArray()

        val payload = Encryption.encrypt(message, aad)
        val decrypted = Encryption.decrypt(payload.ciphertext, aad, payload.keysetHandle)

        assertContentEquals(message, decrypted)
    }

    @Test
    fun decryptWithWrongAadFails() {
        val message = "secret-data".toByteArray()
        val payload = Encryption.encrypt(message, "a".toByteArray())

        assertFailsWith<Exception> {
            Encryption.decrypt(payload.ciphertext, "b".toByteArray(), payload.keysetHandle)
        }
    }
}
