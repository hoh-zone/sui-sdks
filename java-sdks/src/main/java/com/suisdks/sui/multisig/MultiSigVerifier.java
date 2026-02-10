package com.suisdks.sui.multisig;

import com.suisdks.sui.cryptography.SignatureScheme;
import com.suisdks.sui.cryptography.Verify;
import java.util.HashSet;

public final class MultiSigVerifier {
    private MultiSigVerifier() {
    }

    public static boolean verify(byte[] message, MultiSigPublicKey pub, MultiSigSignature sig, SignatureScheme scheme) {
        if (sig.signatures().size() != sig.bitmap().size()) {
            return false;
        }
        var used = new HashSet<Integer>();
        int weight = 0;
        for (int i = 0; i < sig.signatures().size(); i++) {
            int idx = sig.bitmap().get(i);
            if (!used.add(idx)) {
                return false;
            }
            if (idx < 0 || idx >= pub.publicKeys().size()) {
                return false;
            }
            if (!Verify.verifyRawSignature(message, sig.signatures().get(i), pub.publicKeys().get(idx), scheme)) {
                return false;
            }
            weight += pub.weights().get(idx);
        }
        return weight >= pub.threshold();
    }
}
