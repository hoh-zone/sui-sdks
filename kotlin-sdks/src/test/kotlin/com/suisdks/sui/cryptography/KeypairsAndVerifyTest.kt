package com.suisdks.sui.cryptography

import com.suisdks.sui.keypairs.Ed25519Keypair
import com.suisdks.sui.keypairs.Secp256k1Keypair
import com.suisdks.sui.keypairs.Secp256r1Keypair
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class KeypairsAndVerifyTest {
    @Test
    fun ed25519SignVerifyAndSerializeRoundtrip() {
        val kp = Ed25519Keypair.generate()
        val msg = "hello".toByteArray(Charsets.UTF_8)
        val sig = kp.sign(msg)

        assertTrue(kp.verify(msg, sig))
        assertFalse(kp.verify("other".toByteArray(Charsets.UTF_8), sig))

        val serialized = Verify.toSerializedSignature(SignatureScheme.ED25519, sig, kp.publicKeyBytes())
        val parsed = Verify.parseSerializedSignature(serialized)
        assertEquals(SignatureScheme.ED25519, parsed.scheme)
        assertTrue(Verify.verifyRawSignature(msg, parsed.signature, parsed.publicKey, parsed.scheme))
    }

    @Test
    fun secpKeypairsSignVerify() {
        val msg = "hello".toByteArray(Charsets.UTF_8)

        val k1 = Secp256k1Keypair.generate()
        val sig1 = k1.sign(msg)
        assertTrue(k1.verify(msg, sig1))
        assertEquals(33, k1.publicKeyBytes().size)

        val r1 = Secp256r1Keypair.generate()
        val sig2 = r1.sign(msg)
        assertTrue(r1.verify(msg, sig2))
        assertEquals(33, r1.publicKeyBytes().size)
    }

    @Test
    fun personalMessageVerify() {
        val kp = Ed25519Keypair.generate()
        val msg = "abc".toByteArray(Charsets.UTF_8)
        val payload = Verify.personalMessagePayload(msg)
        val sig = kp.sign(payload)

        assertTrue(Verify.verifyPersonalMessage(msg, sig, kp.publicKeyBytes(), SignatureScheme.ED25519))
    }
}
