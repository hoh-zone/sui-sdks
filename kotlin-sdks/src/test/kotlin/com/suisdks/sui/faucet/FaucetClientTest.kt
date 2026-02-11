package com.suisdks.sui.faucet

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class FaucetClientTest {
    @Test
    fun networkHostMapping() {
        assertEquals("https://faucet.testnet.sui.io/v2/gas", getFaucetHost("testnet"))
        assertEquals("https://faucet.devnet.sui.io/v2/gas", getFaucetHost("devnet"))
        assertEquals("http://127.0.0.1:9123/v2/gas", getFaucetHost("localnet"))
    }

    @Test
    fun unsupportedNetworkThrows() {
        assertFailsWith<IllegalArgumentException> {
            getFaucetHost("mainnet")
        }
    }

    @Test
    fun factoryWithTimeoutAndHeaders() {
        FaucetClient.fromNetwork(
            network = "testnet",
            timeoutMs = 1000,
            headers = mapOf("x-api-key" to "k"),
        )
    }
}
