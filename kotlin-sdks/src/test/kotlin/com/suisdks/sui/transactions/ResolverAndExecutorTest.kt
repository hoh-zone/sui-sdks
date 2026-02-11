package com.suisdks.sui.transactions

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue
import java.util.concurrent.CompletableFuture
import java.util.concurrent.ExecutionException

class ResolverAndExecutorTest {
    @Test
    fun resolverFindsUnresolvedAndRunsPlugins() {
        val tx = Transaction()
        tx.obj("0x1")

        var seen = 0
        val resolver = Resolver().apply {
            addPlugin { context -> seen = context.unresolvedInputs.size }
        }
        val context = resolver.resolve(tx)

        assertEquals(1, context.unresolvedInputs.size)
        assertEquals(1, seen)
    }

    @Test
    fun resolverWrapsPluginErrors() {
        val tx = Transaction()
        val resolver = Resolver().apply {
            addPlugin { _ -> error("boom") }
        }

        val err = assertFailsWith<ResolverPluginError> { resolver.resolve(tx) }
        assertTrue(err.message!!.contains("resolver plugin failed"))
    }

    @Test
    fun asyncResolverRunsPluginsAndWrapsErrors() {
        val tx = Transaction()
        tx.obj("0x1")

        val resolver = AsyncResolver().apply {
            addPlugin { context ->
                context.unresolvedInputs.add(mapOf("\$kind" to "UnresolvedObject", "UnresolvedObject" to mapOf("objectId" to "0x2")))
                CompletableFuture.completedFuture(Unit)
            }
        }
        val context = resolver.resolve(tx).get()
        assertEquals(2, context.unresolvedInputs.size)

        val failing = AsyncResolver().apply {
            addPlugin { CompletableFuture.failedFuture(IllegalStateException("boom")) }
        }
        val err = assertFailsWith<ExecutionException> {
            failing.resolve(tx).get()
        }
        assertTrue((err.cause as? ResolverPluginError)?.message?.contains("resolver plugin failed") == true)
    }

    @Test
    fun executorsCacheAndRun() {
        var calls = 0
        val client = object : DynamicClient {
            override fun call(method: String, params: List<Any?>): Any {
                calls += 1
                return mapOf("result" to mapOf("digest" to "0xabc", "params_size" to params.size))
            }
        }

        val tx = Transaction(client)
        tx.setSender("0xabc")
        tx.transferSui("0xdef", 1)

        val caching = CachingExecutor(client)
        val serial = SerialExecutor(caching)
        val parallel = ParallelExecutor(caching, maxWorkers = 2)

        serial.execute(listOf(tx, tx))
        parallel.execute(listOf(tx, tx))
        AsyncSerialExecutor(AsyncCachingExecutor(client)).execute(listOf(tx, tx)).get()

        assertEquals(1, calls)
    }
}
