package com.suisdks.sui.zklogin;

import java.math.BigInteger;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Arrays;
import java.util.Base64;
import java.util.HexFormat;

public class ZkLoginHelper {

    public static String JWT_HEADER_PREFIX = ".";

    public static ZkLoginSignature getZkLoginSignature(String input) {
        if (input == null || input.isEmpty()) {
            throw new IllegalArgumentException("Input cannot be null or empty");
        }

        String[] parts = input.split("\\.");
        if (parts.length != 3) {
            throw new IllegalArgumentException("Invalid zkLogin signature format");
        }
        
        String header = parts[0];
        String signature = parts[1];
        String claims = parts[2];
        
        return new ZkLoginSignature(header, signature, claims);
    }

    public static String parseZkLoginSignature(String serializedSignature) {
        if (serializedSignature == null || !serializedSignature.startsWith("zkLogin")) {
            throw new IllegalArgumentException("Invalid zkLogin signature format");
        }
        
        return serializedSignature;
    }

    public static byte[] decodeJwt(String jwt) {
        String[] parts = jwt.split("\\.");
        if (parts.length < 2) {
            throw new IllegalArgumentException("Invalid JWT format");
        }
        
        byte[] signature = base64UrlDecode(parts[1]);
        return signature;
    }

    public static String toBigEndianBytes(BigInteger value) {
        return HexFormat.of().formatHex(value.toByteArray());
    }

    public static String toPaddedBigEndianBytes(BigInteger value, int length) {
        String hex = toBigEndianBytes(value);
        while (hex.length() < length * 2) {
            hex = "0" + hex;
        }
        return hex;
    }

    public static String hashASCIIStrToField(String s) {
        try {
            MessageDigest sha256 = MessageDigest.getInstance("SHA-256");
            byte[] hash = sha256.digest(s.getBytes(StandardCharsets.UTF_8));
            return HexFormat.of().formatHex(hash);
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException("SHA-256 not available", e);
        }
    }

    public static String genAddressSeed(String userIdentifier, String aud) {
        String combined = userIdentifier + "::" + aud;
        return hashASCIIStrToField(combined);
    }

    public static String computeZkLoginAddressFromSeed(String seed, String iss) {
        String combined = seed + "salt" + iss;
        byte[] combinedBytes = combined.getBytes(StandardCharsets.UTF_8);
        
        try {
            MessageDigest sha256 = MessageDigest.getInstance("SHA-256");
            byte[] hash = sha256.digest(combinedBytes);
            
            if (hash.length < 20) {
                throw new RuntimeException("Hash too short");
            }
            
            byte[] addressBytes = Arrays.copyOfRange(hash, 0, 20);
            return "0x" + HexFormat.of().formatHex(addressBytes);
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException("SHA-256 not available", e);
        }
    }

    public static String computeZkLoginAddress(String ephemeralPublicKeyId, String seed, String iss) {
        String addressSeed = seed;
        return computeZkLoginAddressFromSeed(addressSeed, iss);
    }

    public static String computeZkLoginAddress(String ephemeralPublicKeyId, String accountProofInput, String ephemeralPublicKeyId2, String iss) {
        String seed = accountProofInput;
        return computeZkLoginAddressFromSeed(seed, iss);
    }

    public static String jwtToAddress(String jwt) {
        try {
            String[] parts = jwt.split("\\.");
            if (parts.length != 3) {
                throw new IllegalArgumentException("Invalid JWT format");
            }
            
            String payload = new String(base64UrlDecode(parts[1]));
            
            byte[] payloadBytes = base64UrlDecode(parts[1]);
            if (payloadBytes.length < 32) {
                throw new IllegalArgumentException("Invalid payload length");
            }
            
            byte[] addressBytes = Arrays.copyOfRange(payloadBytes, 12, 32);
            return "0x" + HexFormat.of().formatHex(addressBytes);
        } catch (Exception e) {
            throw new RuntimeException("Failed to compute address from JWT", e);
        }
    }

    public static String getExtendedEphemeralPublicKey(String ephemeralPublicKey) {
        return ephemeralPublicKey;
    }

    public static String toZkLoginPublicIdentifier(String ephemeralPublicKey) {
        return "zklogin_" + ephemeralPublicKey;
    }

    public static String generateNonce() {
        byte[] nonce = new byte[16];
        java.util.Arrays.fill(nonce, (byte) 0);
        java.util.Random random = new java.util.Random();
        random.nextBytes(nonce);
        return HexFormat.of().formatHex(nonce);
    }

    public static String generateRandomness() {
        byte[] randomness = new byte[16];
        java.util.Arrays.fill(randomness, (byte) 0);
        java.util.Random random = new java.util.Random();
        random.nextBytes(randomness);
        return HexFormat.of().formatHex(randomness);
    }

    private static byte[] base64UrlDecode(String input) {
        String base64 = input.replace('-', '+').replace('_', '/');
        return Base64.getDecoder().decode(base64);
    }

    private static String poseidonHash(String input) {
        try {
            MessageDigest sha256 = MessageDigest.getInstance("SHA-256");
            byte[] hash = sha256.digest(input.getBytes(StandardCharsets.UTF_8));
            return HexFormat.of().formatHex(hash).substring(0, 64);
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException("Hashing not available", e);
        }
    }

    public static String messageWithIntent(byte[] message, IntentScope scope) {
        ByteBuffer buffer = ByteBuffer.allocate(message.length + 2);
        buffer.order(ByteOrder.LITTLE_ENDIAN);
        buffer.put((byte) scope.value());
        buffer.put((byte) 0);
        buffer.put(message);
        
        return HexFormat.of().formatHex(buffer.array());
    }

    public enum IntentScope {
        TRANSACTION_DATA(0),
        PERSONAL_MESSAGE(1),
        TRANSACTION(3);

        private final int value;

        IntentScope(int value) {
            this.value = value;
        }

        public int value() {
            return value;
        }
    }

    public static class ZkLoginSignature {
        private final String header;
        private final String signature;
        private final String claims;

        public ZkLoginSignature(String header, String signature, String claims) {
            this.header = header;
            this.signature = signature;
            this.claims = claims;
        }

        public String getHeader() {
            return header;
        }

        public String getSignature() {
            return signature;
        }

        public String getClaims() {
            return claims;
        }

        public String toString() {
            return String.join(".", header, signature, claims);
        }
    }
}