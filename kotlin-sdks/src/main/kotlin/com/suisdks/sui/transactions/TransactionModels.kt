package com.suisdks.sui.transactions

data class ObjectRef(
    val objectId: String,
    val digest: String,
    val version: Long,
)

data class TransactionData(
    var sender: String = "",
    var expiration: Any? = null,
    val gasData: MutableMap<String, Any?> = mutableMapOf(),
    val inputs: MutableList<Map<String, Any?>> = mutableListOf(),
    val commands: MutableList<Map<String, Any?>> = mutableListOf(),
)
