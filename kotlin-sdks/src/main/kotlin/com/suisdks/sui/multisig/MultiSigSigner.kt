package com.suisdks.sui.multisig

import com.suisdks.sui.cryptography.SignatureScheme

data class IndexedSignature(val index: Int, val signature: ByteArray)

data class IndexedSigner(val index: Int, val signer: MessageSigner)

fun interface MessageSigner {
    fun sign(message: ByteArray): ByteArray
}

class MultiSigSigner(
    private val publicKey: MultiSigPublicKey,
    private val scheme: SignatureScheme = SignatureScheme.ED25519,
) {
    fun verify(message: ByteArray, multiSig: MultiSigSignature): Boolean {
        return MultiSigVerifier.verify(message, publicKey, multiSig, scheme)
    }

    fun isThresholdMet(multiSig: MultiSigSignature): Boolean {
        return try {
            multiSig.validate()
            val totalWeight = multiSig.bitmap.fold(0) { acc, idx ->
                if (idx !in publicKey.weights.indices) return false
                acc + publicKey.weights[idx]
            }
            totalWeight >= publicKey.threshold
        } catch (_: IllegalArgumentException) {
            false
        }
    }

    fun build(message: ByteArray, indexedSignatures: List<IndexedSignature>, requireThreshold: Boolean = true): MultiSigSignature {
        require(indexedSignatures.isNotEmpty()) { "no signatures provided" }
        val multiSig = MultiSigSignature(
            signatures = indexedSignatures.map { it.signature },
            bitmap = indexedSignatures.map { it.index },
        )
        multiSig.validate()
        require(verify(message, multiSig)) { "invalid multisig signatures" }
        require(!requireThreshold || isThresholdMet(multiSig)) { "threshold not met" }
        return multiSig
    }

    fun sign(message: ByteArray, indexedSigners: List<IndexedSigner>, requireThreshold: Boolean = true): MultiSigSignature {
        require(indexedSigners.isNotEmpty()) { "no signers provided" }
        val signatures = indexedSigners.map { IndexedSignature(index = it.index, signature = it.signer.sign(message)) }
        return build(message = message, indexedSignatures = signatures, requireThreshold = requireThreshold)
    }
}
