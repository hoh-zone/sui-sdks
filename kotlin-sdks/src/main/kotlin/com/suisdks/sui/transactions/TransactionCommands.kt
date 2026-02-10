package com.suisdks.sui.transactions

import java.util.Base64

object TransactionCommands {
    fun moveCall(target: String, args: List<Map<String, Any?>>, typeArgs: List<String> = emptyList()): Map<String, Any?> {
        val parts = target.split("::") + listOf("", "", "")
        return mapOf(
            "\$kind" to "MoveCall",
            "MoveCall" to mapOf(
                "package" to parts[0],
                "module" to parts[1],
                "function" to parts[2],
                "arguments" to args,
                "typeArguments" to typeArgs,
            ),
        )
    }

    fun splitCoins(coin: Map<String, Any?>, amounts: List<Map<String, Any?>>): Map<String, Any?> = mapOf(
        "\$kind" to "SplitCoins",
        "SplitCoins" to mapOf("coin" to coin, "amounts" to amounts),
    )

    fun transferObjects(objects: List<Map<String, Any?>>, address: Map<String, Any?>): Map<String, Any?> = mapOf(
        "\$kind" to "TransferObjects",
        "TransferObjects" to mapOf("objects" to objects, "address" to address),
    )

    fun mergeCoins(destination: Map<String, Any?>, sources: List<Map<String, Any?>>): Map<String, Any?> = mapOf(
        "\$kind" to "MergeCoins",
        "MergeCoins" to mapOf("destination" to destination, "sources" to sources),
    )

    fun publish(modules: List<ByteArray>, dependencies: List<String>): Map<String, Any?> = mapOf(
        "\$kind" to "Publish",
        "Publish" to mapOf(
            "modules" to modules.map { Base64.getEncoder().encodeToString(it) },
            "dependencies" to dependencies,
        ),
    )

    fun upgrade(
        modules: List<ByteArray>,
        dependencies: List<String>,
        packageId: String,
        ticket: Map<String, Any?>,
    ): Map<String, Any?> = mapOf(
        "\$kind" to "Upgrade",
        "Upgrade" to mapOf(
            "modules" to modules.map { Base64.getEncoder().encodeToString(it) },
            "dependencies" to dependencies,
            "package" to packageId,
            "ticket" to ticket,
        ),
    )

    fun makeMoveVec(typeTag: String?, elements: List<Map<String, Any?>>): Map<String, Any?> = mapOf(
        "\$kind" to "MakeMoveVec",
        "MakeMoveVec" to mapOf("type" to typeTag, "elements" to elements),
    )
}
