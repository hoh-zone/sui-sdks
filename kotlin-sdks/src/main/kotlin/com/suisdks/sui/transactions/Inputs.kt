package com.suisdks.sui.transactions

import java.util.Base64

object Inputs {
    fun pure(value: ByteArray): Map<String, Any?> = mapOf(
        "\$kind" to "Pure",
        "Pure" to mapOf("bytes" to Base64.getEncoder().encodeToString(value)),
    )

    fun objectRef(ref: ObjectRef): Map<String, Any?> = mapOf(
        "\$kind" to "Object",
        "Object" to mapOf(
            "\$kind" to "ImmOrOwnedObject",
            "ImmOrOwnedObject" to mapOf(
                "objectId" to ref.objectId,
                "digest" to ref.digest,
                "version" to ref.version,
            ),
        ),
    )

    fun sharedObjectRef(objectId: String, mutable: Boolean, initialSharedVersion: Long): Map<String, Any?> = mapOf(
        "\$kind" to "Object",
        "Object" to mapOf(
            "\$kind" to "SharedObject",
            "SharedObject" to mapOf(
                "objectId" to objectId,
                "mutable" to mutable,
                "initialSharedVersion" to initialSharedVersion,
            ),
        ),
    )
}
