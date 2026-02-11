package com.suisdks.sui.batch;

import java.util.List;
import java.util.ArrayList;
import java.util.function.Function;
import java.util.concurrent.CompletableFuture;

public class BatchUtils {

    public static <T, R> List<R> mapSync(List<T> items, Function<T, R> fn) {
        List<R> results = new ArrayList<>(items.size());
        for (T item : items) {
            results.add(fn.apply(item));
        }
        return results;
    }

    public static <T, R> CompletableFuture<List<R>> mapAsync(List<T> items, Function<T, CompletableFuture<R>> fn) {
        List<CompletableFuture<R>> futures = new ArrayList<>(items.size());
        for (T item : items) {
            futures.add(fn.apply(item));
        }
        return CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
                .thenApply(v -> {
                    List<R> results = new ArrayList<>(futures.size());
                    for (CompletableFuture<R> future : futures) {
                        results.add(future.join());
                    }
                    return results;
                });
    }

    public static <T> CompletableFuture<List<T>> executeAll(List<CompletableFuture<T>> futures) {
        return CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
                .thenApply(v -> {
                    List<T> results = new ArrayList<>(futures.size());
                    for (CompletableFuture<T> future : futures) {
                        results.add(future.join());
                    }
                    return results;
                });
    }
}