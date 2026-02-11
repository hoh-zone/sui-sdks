package com.suisdks.sui.bcs

class BcsReader(
    private val data: ByteArray,
    private var pos: Int = 0,
) {
    fun remaining(): Int = data.size - pos

    fun readU8(): Int {
        ensureAvailable(1)
        return data[pos++].toInt() and 0xFF
    }

    fun readU16(): Int {
        val b = readBytes(2)
        return (b[0].toInt() and 0xFF) or ((b[1].toInt() and 0xFF) shl 8)
    }

    fun readU32(): Long {
        val b = readBytes(4)
        return (b[0].toLong() and 0xFF) or
            ((b[1].toLong() and 0xFF) shl 8) or
            ((b[2].toLong() and 0xFF) shl 16) or
            ((b[3].toLong() and 0xFF) shl 24)
    }

    fun readU64(): ULong {
        val b = readBytes(8)
        return (b[0].toULong() and 0xFFu) or
            ((b[1].toULong() and 0xFFu) shl 8) or
            ((b[2].toULong() and 0xFFu) shl 16) or
            ((b[3].toULong() and 0xFFu) shl 24) or
            ((b[4].toULong() and 0xFFu) shl 32) or
            ((b[5].toULong() and 0xFFu) shl 40) or
            ((b[6].toULong() and 0xFFu) shl 48) or
            ((b[7].toULong() and 0xFFu) shl 56)
    }

    fun readBytes(n: Int): ByteArray {
        require(n >= 0) { "bcs: out of range" }
        ensureAvailable(n)
        val out = data.copyOfRange(pos, pos + n)
        pos += n
        return out
    }

    fun readBool(): Boolean {
        return when (val v = readU8()) {
            0 -> false
            1 -> true
            else -> throw IllegalArgumentException("invalid bool byte: $v")
        }
    }

    fun readUleb128(): Long {
        val (value, consumed) = decodeUleb128(data, pos)
        pos += consumed
        return value
    }

    private fun ensureAvailable(n: Int) {
        if (remaining() < n) {
            throw IllegalArgumentException("bcs: out of range")
        }
    }
}
