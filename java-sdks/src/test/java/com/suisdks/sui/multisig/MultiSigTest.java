package com.suisdks.sui.multisig;

import com.suisdks.sui.cryptography.SignatureScheme;
import com.suisdks.sui.keypairs.Ed25519Keypair;
import java.nio.charset.StandardCharsets;
import java.util.List;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class MultiSigTest {
    @Test
    void thresholdVerification() {
        var k1 = Ed25519Keypair.generate();
        var k2 = Ed25519Keypair.generate();
        var k3 = Ed25519Keypair.generate();

        var pub = new MultiSigPublicKey(
                List.of(k1.publicKeyBytes(), k2.publicKeyBytes(), k3.publicKeyBytes()),
                List.of(1, 1, 1),
                2);

        byte[] msg = "m".getBytes(StandardCharsets.UTF_8);

        var ok = new MultiSigSignature(List.of(k1.sign(msg), k2.sign(msg)), List.of(0, 1));
        Assertions.assertTrue(MultiSigVerifier.verify(msg, pub, ok, SignatureScheme.ED25519));

        var low = new MultiSigSignature(List.of(k1.sign(msg)), List.of(0));
        Assertions.assertFalse(MultiSigVerifier.verify(msg, pub, low, SignatureScheme.ED25519));
    }
}
