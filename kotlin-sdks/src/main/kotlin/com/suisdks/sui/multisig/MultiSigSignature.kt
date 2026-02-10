package com.suisdks.sui.multisig

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import java.util.Base64

data class MultiSigSignature(
    val signatures: List<ByteArray>,
    val bitmap: List<Int>,
) {
    fun validate() {
        require(signatures.size == bitmap.size) { "signatures and bitmap length mismatch" }
        require(signatures.size <= MultiSigPublicKey.MAX_SIGNERS) { "too many signatures" }
        require(bitmap.toSet().size == bitmap.size) { "duplicate bitmap index" }
        require(bitmap.all { it >= 0 }) { "bitmap index must be non-negative" }
    }

    fun toBase64(): String {
        validate()
        val payload = mapOf(
            "bitmap" to bitmap,
            "signatures" to signatures.map { Base64.getEncoder().encodeToString(it) },
        )
        return Base64.getEncoder().encodeToString(Gson().toJson(payload).toByteArray(Charsets.UTF_8))
    }

    companion object {
        fun fromBase64(serialized: String): MultiSigSignature {
            val raw = Base64.getDecoder().decode(serialized)
            val type = object : TypeToken<Map<String, Any?>>() {}.type
            val payload = Gson().fromJson<Map<String, Any?>>(raw.toString(Charsets.UTF_8), type)

            @Suppress("UNCHECKED_CAST")
            val signatures = (payload["signatures"] as? List<String>).orEmpty().map { Base64.getDecoder().decode(it) }
            @Suppress("UNCHECKED_CAST")
            val bitmap = (payload["bitmap"] as? List<Number>).orEmpty().map { it.toInt() }
            return MultiSigSignature(signatures = signatures, bitmap = bitmap)
        }
    }
}
