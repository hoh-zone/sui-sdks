package com.suisdks.sui.multisig

import com.suisdks.sui.cryptography.SignatureScheme
import com.suisdks.sui.cryptography.Verify

object MultiSigVerifier {
    fun verify(message: ByteArray, pub: MultiSigPublicKey, sig: MultiSigSignature, scheme: SignatureScheme): Boolean {
        try {
            sig.validate()
        } catch (_: IllegalArgumentException) {
            return false
        }
        val used = mutableSetOf<Int>()
        var weight = 0

        sig.signatures.indices.forEach { i ->
            val idx = sig.bitmap[i]
            if (!used.add(idx)) {
                return false
            }
            if (idx !in pub.publicKeys.indices) {
                return false
            }
            if (!Verify.verifyRawSignature(message, sig.signatures[i], pub.publicKeys[idx], scheme)) {
                return false
            }
            weight += pub.weights[idx]
        }

        return weight >= pub.threshold
    }
}
