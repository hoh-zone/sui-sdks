package com.suisdks.sui.multisig;

import java.util.List;

public record MultiSigPublicKey(List<byte[]> publicKeys, List<Integer> weights, int threshold) {
    public static final int MAX_SIGNERS = 10;

    public MultiSigPublicKey {
        if (publicKeys == null || publicKeys.isEmpty()) {
            throw new IllegalArgumentException("publicKeys must not be empty");
        }
        if (publicKeys.size() > MAX_SIGNERS) {
            throw new IllegalArgumentException("too many signers");
        }
        if (weights == null || weights.size() != publicKeys.size()) {
            throw new IllegalArgumentException("weights size mismatch");
        }
        if (threshold <= 0) {
            throw new IllegalArgumentException("threshold must be positive");
        }
        int total = 0;
        for (int w : weights) {
            if (w <= 0) {
                throw new IllegalArgumentException("weights must be positive");
            }
            total += w;
        }
        if (threshold > total) {
            throw new IllegalArgumentException("unreachable threshold");
        }
    }
}
