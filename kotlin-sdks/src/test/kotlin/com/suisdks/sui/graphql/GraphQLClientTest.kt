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
        val server = HttpServer.create(InetSocketAddress("127.0.0.1", 0), 0)
        server.createContext("/") { exchange ->
            val response = "{\"data\":{\"echo\":\"ok\"}}"
            exchange.responseHeaders.add("Content-Type", "application/json")
            exchange.sendResponseHeaders(200, response.toByteArray().size.toLong())
            exchange.responseBody.use { it.write(response.toByteArray()) }
        }
        server.executor = Executors.newSingleThreadExecutor()
        server.start()

        try {
            val endpoint = "http://127.0.0.1:${server.address.port}/"
            val client = GraphQLClient(endpoint)
            val out = client.execute("query { ping }")

            @Suppress("UNCHECKED_CAST")
            val data = out["data"] as Map<String, Any?>
            assertEquals("ok", data["echo"])
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
}
