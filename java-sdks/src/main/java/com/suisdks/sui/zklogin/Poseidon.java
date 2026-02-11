package com.suisdks.sui.zklogin;

import java.math.BigInteger;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.SecureRandom;
import java.util.Base64;
import java.util.HexFormat;

public class Poseidon {

    private static final BigInteger F = new BigInteger("21888242871839275222246405745257275088696311157297823662689037894645226208583");

    public static String poseidonHex(String input) {
        byte[] bytes = input.getBytes(StandardCharsets.UTF_8);
        return poseidon(bytes);
    }

    public static String poseidon(byte[] input) {
        BigInteger hash = hashToField(input);
        return hash.toString(16);
    }

    public static BigInteger hashToField(byte[] input) {
        try {
            MessageDigest sha256 = MessageDigest.getInstance("SHA-256");
            byte[] sha256Hash = sha256.digest(input);

            byte[] sha512Hash = sha256.digest(sha256Hash);

            byte[] hash = new byte[32];
            System.arraycopy(sha512Hash, 0, hash, 0, 32);

            return new BigInteger(1, hash);
        } catch (Exception e) {
            throw new RuntimeException("Poseidon hashing failed", e);
        }
    }

    public static String poseidonHashWithFields(String... fields) {
        try {
            ByteBuffer buffer = ByteBuffer.allocate(fields.length * 32);
            buffer.order(ByteOrder.LITTLE_ENDIAN);

            for (String field : fields) {
                BigInteger value = new BigInteger(field, 16);
                byte[] bytes = value.toByteArray();

                if (bytes.length > 32) {
                    throw new IllegalArgumentException("Field value too large: " + field);
                }

                byte[] padded = new byte[32];
                System.arraycopy(bytes, 0, padded, 32 - bytes.length, bytes.length);

                buffer.put(padded);
            }

            return poseidon(buffer.array());
        } catch (Exception e) {
            throw new RuntimeException("Poseidon hashing failed", e);
        }
    }

    public static String hashToFieldLe(BigInteger value) {
        String hex = value.toString(16);
        while (hex.length() < 64) {
            hex = "0" + hex;
        }
        return hex;
    }

    public static String padToHex(BigInteger value) {
        return padToHex(value, 64);
    }

    public static String padToHex(BigInteger value, int length) {
        String hex = value.toString(16);
        while (hex.length() < length) {
            hex = "0" + hex;
        }
        return hex;
    }
}