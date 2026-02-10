package com.suisdks.sui.keypairs

import com.suisdks.sui.cryptography.SignatureScheme

interface Keypair {
    fun scheme(): SignatureScheme
    fun publicKeyBytes(): ByteArray
    fun sign(message: ByteArray): ByteArray
    fun verify(message: ByteArray, signature: ByteArray): Boolean
}
