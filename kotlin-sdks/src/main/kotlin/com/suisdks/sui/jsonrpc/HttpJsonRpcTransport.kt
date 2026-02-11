package com.suisdks.sui.jsonrpc

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL

class HttpJsonRpcTransport(
    private val endpoint: String,
    private val timeoutMs: Int = 30_000,
    private val headers: Map<String, String> = emptyMap(),
) : JsonRpcTransport {
    private val gson = Gson()

    override fun request(method: String, params: List<Any?>): Map<String, Any?> {
        return try {
            val conn = (URL(endpoint).openConnection() as HttpURLConnection).apply {
                requestMethod = "POST"
                setRequestProperty("Content-Type", "application/json")
                headers.forEach { (k, v) -> setRequestProperty(k, v) }
                connectTimeout = timeoutMs
                readTimeout = timeoutMs
                doOutput = true
            }

            val payload = mapOf(
                "jsonrpc" to "2.0",
                "id" to 1,
                "method" to method,
                "params" to params,
            )

            conn.outputStream.use { it.write(gson.toJson(payload).toByteArray(Charsets.UTF_8)) }
            InputStreamReader(conn.inputStream, Charsets.UTF_8).use {
                val type = object : TypeToken<Map<String, Any?>>() {}.type
                gson.fromJson<Map<String, Any?>>(it, type)
            }
        } catch (e: Exception) {
            throw IllegalStateException("JSON-RPC request failed: $method", e)
        }
    }
}
