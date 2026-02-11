package com.suisdks.sui.batch

import java.util.concurrent.CompletableFuture
import java.util.concurrent.Executor
import java.util.concurrent.ForkJoinPool

fun <T, R> mapSync(items: Iterable<T>, fn: (T) -> R): List<R> = items.map(fn)

fun <T, R> mapAsync(
    items: Iterable<T>,
    fn: (T) -> R,
    executor: Executor = ForkJoinPool.commonPool(),
): CompletableFuture<List<R>> {
    val futures = items.map { item -> CompletableFuture.supplyAsync({ fn(item) }, executor) }
    return CompletableFuture.allOf(*futures.toTypedArray()).thenApply { futures.map { it.join() } }
}
