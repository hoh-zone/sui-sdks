package com.suisdks.sui.bcs

class BcsWriter {
    private val buf = ArrayList<Byte>()

    fun writeU8(value: Int) {
        require(value in 0..0xFF) { "u8 out of range" }
        buf.add(value.toByte())
    }

    fun writeU16(value: Int) {
        require(value >= 0) { "negative integer not allowed" }
        require(value <= 0xFFFF) { "u16 out of range" }
        buf.add((value and 0xFF).toByte())
        buf.add(((value ushr 8) and 0xFF).toByte())
    }

    fun writeU32(value: Long) {
        require(value >= 0) { "negative integer not allowed" }
        require(value <= 0xFFFF_FFFFL) { "u32 out of range" }
        for (i in 0 until 4) {
            buf.add(((value ushr (8 * i)) and 0xFF).toByte())
        }
    }

    fun writeU64(value: ULong) {
        for (i in 0 until 8) {
            buf.add(((value shr (8 * i)) and 0xFFu).toByte())
        }
    }

    fun writeU64(value: Long) {
        require(value >= 0) { "negative integer not allowed" }
        writeU64(value.toULong())
    }

    fun writeBool(value: Boolean) {
        writeU8(if (value) 1 else 0)
    }

    fun writeBytes(value: ByteArray) {
        buf.addAll(value.toList())
    }

    fun writeUleb128(value: Long) {
        buf.addAll(encodeUleb128(value).toList())
    }

    fun toByteArray(): ByteArray = buf.toByteArray()
}
