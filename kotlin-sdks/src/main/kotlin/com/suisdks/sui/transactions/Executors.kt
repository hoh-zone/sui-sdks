package com.suisdks.sui.transactions

import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CompletableFuture
import java.util.concurrent.Executors

class CachingExecutor(private val client: DynamicClient) {
    private val cache = ConcurrentHashMap<String, Map<String, Any?>>()

    fun executeTransaction(tx: Transaction): Map<String, Any?> {
        val key = tx.buildBase64()
        return cache.computeIfAbsent(key) {
            tx.execute(client = client)
        }
    }
}

class SerialExecutor(private val executor: CachingExecutor) {
    fun execute(txs: List<Transaction>): List<Map<String, Any?>> = txs.map { executor.executeTransaction(it) }
}

class ParallelExecutor(
    private val executor: CachingExecutor,
    private val maxWorkers: Int = 4,
) {
    fun execute(txs: List<Transaction>): List<Map<String, Any?>> {
        val pool = Executors.newFixedThreadPool(maxWorkers)
        return try {
            val futures = txs.map { tx -> pool.submit<Map<String, Any?>> { executor.executeTransaction(tx) } }
            futures.map { it.get() }
        } finally {
            pool.shutdownNow()
        }
    }
}

class AsyncCachingExecutor(
    private val client: DynamicClient,
) {
    private val cache = ConcurrentHashMap<String, CompletableFuture<Map<String, Any?>>>()

    fun executeTransaction(tx: Transaction): CompletableFuture<Map<String, Any?>> {
        val key = tx.buildBase64()
        return cache.computeIfAbsent(key) {
            tx.executeAsync(client = client)
        }
    }
}

class AsyncSerialExecutor(private val executor: AsyncCachingExecutor) {
    fun execute(txs: List<Transaction>): CompletableFuture<List<Map<String, Any?>>> {
        var chain = CompletableFuture.completedFuture(emptyList<Map<String, Any?>>())
        txs.forEach { tx ->
            chain = chain.thenCompose { acc ->
                executor.executeTransaction(tx).thenApply { result -> acc + result }
            }
        }
        return chain
    }
}

class AsyncParallelExecutor(
    private val executor: AsyncCachingExecutor,
    private val maxWorkers: Int = 4,
) {
    fun execute(txs: List<Transaction>): CompletableFuture<List<Map<String, Any?>>> {
        val pool = Executors.newFixedThreadPool(maxWorkers)
        val futures = txs.map { tx ->
            CompletableFuture.supplyAsync({ executor.executeTransaction(tx).get() }, pool)
        }
        return CompletableFuture.allOf(*futures.toTypedArray()).whenComplete { _, _ ->
            pool.shutdownNow()
        }.thenApply {
            futures.map { it.get() }
        }
    }
}
