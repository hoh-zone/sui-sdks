package com.suisdks.sui.cryptography;

public enum SignatureScheme {
    ED25519(0, 32),
    SECP256K1(1, 33),
    SECP256R1(2, 33);

    private final int flag;
    private final int publicKeySize;

    SignatureScheme(int flag, int publicKeySize) {
        this.flag = flag;
        this.publicKeySize = publicKeySize;
    }

    public int getFlag() {
        return flag;
    }

    public int getPublicKeySize() {
        return publicKeySize;
    }

    public static SignatureScheme fromFlag(int flag) {
        for (SignatureScheme scheme : values()) {
            if (scheme.flag == flag) {
                return scheme;
            }
        }
        throw new IllegalArgumentException("Unsupported signature scheme flag: " + flag);
    }
}
