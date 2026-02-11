package com.suisdks.sui.bcs

import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class BcsReaderWriterTest {
    @Test
    fun readWriteRoundtrip() {
        val writer = BcsWriter()
        writer.writeU8(0xAB)
        writer.writeU16(0x1234)
        writer.writeU32(0x89ABCDEFL)
        writer.writeU64(0x0102030405060708u)
        writer.writeBool(true)
        writer.writeBool(false)
        writer.writeBytes(byteArrayOf(0x11, 0x22))
        writer.writeUleb128(300)

        val reader = BcsReader(writer.toByteArray())
        assertEquals(0xAB, reader.readU8())
        assertEquals(0x1234, reader.readU16())
        assertEquals(0x89ABCDEFL, reader.readU32())
        assertEquals(0x0102030405060708u, reader.readU64())
        assertEquals(true, reader.readBool())
        assertEquals(false, reader.readBool())
        assertContentEquals(byteArrayOf(0x11, 0x22), reader.readBytes(2))
        assertEquals(300, reader.readUleb128())
        assertEquals(0, reader.remaining())
    }

    @Test
    fun rejectsInvalidInputs() {
        val reader = BcsReader(byteArrayOf())
        assertFailsWith<IllegalArgumentException> { reader.readU8() }
        assertFailsWith<IllegalArgumentException> { BcsWriter().writeU8(256) }
        assertFailsWith<IllegalArgumentException> { BcsWriter().writeU16(-1) }
        assertFailsWith<IllegalArgumentException> { BcsWriter().writeU32(-1) }
        assertFailsWith<IllegalArgumentException> { BcsWriter().writeU64(-1) }
        assertFailsWith<IllegalArgumentException> { BcsReader(byteArrayOf(2)).readBool() }
    }
}
