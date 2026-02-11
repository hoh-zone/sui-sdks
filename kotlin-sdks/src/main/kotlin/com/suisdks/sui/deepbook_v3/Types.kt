package com.suisdks.sui.deepbook_v3

data class BalanceManager(
    val address: String,
    val tradeCap: String = "",
    val depositCap: String = "",
    val withdrawCap: String = "",
)

data class MarginManager(
    val address: String,
    val poolKey: String,
)

data class Coin(
    val address: String,
    val type: String,
    val scalar: Double,
    val feed: String? = null,
    val priceInfoObjectId: String? = null,
)

data class Pool(
    val address: String,
    val baseCoin: String,
    val quoteCoin: String,
)
