package com.suisdks.sui.jsonrpc

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class NetworkTest {
    @Test
    fun knownNetworks() {
        assertEquals("https://fullnode.mainnet.sui.io:443", getJsonRpcFullnodeUrl("mainnet"))
        assertEquals("https://fullnode.testnet.sui.io:443", getJsonRpcFullnodeUrl("testnet"))
        assertEquals("https://fullnode.devnet.sui.io:443", getJsonRpcFullnodeUrl("devnet"))
        assertEquals("http://127.0.0.1:9000", getJsonRpcFullnodeUrl("localnet"))
    }

    @Test
    fun unknownNetwork() {
        assertFailsWith<IllegalArgumentException> {
            getJsonRpcFullnodeUrl("unknown")
        }
    }
}
