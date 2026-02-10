package com.suisdks.sui.keypairs

import com.suisdks.sui.cryptography.BouncyCastleProviderHolder
import com.suisdks.sui.cryptography.SignatureScheme
import java.security.AlgorithmParameters
import java.security.KeyFactory
import java.security.KeyPairGenerator
import java.security.PrivateKey
import java.security.PublicKey
import java.security.Signature
import java.security.spec.ECGenParameterSpec
import java.security.spec.ECParameterSpec
import java.security.spec.ECPublicKeySpec
import org.bouncycastle.jce.ECNamedCurveTable
import org.bouncycastle.jce.interfaces.ECPublicKey as BCECPublicKey

class Secp256k1Keypair private constructor(
    private val privateKey: PrivateKey,
    private val publicKey: PublicKey,
) : Keypair {
    companion object {
        fun generate(): Secp256k1Keypair {
            return try {
                BouncyCastleProviderHolder.ensureInstalled()
                val generator = KeyPairGenerator.getInstance("EC", "BC")
                generator.initialize(ECGenParameterSpec("secp256k1"))
                val pair = generator.generateKeyPair()
                Secp256k1Keypair(pair.private, pair.public)
            } catch (e: Exception) {
                throw IllegalStateException("Failed to generate secp256k1 keypair", e)
            }
        }

        fun verifyWithPublicKey(compressedPublicKey: ByteArray, message: ByteArray, signature: ByteArray): Boolean {
            return try {
                BouncyCastleProviderHolder.ensureInstalled()
                val verifier = Signature.getInstance("SHA256withECDSA", "BC")
                verifier.initVerify(publicKeyFromCompressed(compressedPublicKey, "secp256k1"))
                verifier.update(message)
                verifier.verify(signature)
            } catch (_: Exception) {
                false
            }
        }

        private fun publicKeyFromCompressed(compressed: ByteArray, curveName: String): PublicKey {
            return try {
                BouncyCastleProviderHolder.ensureInstalled()
                val bcSpec = ECNamedCurveTable.getParameterSpec(curveName)
                val point = bcSpec.curve.decodePoint(compressed)

                val parameters = AlgorithmParameters.getInstance("EC", "BC")
                parameters.init(ECGenParameterSpec(curveName))
                val ecParameterSpec = parameters.getParameterSpec(ECParameterSpec::class.java)

                val spec = ECPublicKeySpec(
                    java.security.spec.ECPoint(
                        point.affineXCoord.toBigInteger(),
                        point.affineYCoord.toBigInteger(),
                    ),
                    ecParameterSpec,
                )
                KeyFactory.getInstance("EC", "BC").generatePublic(spec)
            } catch (e: Exception) {
                throw IllegalArgumentException("Invalid compressed EC public key", e)
            }
        }
    }

    override fun scheme(): SignatureScheme = SignatureScheme.SECP256K1

    override fun publicKeyBytes(): ByteArray = (publicKey as BCECPublicKey).q.getEncoded(true)

    override fun sign(message: ByteArray): ByteArray {
        return try {
            BouncyCastleProviderHolder.ensureInstalled()
            val signer = Signature.getInstance("SHA256withECDSA", "BC")
            signer.initSign(privateKey)
            signer.update(message)
            signer.sign()
        } catch (e: Exception) {
            throw IllegalStateException("Failed to sign with secp256k1", e)
        }
    }

    override fun verify(message: ByteArray, signature: ByteArray): Boolean {
        return verifyWithPublicKey(publicKeyBytes(), message, signature)
    }
}
