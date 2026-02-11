package com.suisdks.sui.jsonrpc

fun getJsonRpcFullnodeUrl(network: String): String = when (network) {
    "mainnet" -> "https://fullnode.mainnet.sui.io:443"
    "testnet" -> "https://fullnode.testnet.sui.io:443"
    "devnet" -> "https://fullnode.devnet.sui.io:443"
    "localnet" -> "http://127.0.0.1:9000"
    else -> throw IllegalArgumentException("unknown network: $network")
}
