package com.suisdks.sui.utils

private val HEX_REGEX = Regex("^[0-9a-fA-F]+$")
private val BASE58_REGEX = Regex("^[1-9A-HJ-NP-Za-km-z]+$")

fun normalizeSuiAddress(value: String, forceAdd0x: Boolean = false): String {
    val raw = value.removePrefix("0x").removePrefix("0X").lowercase()
    require(raw.isNotEmpty()) { "address must not be empty" }
    require(raw.length <= 64) { "address hex too long" }
    require(HEX_REGEX.matches(raw)) { "address must be hex" }

    val padded = raw.padStart(64, '0')
    return if (forceAdd0x || value.startsWith("0x") || value.startsWith("0X")) "0x$padded" else padded
}

fun normalizeSuiObjectId(value: String, forceAdd0x: Boolean = false): String =
    normalizeSuiAddress(value, forceAdd0x)

fun isValidSuiAddress(value: String): Boolean = runCatching {
    normalizeSuiAddress(value, forceAdd0x = true)
}.isSuccess

fun isValidSuiObjectId(value: String): Boolean = isValidSuiAddress(value)

fun isValidTransactionDigest(value: String): Boolean {
    if (value.isBlank()) return false
    // Sui digest is generally base58-encoded.
    return BASE58_REGEX.matches(value)
}
