package com.suisdks.sui.cryptography;

public record SerializedSignature(SignatureScheme scheme, byte[] signature, byte[] publicKey) {
}
