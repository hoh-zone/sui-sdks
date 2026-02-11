package com.suisdks.sui.bcs;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.ArrayList;
import java.util.List;

public class BcsWriter {
    private final ByteArrayOutputStream output;
    private ByteOrder byteOrder;

    public BcsWriter() {
        this.output = new ByteArrayOutputStream();
        this.byteOrder = ByteOrder.LITTLE_ENDIAN;
    }

    public byte[] toByteArray() {
        return output.toByteArray();
    }

    public void writeUInt8(int value) {
        output.write(value & 0xFF);
    }

    public void writeUInt16(int value) {
        writeBytes(ByteBuffer.allocate(2).order(byteOrder).putShort((short) value).array());
    }

    public void writeUInt32(int value) {
        writeBytes(ByteBuffer.allocate(4).order(byteOrder).order(byteOrder).putInt(value).array());
    }

    public void writeUInt64(long value) {
        writeBytes(ByteBuffer.allocate(8).order(byteOrder).putLong(value).array());
    }

    public void writeInt8(byte value) {
        output.write(value);
    }

    public void writeInt16(short value) {
        writeBytes(ByteBuffer.allocate(2).order(byteOrder).putShort(value).array());
    }

    public void writeInt32(int value) {
        writeBytes(ByteBuffer.allocate(4).order(byteOrder).putInt(value).array());
    }

    public void writeInt64(long value) {
        writeBytes(ByteBuffer.allocate(8).order(byteOrder).putLong(value).array());
    }

    public void writeBool(boolean value) {
        writeUInt8(value ? 1 : 0);
    }

    public void writeString(String value) {
        byte[] bytes = value.getBytes(java.nio.charset.StandardCharsets.UTF_8);
        writeULEB128(bytes.length);
        output.write(bytes, 0, bytes.length);
    }

    public void writeBytes(byte[] bytes) {
        try {
            output.write(bytes);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public void writeULEB128(long value) {
        do {
            byte b = (byte) (value & 0x7FL);
            value >>>= 7;
            if (value != 0) {
                b |= 0x80L;
            }
            output.write(b);
        } while (value != 0);
    }

    public void writeAddress(String address) {
        if (!address.startsWith("0x")) {
            throw new IllegalArgumentException("Invalid address format");
        }
        byte[] bytes = hexToBytes(address.substring(2));
        output.write(bytes, bytes.length - 20, 20);
    }

    private static byte[] hexToBytes(String hex) {
        int len = hex.length();
        byte[] data = new byte[len / 2];
        for (int i = 0; i < len; i += 2) {
            data[i / 2] = (byte) ((Character.digit(hex.charAt(i), 16) << 4)
                    + Character.digit(hex.charAt(i + 1), 16));
        }
        return data;
    }
}

public class BcsReader {
    private final java.io.ByteArrayInputStream input;
    private ByteOrder byteOrder;

    public BcsReader(byte[] data) {
        this.input = new java.io.ByteArrayInputStream(data);
        this.byteOrder = ByteOrder.LITTLE_ENDIAN;
    }

    public byte readUInt8() {
        return (byte) input.read();
    }

    public int readUInt16() {
        byte[] bytes = readBytes(2);
        return ByteBuffer.wrap(bytes).order(byteOrder).getShort() & 0xFFFF;
    }

    public int readUInt32() {
        byte[] bytes = readBytes(4);
        return ByteBuffer.wrap(bytes).order(byteOrder).getInt() & 0xFFFFFFFFL;
    }

    public long readUInt64() {
        byte[] bytes = readBytes(8);
        return ByteBuffer.wrap(bytes).order(byteOrder).getLong() & 0xFFFFFFFFFFFFFFFFL;
    }

    public byte readInt8() {
        return readUInt8();
    }

    public short readInt16() {
        byte[] bytes = readBytes(2);
        return ByteBuffer.wrap(bytes).order(byteOrder).getShort();
    }

    public int readInt32() {
        byte[] bytes = readBytes(4);
        return ByteBuffer.wrap(bytes).order(byteOrder).getInt();
    }

    public long readInt64() {
        byte[] bytes = readBytes(8);
        return ByteBuffer.wrap(bytes).order(byteOrder).getLong();
    }

    public boolean readBool() {
        return readUInt8() != 0;
    }

    public String readString() {
        int length = readULEB128();
        byte[] bytes = readBytes(length);
        return new String(bytes, java.nio.charset.StandardCharsets.UTF_8);
    }

    public long readULEB128() {
        long result = 0;
        int shift = 0;
        byte b;
        do {
            b = readUInt8();
            result |= ((b & 0x7FL) << shift);
            shift += 7;
        } while ((b & 0x80L) != 0);
        return result;
    }

    public String readAddress() {
        byte[] bytes = readBytes(20);
        StringBuilder sb = new StringBuilder();
        sb.append("0x");
        for (byte b : bytes) {
            sb.append(String.format("%02x", b & 0xFF));
        }
        return sb.toString();
    }

    private byte[] readBytes(int length) {
        byte[] bytes = new byte[length];
        int read = 0;
        while (read < length) {
            int n = input.read(bytes, read, length - read);
            if (n < 0) {
                throw new RuntimeException("Unexpected end of input");
            }
            read += n;
        }
        return bytes;
    }
}

public class ULEB128 {
    public static byte[] encode(long value) {
        java.io.ByteArrayOutputStream output = new java.io.ByteArrayOutputStream();
        do {
            byte b = (byte) (value & 0x7FL);
            value >>>= 7;
            if (value != 0) {
                b |= 0x80L;
            }
            output.write(b);
        } while (value != 0);
        return output.toByteArray();
    }

    public static long decode(byte[] data, int offset) {
        long result = 0;
        int shift = 0;
        int i = offset;
        while (true) {
            if (i >= data.length) {
                throw new RuntimeException("ULEB128 decode error: unexpected end of data");
            }
            byte b = data[i++];
            result |= ((b & 0x7FL) << shift);
            shift += 7;
            if ((b & 0x80L) == 0) {
                break;
            }
        }
        return result;
    }

    public static long decode(byte[] data) {
        return decode(data, 0);
    }
}