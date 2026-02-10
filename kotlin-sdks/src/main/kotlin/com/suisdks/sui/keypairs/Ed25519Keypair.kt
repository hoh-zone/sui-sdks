package com.suisdks.sui.keypairs

import com.suisdks.sui.cryptography.BouncyCastleProviderHolder
import com.suisdks.sui.cryptography.SignatureScheme
import java.security.KeyFactory
import java.security.KeyPairGenerator
import java.security.PrivateKey
import java.security.PublicKey
import java.security.Signature
import java.security.spec.X509EncodedKeySpec

class Ed25519Keypair private constructor(
    private val privateKey: PrivateKey,
    private val publicKey: PublicKey,
) : Keypair {
    companion object {
        private val ED25519_SPKI_PREFIX = byteArrayOf(
            0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
        )

        fun generate(): Ed25519Keypair {
            return try {
                BouncyCastleProviderHolder.ensureInstalled()
                val generator = KeyPairGenerator.getInstance("Ed25519", "BC")
                val pair = generator.generateKeyPair()
                Ed25519Keypair(pair.private, pair.public)
            } catch (e: Exception) {
                throw IllegalStateException("Failed to generate Ed25519 keypair", e)
            }
        }

        fun publicKeyFromRaw(raw32: ByteArray): PublicKey {
            require(raw32.size == 32) { "Ed25519 public key must be 32 bytes" }
            val full = ED25519_SPKI_PREFIX + raw32
            return try {
                BouncyCastleProviderHolder.ensureInstalled()
                KeyFactory.getInstance("Ed25519", "BC").generatePublic(X509EncodedKeySpec(full))
            } catch (e: Exception) {
                throw IllegalArgumentException("Invalid Ed25519 public key bytes", e)
            }
        }

        fun verifyWithPublicKey(publicKey: ByteArray, message: ByteArray, signature: ByteArray): Boolean {
            return try {
                BouncyCastleProviderHolder.ensureInstalled()
                val verifier = Signature.getInstance("Ed25519", "BC")
                verifier.initVerify(publicKeyFromRaw(publicKey))
                verifier.update(message)
                verifier.verify(signature)
            } catch (_: Exception) {
                false
            }
        }
    }

    override fun scheme(): SignatureScheme = SignatureScheme.ED25519

    override fun publicKeyBytes(): ByteArray {
        val x509 = publicKey.encoded
        return x509.sliceArray(x509.size - 32 until x509.size)
    }

    override fun sign(message: ByteArray): ByteArray {
        return try {
            BouncyCastleProviderHolder.ensureInstalled()
            val signer = Signature.getInstance("Ed25519", "BC")
            signer.initSign(privateKey)
            signer.update(message)
            signer.sign()
        } catch (e: Exception) {
            throw IllegalStateException("Failed to sign with Ed25519", e)
        }
    }

    override fun verify(message: ByteArray, signature: ByteArray): Boolean {
        return verifyWithPublicKey(publicKeyBytes(), message, signature)
    }
}
