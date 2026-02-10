package com.suisdks.sui.cryptography;

import com.suisdks.sui.keypairs.Ed25519Keypair;
import com.suisdks.sui.keypairs.Secp256k1Keypair;
import com.suisdks.sui.keypairs.Secp256r1Keypair;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Base64;

public final class Verify {
    private Verify() {
    }

    public static boolean verifyRawSignature(byte[] message, byte[] signature, byte[] publicKey, SignatureScheme scheme) {
        return switch (scheme) {
            case ED25519 -> Ed25519Keypair.verifyWithPublicKey(publicKey, message, signature);
            case SECP256K1 -> Secp256k1Keypair.verifyWithPublicKey(publicKey, message, signature);
            case SECP256R1 -> Secp256r1Keypair.verifyWithPublicKey(publicKey, message, signature);
        };
    }

    public static boolean verifyPersonalMessage(byte[] message, byte[] signature, byte[] publicKey, SignatureScheme scheme) {
        byte[] prefixed = personalMessagePayload(message);
        return verifyRawSignature(prefixed, signature, publicKey, scheme);
    }

    public static byte[] personalMessagePayload(byte[] message) {
        byte[] prefix = "\u0019Sui Signed Message:\n".getBytes(StandardCharsets.UTF_8);
        byte[] len = Integer.toString(message.length).getBytes(StandardCharsets.UTF_8);
        byte[] nl = "\n".getBytes(StandardCharsets.UTF_8);
        byte[] out = new byte[prefix.length + len.length + nl.length + message.length];
        int off = 0;
        System.arraycopy(prefix, 0, out, off, prefix.length);
        off += prefix.length;
        System.arraycopy(len, 0, out, off, len.length);
        off += len.length;
        System.arraycopy(nl, 0, out, off, nl.length);
        off += nl.length;
        System.arraycopy(message, 0, out, off, message.length);
        return out;
    }

    public static String toSerializedSignature(SignatureScheme scheme, byte[] signature, byte[] publicKey) {
        if (publicKey.length != scheme.getPublicKeySize()) {
            throw new IllegalArgumentException("Public key length mismatch for scheme " + scheme);
        }
        byte[] out = new byte[1 + signature.length + publicKey.length];
        out[0] = (byte) scheme.getFlag();
        System.arraycopy(signature, 0, out, 1, signature.length);
        System.arraycopy(publicKey, 0, out, 1 + signature.length, publicKey.length);
        return Base64.getEncoder().encodeToString(out);
    }

    public static SerializedSignature parseSerializedSignature(String serialized) {
        byte[] raw = Base64.getDecoder().decode(serialized);
        if (raw.length < 2) {
            throw new IllegalArgumentException("Serialized signature too short");
        }
        SignatureScheme scheme = SignatureScheme.fromFlag(raw[0] & 0xff);
        int pkLen = scheme.getPublicKeySize();
        if (raw.length <= 1 + pkLen) {
            throw new IllegalArgumentException("Serialized signature too short for scheme");
        }
        byte[] signature = Arrays.copyOfRange(raw, 1, raw.length - pkLen);
        byte[] publicKey = Arrays.copyOfRange(raw, raw.length - pkLen, raw.length);
        return new SerializedSignature(scheme, signature, publicKey);
    }
}
