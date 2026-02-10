package com.suisdks.sui.cryptography

data class SerializedSignature(
    val scheme: SignatureScheme,
    val signature: ByteArray,
    val publicKey: ByteArray,
)
