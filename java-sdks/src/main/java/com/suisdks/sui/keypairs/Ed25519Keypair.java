package com.suisdks.sui.keypairs;

import com.suisdks.sui.cryptography.BouncyCastleProviderHolder;
import com.suisdks.sui.cryptography.SignatureScheme;
import java.security.KeyFactory;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.Signature;
import java.security.spec.X509EncodedKeySpec;
import java.util.Arrays;

public final class Ed25519Keypair implements Keypair {
    private final PrivateKey privateKey;
    private final PublicKey publicKey;

    private Ed25519Keypair(PrivateKey privateKey, PublicKey publicKey) {
        this.privateKey = privateKey;
        this.publicKey = publicKey;
    }

    public static Ed25519Keypair generate() {
        try {
            BouncyCastleProviderHolder.ensureInstalled();
            KeyPairGenerator g = KeyPairGenerator.getInstance("Ed25519", "BC");
            KeyPair kp = g.generateKeyPair();
            return new Ed25519Keypair(kp.getPrivate(), kp.getPublic());
        } catch (Exception e) {
            throw new IllegalStateException("Failed to generate Ed25519 keypair", e);
        }
    }

    @Override
    public SignatureScheme scheme() {
        return SignatureScheme.ED25519;
    }

    @Override
    public byte[] publicKeyBytes() {
        byte[] x509 = publicKey.getEncoded();
        return Arrays.copyOfRange(x509, x509.length - 32, x509.length);
    }

    public static PublicKey publicKeyFromRaw(byte[] raw32) {
        if (raw32.length != 32) {
            throw new IllegalArgumentException("Ed25519 public key must be 32 bytes");
        }
        // RFC8410 SubjectPublicKeyInfo prefix for Ed25519.
        byte[] prefix = new byte[] {0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00};
        byte[] full = new byte[prefix.length + raw32.length];
        System.arraycopy(prefix, 0, full, 0, prefix.length);
        System.arraycopy(raw32, 0, full, prefix.length, raw32.length);
        try {
            BouncyCastleProviderHolder.ensureInstalled();
            return KeyFactory.getInstance("Ed25519", "BC").generatePublic(new X509EncodedKeySpec(full));
        } catch (Exception e) {
            throw new IllegalArgumentException("Invalid Ed25519 public key bytes", e);
        }
    }

    @Override
    public byte[] sign(byte[] message) {
        try {
            Signature s = Signature.getInstance("Ed25519", "BC");
            s.initSign(privateKey);
            s.update(message);
            return s.sign();
        } catch (Exception e) {
            throw new IllegalStateException("Failed to sign with Ed25519", e);
        }
    }

    @Override
    public boolean verify(byte[] message, byte[] signature) {
        return verifyWithPublicKey(publicKeyBytes(), message, signature);
    }

    public static boolean verifyWithPublicKey(byte[] publicKey, byte[] message, byte[] signature) {
        try {
            Signature s = Signature.getInstance("Ed25519", "BC");
            s.initVerify(publicKeyFromRaw(publicKey));
            s.update(message);
            return s.verify(signature);
        } catch (Exception e) {
            return false;
        }
    }
}
