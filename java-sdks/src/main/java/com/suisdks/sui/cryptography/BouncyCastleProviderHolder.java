package com.suisdks.sui.cryptography;

import java.security.Security;
import org.bouncycastle.jce.provider.BouncyCastleProvider;

public final class BouncyCastleProviderHolder {
    private BouncyCastleProviderHolder() {
    }

    public static void ensureInstalled() {
        if (Security.getProvider("BC") == null) {
            Security.addProvider(new BouncyCastleProvider());
        }
    }
}
