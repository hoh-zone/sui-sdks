package com.suisdks.sui.utils

data class StructTag(
    val address: String,
    val module: String,
    val name: String,
    val typeParams: List<String> = emptyList(),
)

fun parseStructTag(type: String): StructTag {
    val trimmed = type.trim()
    require(trimmed.isNotEmpty()) { "type must not be empty" }

    val lt = trimmed.indexOf('<')
    val base = if (lt >= 0) trimmed.substring(0, lt) else trimmed
    val paramsRaw = if (lt >= 0) {
        require(trimmed.endsWith(">")) { "invalid struct tag generic tail: $trimmed" }
        trimmed.substring(lt + 1, trimmed.length - 1)
    } else {
        ""
    }

    val parts = base.split("::")
    require(parts.size == 3) { "invalid struct tag format: $trimmed" }

    val typeParams = if (paramsRaw.isBlank()) emptyList() else splitTypeParams(paramsRaw)
    return StructTag(
        address = normalizeSuiAddress(parts[0], forceAdd0x = true),
        module = parts[1],
        name = parts[2],
        typeParams = typeParams,
    )
}

fun normalizeStructTag(type: String): String = normalizeStructTag(parseStructTag(type))

fun normalizeStructTag(type: StructTag): String {
    val params = if (type.typeParams.isEmpty()) {
        ""
    } else {
        type.typeParams.joinToString(prefix = "<", postfix = ">") { param ->
            runCatching { normalizeStructTag(parseStructTag(param)) }.getOrElse { param.trim() }
        }
    }
    return "${normalizeSuiAddress(type.address, forceAdd0x = true)}::${type.module}::${type.name}$params"
}

private fun splitTypeParams(input: String): List<String> {
    val out = mutableListOf<String>()
    val current = StringBuilder()
    var depth = 0
    input.forEach { ch ->
        when (ch) {
            '<' -> {
                depth += 1
                current.append(ch)
            }
            '>' -> {
                depth -= 1
                require(depth >= 0) { "unbalanced generic brackets: $input" }
                current.append(ch)
            }
            ',' -> {
                if (depth == 0) {
                    out.add(current.toString().trim())
                    current.clear()
                } else {
                    current.append(ch)
                }
            }
            else -> current.append(ch)
        }
    }
    if (current.isNotBlank()) {
        out.add(current.toString().trim())
    }
    require(depth == 0) { "unbalanced generic brackets: $input" }
    return out
}
