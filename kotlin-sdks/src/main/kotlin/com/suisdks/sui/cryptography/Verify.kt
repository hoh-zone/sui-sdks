package com.suisdks.sui.cryptography

import com.suisdks.sui.keypairs.Ed25519Keypair
import com.suisdks.sui.keypairs.Secp256k1Keypair
import com.suisdks.sui.keypairs.Secp256r1Keypair
import java.util.Base64

object Verify {
    fun verifyRawSignature(
        message: ByteArray,
        signature: ByteArray,
        publicKey: ByteArray,
        scheme: SignatureScheme,
    ): Boolean = when (scheme) {
        SignatureScheme.ED25519 -> Ed25519Keypair.verifyWithPublicKey(publicKey, message, signature)
        SignatureScheme.SECP256K1 -> Secp256k1Keypair.verifyWithPublicKey(publicKey, message, signature)
        SignatureScheme.SECP256R1 -> Secp256r1Keypair.verifyWithPublicKey(publicKey, message, signature)
    }

    fun verifyPersonalMessage(
        message: ByteArray,
        signature: ByteArray,
        publicKey: ByteArray,
        scheme: SignatureScheme,
    ): Boolean = verifyRawSignature(personalMessagePayload(message), signature, publicKey, scheme)

    fun personalMessagePayload(message: ByteArray): ByteArray {
        val prefix = "\u0019Sui Signed Message:\n".toByteArray(Charsets.UTF_8)
        val len = message.size.toString().toByteArray(Charsets.UTF_8)
        val newline = "\n".toByteArray(Charsets.UTF_8)
        return prefix + len + newline + message
    }

    fun toSerializedSignature(scheme: SignatureScheme, signature: ByteArray, publicKey: ByteArray): String {
        require(publicKey.size == scheme.publicKeySize) {
            "Public key length mismatch for scheme $scheme"
        }
        val out = ByteArray(1 + signature.size + publicKey.size)
        out[0] = scheme.flag.toByte()
        signature.copyInto(out, destinationOffset = 1)
        publicKey.copyInto(out, destinationOffset = 1 + signature.size)
        return Base64.getEncoder().encodeToString(out)
    }

    fun parseSerializedSignature(serialized: String): SerializedSignature {
        val raw = Base64.getDecoder().decode(serialized)
        require(raw.size >= 2) { "Serialized signature too short" }

        val scheme = SignatureScheme.fromFlag(raw[0].toInt() and 0xff)
        val pkLen = scheme.publicKeySize
        require(raw.size > 1 + pkLen) { "Serialized signature too short for scheme" }

        val signature = raw.sliceArray(1 until raw.size - pkLen)
        val publicKey = raw.sliceArray(raw.size - pkLen until raw.size)
        return SerializedSignature(scheme = scheme, signature = signature, publicKey = publicKey)
    }
}
