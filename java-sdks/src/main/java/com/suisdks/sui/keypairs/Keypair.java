package com.suisdks.sui.keypairs;

import com.suisdks.sui.cryptography.SignatureScheme;

public interface Keypair {
    SignatureScheme scheme();

    byte[] publicKeyBytes();

    byte[] sign(byte[] message);

    boolean verify(byte[] message, byte[] signature);
}
