package com.suisdks.sui.cryptography

enum class SignatureScheme(val flag: Int, val publicKeySize: Int) {
    ED25519(0, 32),
    SECP256K1(1, 33),
    SECP256R1(2, 33);

    companion object {
        fun fromFlag(flag: Int): SignatureScheme = entries.firstOrNull { it.flag == flag }
            ?: throw IllegalArgumentException("Unsupported signature scheme flag: $flag")
    }
}
