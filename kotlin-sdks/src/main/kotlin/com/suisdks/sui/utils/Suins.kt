package com.suisdks.sui.utils

private val SUINS_PATTERN = Regex("^[a-z0-9-]{1,63}(\\.[a-z0-9-]{1,63})*\\.(sui|move)$")

fun isValidSuiNSName(name: String): Boolean {
    val n = name.trim().lowercase()
    return SUINS_PATTERN.matches(n)
}

fun normalizeSuiNSName(name: String, format: String = "at"): String {
    val n = name.trim().lowercase().removePrefix("@")
    require(isValidSuiNSName(n)) { "invalid suins name: $name" }
    return when (format) {
        "at" -> "@$n"
        "dot" -> n
        else -> throw IllegalArgumentException("unsupported suins format: $format")
    }
}
