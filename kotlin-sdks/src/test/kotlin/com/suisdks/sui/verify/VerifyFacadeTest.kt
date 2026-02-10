package com.suisdks.sui.verify

import com.suisdks.sui.cryptography.SignatureScheme
import com.suisdks.sui.keypairs.Ed25519Keypair
import kotlin.test.Test
import kotlin.test.assertTrue

class VerifyFacadeTest {
    @Test
    fun verifyRawAndPersonalMessage() {
        val kp = Ed25519Keypair.generate()
        val msg = "hi".toByteArray()

        val rawSig = kp.sign(msg)
        assertTrue(
            VerifyFacade.verifySignature(
                scheme = SignatureScheme.ED25519,
                publicKey = kp.publicKeyBytes(),
                message = msg,
                signature = rawSig,
            ),
        )

        val personalPayload = "\u0019Sui Signed Message:\n${msg.size}\n".toByteArray(Charsets.UTF_8) + msg
        val personalSig = kp.sign(personalPayload)
        assertTrue(
            VerifyFacade.verifyPersonalMessage(
                scheme = SignatureScheme.ED25519,
                publicKey = kp.publicKeyBytes(),
                message = msg,
                signature = personalSig,
            ),
        )
    }
}
