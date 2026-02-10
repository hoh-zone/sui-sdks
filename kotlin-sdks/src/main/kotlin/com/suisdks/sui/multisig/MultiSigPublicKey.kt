package com.suisdks.sui.multisig

data class MultiSigPublicKey(
    val publicKeys: List<ByteArray>,
    val weights: List<Int>,
    val threshold: Int,
) {
    companion object {
        const val MAX_SIGNERS = 10
    }

    init {
        require(publicKeys.isNotEmpty()) { "publicKeys must not be empty" }
        require(publicKeys.size <= MAX_SIGNERS) { "too many signers" }
        require(weights.size == publicKeys.size) { "weights size mismatch" }
        require(threshold > 0) { "threshold must be positive" }
        require(weights.all { it > 0 }) { "weights must be positive" }
        require(threshold <= weights.sum()) { "unreachable threshold" }
    }
}
