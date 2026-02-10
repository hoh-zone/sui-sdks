package com.suisdks.sui.cryptography

import java.security.Security
import org.bouncycastle.jce.provider.BouncyCastleProvider

object BouncyCastleProviderHolder {
    fun ensureInstalled() {
        if (Security.getProvider("BC") == null) {
            Security.addProvider(BouncyCastleProvider())
        }
    }
}
