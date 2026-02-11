package com.suisdks.sui.bcs

const val MAX_ULEB128_VALUE: Long = 0xFFFF_FFFFL

fun encodeUleb128(value: Long): ByteArray {
    require(value >= 0) { "uleb128 cannot encode negative value" }
    if (value == 0L) {
        return byteArrayOf(0)
    }

    var v = value
    val out = ArrayList<Byte>()
    while (v > 0) {
        var b = (v and 0x7F).toInt()
        v = v ushr 7
        if (v > 0) {
            b = b or 0x80
        }
        out.add(b.toByte())
    }
    return out.toByteArray()
}

fun decodeUleb128(data: ByteArray, offset: Int = 0): Pair<Long, Int> {
    require(offset >= 0 && offset <= data.size) { "invalid offset: $offset" }

    var total = 0L
    var shift = 0

    for (i in offset until data.size) {
        val byte = data[i].toInt() and 0xFF
        total = total or ((byte and 0x7F).toLong() shl shift)

        if ((byte and 0x80) == 0) {
            val consumed = i - offset + 1
            require(total <= MAX_ULEB128_VALUE) { "uleb128 exceeds u32 range" }
            val canonical = encodeUleb128(total)
            require(canonical.size == consumed) { "non-canonical uleb128 encoding" }
            return total to consumed
        }

        shift += 7
        require(shift < 64) { "uleb128 overflow" }
    }

    throw IllegalArgumentException("uleb128 buffer overflow")
}
