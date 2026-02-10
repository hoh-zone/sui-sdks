package com.suisdks.sui.cryptography

import com.google.crypto.tink.KeysetHandle
import com.google.crypto.tink.aead.AeadConfig
import com.google.crypto.tink.aead.AeadKeyTemplates
import com.google.crypto.tink.Aead

/**
 * AEAD encryption helpers backed by Google's official Tink crypto library.
 */
object Encryption {
    init {
        AeadConfig.register()
    }

    fun generateAes256GcmKeyset(): KeysetHandle =
        KeysetHandle.generateNew(AeadKeyTemplates.AES256_GCM)

    fun encrypt(
        plaintext: ByteArray,
        aad: ByteArray = byteArrayOf(),
        keysetHandle: KeysetHandle = generateAes256GcmKeyset(),
    ): EncryptedPayload {
        val aead = keysetHandle.getPrimitive(Aead::class.java)
        val ciphertext = aead.encrypt(plaintext, aad)
        return EncryptedPayload(ciphertext = ciphertext, keysetHandle = keysetHandle)
    }

    fun decrypt(
        ciphertext: ByteArray,
        aad: ByteArray = byteArrayOf(),
        keysetHandle: KeysetHandle,
    ): ByteArray {
        val aead = keysetHandle.getPrimitive(Aead::class.java)
        return aead.decrypt(ciphertext, aad)
    }
}

data class EncryptedPayload(
    val ciphertext: ByteArray,
    val keysetHandle: KeysetHandle,
)
