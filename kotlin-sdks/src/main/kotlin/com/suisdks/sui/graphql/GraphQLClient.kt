package com.suisdks.sui.graphql

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import graphql.parser.Parser
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL

data class GraphQLRequest(
    val query: String,
    val variables: Map<String, Any?> = emptyMap(),
    val operationName: String? = null,
    val extensions: Map<String, Any?>? = null,
)

class GraphQLClient(
    private val endpoint: String,
    private val headers: Map<String, String> = emptyMap(),
) {
    private val gson = Gson()
    private val parser = Parser()

    fun execute(
        query: String,
        variables: Map<String, Any?> = emptyMap(),
        operationName: String? = null,
        extensions: Map<String, Any?>? = null,
    ): Map<String, Any?> = query(
        GraphQLRequest(
            query = query,
            variables = variables,
            operationName = operationName,
            extensions = extensions,
        ),
    )

    fun query(request: GraphQLRequest): Map<String, Any?> {
        try {
            parser.parseDocument(request.query)

            val conn = (URL(endpoint).openConnection() as HttpURLConnection).apply {
                requestMethod = "POST"
                setRequestProperty("Content-Type", "application/json")
                headers.forEach { (k, v) -> setRequestProperty(k, v) }
                doOutput = true
            }
            conn.outputStream.use { it.write(gson.toJson(request).toByteArray(Charsets.UTF_8)) }

            if (conn.responseCode !in 200..299) {
                throw GraphQLRequestError(conn.responseCode, conn.responseMessage ?: "request failed")
            }

            InputStreamReader(conn.inputStream, Charsets.UTF_8).use {
                val type = object : TypeToken<Map<String, Any?>>() {}.type
                val parsed = gson.fromJson<Map<String, Any?>>(it, type)
                if (parsed["errors"] != null) {
                    throw IllegalStateException("graphql error: ${parsed["errors"]}")
                }
                return parsed
            }
        } catch (e: GraphQLRequestError) {
            throw e
        } catch (e: Exception) {
            throw IllegalStateException("graphql request failed", e)
        }
    }
}

class GraphQLRequestError(val statusCode: Int, status: String) : RuntimeException("graphql request failed: $status ($statusCode)")
