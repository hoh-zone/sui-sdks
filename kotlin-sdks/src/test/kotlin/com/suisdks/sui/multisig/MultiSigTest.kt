package com.suisdks.sui.multisig

import com.suisdks.sui.cryptography.SignatureScheme
import com.suisdks.sui.keypairs.Ed25519Keypair
import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class MultiSigTest {
    @Test
    fun thresholdVerification() {
        val k1 = Ed25519Keypair.generate()
        val k2 = Ed25519Keypair.generate()
        val k3 = Ed25519Keypair.generate()

        val pub = MultiSigPublicKey(
            publicKeys = listOf(k1.publicKeyBytes(), k2.publicKeyBytes(), k3.publicKeyBytes()),
            weights = listOf(1, 1, 1),
            threshold = 2,
        )

        val msg = "m".toByteArray(Charsets.UTF_8)

        val ok = MultiSigSignature(signatures = listOf(k1.sign(msg), k2.sign(msg)), bitmap = listOf(0, 1))
        assertTrue(MultiSigVerifier.verify(msg, pub, ok, SignatureScheme.ED25519))

        val low = MultiSigSignature(signatures = listOf(k1.sign(msg)), bitmap = listOf(0))
        assertFalse(MultiSigVerifier.verify(msg, pub, low, SignatureScheme.ED25519))
    }
}
