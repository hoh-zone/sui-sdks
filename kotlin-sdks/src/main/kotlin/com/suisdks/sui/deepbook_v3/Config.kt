package com.suisdks.sui.deepbook_v3

const val FLOAT_SCALAR = 1_000_000_000.0
const val DEEP_SCALAR = 1_000_000.0
const val MAX_TIMESTAMP: Long = 1_844_674_407_370_955_161

data class PackageIds(
    val deepbookPackageId: String,
    val registryId: String,
    val deepTreasuryId: String,
    val marginPackageId: String,
    val marginRegistryId: String,
)

val TESTNET_PACKAGE_IDS = PackageIds(
    deepbookPackageId = "0x22be4cade64bf2d02412c7e8d0e8beea2f78828b948118d46735315409371a3c",
    registryId = "0x7c256edbda983a2cd6f946655f4bf3f00a41043993781f8674a7046e8c0e11d1",
    deepTreasuryId = "0x69fffdae0075f8f71f4fa793549c11079266910e8905169845af1f5d00e09dcb",
    marginPackageId = "0xd6a42f4df4db73d68cbeb52be66698d2fe6a9464f45ad113ca52b0c6ebd918b6",
    marginRegistryId = "0x48d7640dfae2c6e9ceeada197a7a1643984b5a24c55a0c6c023dac77e0339f75",
)

val TESTNET_COINS: Map<String, Coin> = mapOf(
    "DEEP" to Coin(
        address = "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8",
        type = "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP",
        scalar = 1_000_000.0,
    ),
    "SUI" to Coin(
        address = "0x2",
        type = "0x2::sui::SUI",
        scalar = 1_000_000_000.0,
    ),
    "DBUSDC" to Coin(
        address = "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7",
        type = "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC",
        scalar = 1_000_000.0,
    ),
)

val TESTNET_POOLS: Map<String, Pool> = mapOf(
    "DEEP_SUI" to Pool(
        address = "0x48c95963e9eac37a316b7ae04a0deb761bcdcc2b67912374d6036e7f0e9bae9f",
        baseCoin = "DEEP",
        quoteCoin = "SUI",
    ),
)

data class DeepBookConfig(
    val address: String,
    val network: String = "testnet",
    val balanceManagers: Map<String, BalanceManager> = emptyMap(),
    val marginManagers: Map<String, MarginManager> = emptyMap(),
    val coins: Map<String, Coin> = TESTNET_COINS,
    val pools: Map<String, Pool> = TESTNET_POOLS,
    val packageIds: PackageIds = TESTNET_PACKAGE_IDS,
) {
    companion object {
        fun testnet(
            address: String,
            balanceManagers: Map<String, BalanceManager> = emptyMap(),
            marginManagers: Map<String, MarginManager> = emptyMap(),
        ): DeepBookConfig = DeepBookConfig(
            address = address,
            network = "testnet",
            balanceManagers = balanceManagers,
            marginManagers = marginManagers,
            coins = TESTNET_COINS,
            pools = TESTNET_POOLS,
            packageIds = TESTNET_PACKAGE_IDS,
        )
    }

    fun getCoin(key: String): Coin = coins[key] ?: throw NoSuchElementException("coin not found: $key")
    fun getPool(key: String): Pool = pools[key] ?: throw NoSuchElementException("pool not found: $key")
    fun getBalanceManager(key: String): BalanceManager =
        balanceManagers[key] ?: throw NoSuchElementException("balance manager not found: $key")

    fun getMarginManager(key: String): MarginManager =
        marginManagers[key] ?: throw NoSuchElementException("margin manager not found: $key")
}
