package com.suisdks.sui.verify

import com.suisdks.sui.cryptography.SignatureScheme
import com.suisdks.sui.cryptography.Verify

object VerifyFacade {
    fun verifySignature(
        scheme: SignatureScheme,
        publicKey: ByteArray,
        message: ByteArray,
        signature: ByteArray,
    ): Boolean = Verify.verifyRawSignature(message, signature, publicKey, scheme)

    fun verifyPersonalMessage(
        scheme: SignatureScheme,
        publicKey: ByteArray,
        message: ByteArray,
        signature: ByteArray,
    ): Boolean = Verify.verifyPersonalMessage(message, signature, publicKey, scheme)
}
