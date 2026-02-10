package com.suisdks.sui.keypairs;

import com.suisdks.sui.cryptography.BouncyCastleProviderHolder;
import com.suisdks.sui.cryptography.SignatureScheme;
import java.security.AlgorithmParameters;
import java.security.KeyFactory;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.Signature;
import java.security.spec.ECGenParameterSpec;
import java.security.spec.ECParameterSpec;
import java.security.spec.ECPublicKeySpec;
import org.bouncycastle.jce.ECNamedCurveTable;
import org.bouncycastle.jce.interfaces.ECPublicKey;

public final class Secp256k1Keypair implements Keypair {
    private final PrivateKey privateKey;
    private final PublicKey publicKey;

    private Secp256k1Keypair(PrivateKey privateKey, PublicKey publicKey) {
        this.privateKey = privateKey;
        this.publicKey = publicKey;
    }

    public static Secp256k1Keypair generate() {
        try {
            BouncyCastleProviderHolder.ensureInstalled();
            KeyPairGenerator g = KeyPairGenerator.getInstance("EC", "BC");
            g.initialize(new ECGenParameterSpec("secp256k1"));
            KeyPair kp = g.generateKeyPair();
            return new Secp256k1Keypair(kp.getPrivate(), kp.getPublic());
        } catch (Exception e) {
            throw new IllegalStateException("Failed to generate secp256k1 keypair", e);
        }
    }

    @Override
    public SignatureScheme scheme() {
        return SignatureScheme.SECP256K1;
    }

    @Override
    public byte[] publicKeyBytes() {
        ECPublicKey ecPub = (ECPublicKey) publicKey;
        return ecPub.getQ().getEncoded(true);
    }

    private static PublicKey publicKeyFromCompressed(byte[] compressed, String curveName) {
        try {
            BouncyCastleProviderHolder.ensureInstalled();
            var bcSpec = ECNamedCurveTable.getParameterSpec(curveName);
            var point = bcSpec.getCurve().decodePoint(compressed);

            AlgorithmParameters parameters = AlgorithmParameters.getInstance("EC", "BC");
            parameters.init(new ECGenParameterSpec(curveName));
            ECParameterSpec ecParameterSpec = parameters.getParameterSpec(ECParameterSpec.class);

            ECPublicKeySpec spec = new ECPublicKeySpec(
                    new java.security.spec.ECPoint(point.getAffineXCoord().toBigInteger(), point.getAffineYCoord().toBigInteger()),
                    ecParameterSpec);
            return KeyFactory.getInstance("EC", "BC").generatePublic(spec);
        } catch (Exception e) {
            throw new IllegalArgumentException("Invalid compressed EC public key", e);
        }
    }

    @Override
    public byte[] sign(byte[] message) {
        try {
            Signature s = Signature.getInstance("SHA256withECDSA", "BC");
            s.initSign(privateKey);
            s.update(message);
            return s.sign();
        } catch (Exception e) {
            throw new IllegalStateException("Failed to sign with secp256k1", e);
        }
    }

    @Override
    public boolean verify(byte[] message, byte[] signature) {
        return verifyWithPublicKey(publicKeyBytes(), message, signature);
    }

    public static boolean verifyWithPublicKey(byte[] compressedPublicKey, byte[] message, byte[] signature) {
        try {
            Signature s = Signature.getInstance("SHA256withECDSA", "BC");
            s.initVerify(publicKeyFromCompressed(compressedPublicKey, "secp256k1"));
            s.update(message);
            return s.verify(signature);
        } catch (Exception e) {
            return false;
        }
    }
}
