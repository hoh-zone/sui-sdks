package com.suisdks.sui.graphql

import com.sun.net.httpserver.HttpServer
import java.net.InetSocketAddress
import java.util.concurrent.Executors
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class GraphQLClientTest {
    @Test
    fun queryAndExecute() {
        var capturedRequestBody = ""
        var capturedApiKey = ""
        val server = HttpServer.create(InetSocketAddress("127.0.0.1", 0), 0)
        server.createContext("/") { exchange ->
            capturedRequestBody = exchange.requestBody.bufferedReader(Charsets.UTF_8).use { it.readText() }
            capturedApiKey = exchange.requestHeaders.getFirst("x-api-key") ?: ""
            val response = "{\"data\":{\"echo\":\"ok\"}}"
            exchange.responseHeaders.add("Content-Type", "application/json")
            exchange.sendResponseHeaders(200, response.toByteArray().size.toLong())
            exchange.responseBody.use { it.write(response.toByteArray()) }
        }
        server.executor = Executors.newSingleThreadExecutor()
        server.start()

        try {
            val endpoint = "http://127.0.0.1:${server.address.port}/"
            val client = GraphQLClient(
                endpoint = endpoint,
                timeoutMs = 1000,
                headers = mapOf("x-api-key" to "k"),
            )
            val out = client.execute("query { ping }")

            @Suppress("UNCHECKED_CAST")
            val data = out["data"] as Map<String, Any?>
            assertEquals("ok", data["echo"])

            val named = GraphQLClient(endpoint, queries = mapOf("q1" to "query { ping }"))
            val namedOut = named.executeNamed("q1")
            @Suppress("UNCHECKED_CAST")
            val namedData = namedOut["data"] as Map<String, Any?>
            assertEquals("ok", namedData["echo"])

            val persisted = client.executePersistedQuery(
                queryText = "query Ping { ping }",
                sha256Hash = "abc123",
                operationName = "Ping",
            )
            @Suppress("UNCHECKED_CAST")
            val persistedData = persisted["data"] as Map<String, Any?>
            assertEquals("ok", persistedData["echo"])
            assertEquals("k", capturedApiKey)
            assertEquals(true, capturedRequestBody.contains("\"persistedQuery\""))
            assertEquals(true, capturedRequestBody.contains("\"sha256Hash\":\"abc123\""))
        } finally {
            server.stop(0)
        }
    }

    @Test
    fun throwsOnGraphQLErrors() {
        val server = HttpServer.create(InetSocketAddress("127.0.0.1", 0), 0)
        server.createContext("/") { exchange ->
            val response = "{\"errors\":[{\"message\":\"boom\"}]}"
            exchange.responseHeaders.add("Content-Type", "application/json")
            exchange.sendResponseHeaders(200, response.toByteArray().size.toLong())
            exchange.responseBody.use { it.write(response.toByteArray()) }
        }
        server.executor = Executors.newSingleThreadExecutor()
        server.start()

        try {
            val endpoint = "http://127.0.0.1:${server.address.port}/"
            val client = GraphQLClient(endpoint)
            assertFailsWith<IllegalStateException> {
                client.execute("query { ping }")
            }
        } finally {
            server.stop(0)
        }
    }

    @Test
    fun throwsOnInvalidQuerySyntaxBeforeRequest() {
        val client = GraphQLClient("http://127.0.0.1:9/")
        assertFailsWith<IllegalStateException> {
            client.execute("query {")
        }
    }

    @Test
    fun executeNamedThrowsOnUnknownQuery() {
        val client = GraphQLClient("http://127.0.0.1:9/")
        assertFailsWith<IllegalArgumentException> {
            client.executeNamed("missing")
        }
    }
}
