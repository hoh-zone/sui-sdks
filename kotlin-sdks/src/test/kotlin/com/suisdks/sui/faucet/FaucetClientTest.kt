package com.suisdks.sui.faucet

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class FaucetClientTest {
    @Test
    fun networkHostMapping() {
        assertEquals("https://faucet.testnet.sui.io/v2/gas", getFaucetHost("testnet"))
        assertEquals("https://faucet.devnet.sui.io/v2/gas", getFaucetHost("devnet"))
    }

    @Test
    fun unsupportedNetworkThrows() {
        assertFailsWith<IllegalArgumentException> {
            getFaucetHost("mainnet")
        }
    }
}
