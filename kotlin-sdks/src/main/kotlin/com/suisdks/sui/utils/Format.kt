package com.suisdks.sui.utils

private fun formatHexLike(value: String): String {
    val normalized = normalizeSuiAddress(value, forceAdd0x = true)
    return if (normalized.length <= 14) normalized else "${normalized.take(8)}...${normalized.takeLast(6)}"
}

fun formatAddress(address: String): String = formatHexLike(address)

fun formatDigest(digest: String): String {
    if (digest.length <= 14) return digest
    return "${digest.take(6)}...${digest.takeLast(4)}"
}
