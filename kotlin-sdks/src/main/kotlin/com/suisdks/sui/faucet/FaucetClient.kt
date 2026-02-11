package com.suisdks.sui.faucet

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL

class FaucetRateLimitError(message: String, cause: Throwable? = null) : RuntimeException(message, cause)

fun getFaucetHost(network: String = "testnet"): String {
    val hosts = mapOf(
        "testnet" to "https://faucet.testnet.sui.io/v2/gas",
        "devnet" to "https://faucet.devnet.sui.io/v2/gas",
        "localnet" to "http://127.0.0.1:9123/v2/gas",
    )
    return hosts[network] ?: throw IllegalArgumentException("unsupported faucet network: $network")
}

class FaucetClient(
    private val endpoint: String,
    private val timeoutMs: Int = 30_000,
    private val headers: Map<String, String> = emptyMap(),
) {
    private val gson = Gson()

    companion object {
        fun fromNetwork(
            network: String = "testnet",
            timeoutMs: Int = 30_000,
            headers: Map<String, String> = emptyMap(),
        ): FaucetClient = FaucetClient(getFaucetHost(network), timeoutMs, headers)
    }

    fun requestSuiFromFaucetV2(recipient: String, fixedAmount: Int? = null): Map<String, Any?> {
        val fixed = mutableMapOf<String, Any?>("recipient" to recipient)
        if (fixedAmount != null) {
            fixed["amount"] = fixedAmount
        }
        val payload = mapOf("FixedAmountRequest" to fixed)

        try {
            val conn = (URL(endpoint).openConnection() as HttpURLConnection).apply {
                requestMethod = "POST"
                setRequestProperty("Content-Type", "application/json")
                headers.forEach { (k, v) -> setRequestProperty(k, v) }
                connectTimeout = timeoutMs
                readTimeout = timeoutMs
                doOutput = true
            }
            conn.outputStream.use { it.write(gson.toJson(payload).toByteArray(Charsets.UTF_8)) }
            InputStreamReader(conn.inputStream, Charsets.UTF_8).use {
                val type = object : TypeToken<Map<String, Any?>>() {}.type
                return gson.fromJson(it, type)
            }
        } catch (e: Exception) {
            if ((e.message ?: "").contains("429")) {
                throw FaucetRateLimitError(e.message ?: "429", e)
            }
            throw e
        }
    }
}
