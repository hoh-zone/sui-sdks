package com.suisdks.sui.multisig

import com.suisdks.sui.cryptography.SignatureScheme
import com.suisdks.sui.keypairs.Ed25519Keypair
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class MultiSigEncodingAndSignerTest {
    @Test
    fun base64RoundTrip() {
        val sig = MultiSigSignature(
            signatures = listOf("a".toByteArray(), "b".toByteArray()),
            bitmap = listOf(0, 2),
        )
        val encoded = sig.toBase64()
        val decoded = MultiSigSignature.fromBase64(encoded)

        assertEquals(listOf(0, 2), decoded.bitmap)
        assertEquals("a", decoded.signatures[0].toString(Charsets.UTF_8))
        assertEquals("b", decoded.signatures[1].toString(Charsets.UTF_8))
    }

    @Test
    fun signerBuildAndVerify() {
        val k1 = Ed25519Keypair.generate()
        val k2 = Ed25519Keypair.generate()
        val pub = MultiSigPublicKey(
            publicKeys = listOf(k1.publicKeyBytes(), k2.publicKeyBytes()),
            weights = listOf(1, 1),
            threshold = 2,
        )
        val signer = MultiSigSigner(pub, SignatureScheme.ED25519)
        val msg = "hello".toByteArray()

        val msig = signer.sign(
            message = msg,
            indexedSigners = listOf(
                IndexedSigner(0) { m -> k1.sign(m) },
                IndexedSigner(1) { m -> k2.sign(m) },
            ),
        )

        assertTrue(signer.isThresholdMet(msig))
        assertTrue(signer.verify(msg, msig))
    }
}
