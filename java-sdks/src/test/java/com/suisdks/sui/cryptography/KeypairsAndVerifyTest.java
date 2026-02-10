package com.suisdks.sui.cryptography;

import com.suisdks.sui.keypairs.Ed25519Keypair;
import com.suisdks.sui.keypairs.Secp256k1Keypair;
import com.suisdks.sui.keypairs.Secp256r1Keypair;
import java.nio.charset.StandardCharsets;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class KeypairsAndVerifyTest {
    @Test
    void ed25519SignVerifyAndSerializeRoundtrip() {
        var kp = Ed25519Keypair.generate();
        byte[] msg = "hello".getBytes(StandardCharsets.UTF_8);
        byte[] sig = kp.sign(msg);

        Assertions.assertTrue(kp.verify(msg, sig));
        Assertions.assertFalse(kp.verify("other".getBytes(StandardCharsets.UTF_8), sig));

        String ser = Verify.toSerializedSignature(SignatureScheme.ED25519, sig, kp.publicKeyBytes());
        SerializedSignature parsed = Verify.parseSerializedSignature(ser);
        Assertions.assertEquals(SignatureScheme.ED25519, parsed.scheme());
        Assertions.assertTrue(Verify.verifyRawSignature(msg, parsed.signature(), parsed.publicKey(), parsed.scheme()));
    }

    @Test
    void secpKeypairsSignVerify() {
        byte[] msg = "hello".getBytes(StandardCharsets.UTF_8);

        var k1 = Secp256k1Keypair.generate();
        byte[] sig1 = k1.sign(msg);
        Assertions.assertTrue(k1.verify(msg, sig1));
        Assertions.assertEquals(33, k1.publicKeyBytes().length);

        var r1 = Secp256r1Keypair.generate();
        byte[] sig2 = r1.sign(msg);
        Assertions.assertTrue(r1.verify(msg, sig2));
        Assertions.assertEquals(33, r1.publicKeyBytes().length);
    }

    @Test
    void personalMessageVerify() {
        var kp = Ed25519Keypair.generate();
        byte[] msg = "abc".getBytes(StandardCharsets.UTF_8);
        byte[] payload = Verify.personalMessagePayload(msg);
        byte[] sig = kp.sign(payload);

        Assertions.assertTrue(Verify.verifyPersonalMessage(msg, sig, kp.publicKeyBytes(), SignatureScheme.ED25519));
    }
}
