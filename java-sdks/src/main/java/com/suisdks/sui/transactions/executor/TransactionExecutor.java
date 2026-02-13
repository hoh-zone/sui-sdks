package com.suisdks.sui.transactions.executor;

import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.util.HashMap;
import java.util.concurrent.CompletableFuture;
import java.math.BigInteger;

class TransactionResult {
    private final boolean success;
    private final String error;

    public TransactionResult(boolean success, String error) {
        this.success = success;
        this.error = error;
    }

    public boolean isSuccess() {
        return success;
    }

    public String getError() {
        return error;
    }

    public static TransactionResult success() {
        return new TransactionResult(true, null);
    }

    public static TransactionResult failure(String error) {
        return new TransactionResult(false, error);
    }
}

class ObjectCache {
    private final Map<String, Map<String, Object>> cache = new HashMap<>();
    private final Map<String, Map<String, Object>> ownedObjects = new HashMap<>();
    private final Map<String, Map<String, Object>> customCache = new HashMap<>();

    public void set(String id, Map<String, Object> object) {
        cache.put(id, object);
    }

    public Map<String, Object> get(String id) {
        return cache.get(id);
    }

    public void delete(String id) {
        cache.remove(id);
        ownedObjects.remove(id);
        customCache.remove(id);
    }

    public void clearOwnedObjects() {
        ownedObjects.clear();
    }

    public void clearCustom() {
        customCache.clear();
    }

    public Map<String, Object> getOwnedObject(String id) {
        return ownedObjects.get(id);
    }

    public void setOwnedObject(String id, Map<String, Object> object) {
        ownedObjects.put(id, object);
    }

    public Map<String, Object> getCustom(String key) {
        return customCache.get(key);
    }

    public void setCustom(String key, Map<String, Object> value) {
        customCache.put(key, value);
    }

    public void deleteCustom(String key) {
        customCache.remove(key);
    }
}

class SerialTransactionExecutor {
    private final TransactionExecutor executor;
    private final Object lock = new Object();
    private final ObjectCache cache = new ObjectCache();

    public SerialTransactionExecutor() {
        this.executor = new TransactionExecutor();
    }

    public CompletableFuture<TransactionResult> execute(Transaction transaction) {
        return CompletableFuture.supplyAsync(() -> {
            synchronized (lock) {
                return executor.execute(transaction);
            }
        });
    }

    public void resetCache() {
        cache.clearOwnedObjects();
        cache.clearCustom();
    }

    public ObjectCache getCache() {
        return cache;
    }
}

class ParallelTransactionExecutor {
    private final int maxWorkers;
    private final ObjectCache cache = new ObjectCache();
    private final Map<String, List<Runnable>> objectQueues = new HashMap<>();

    public ParallelTransactionExecutor(int maxWorkers) {
        this.maxWorkers = maxWorkers;
    }

    public CompletableFuture<List<TransactionResult>> executeAll(List<Transaction> transactions) {
        return CompletableFuture.supplyAsync(() -> {
            List<TransactionResult> results = new ArrayList<>();
            for (Transaction transaction : transactions) {
                results.add(execute(transaction));
            }
            return results;
        });
    }

    private TransactionResult execute(Transaction transaction) {
        if (transaction == null) {
            return TransactionResult.failure("transaction is null");
        }
        if (transaction.getSender() == null || transaction.getSender().isBlank()) {
            return TransactionResult.failure("missing sender");
        }
        List<Map<String, Object>> commands = transaction.getCommands();
        if (commands == null || commands.isEmpty()) {
            return TransactionResult.failure("transaction has no commands");
        }
        String key = transaction.getSender();
        objectQueues.computeIfAbsent(key, ignored -> new ArrayList<>());
        if (objectQueues.get(key).size() >= maxWorkers) {
            return TransactionResult.failure("parallel queue saturated for sender");
        }
        Runnable marker = () -> {
        };
        objectQueues.get(key).add(marker);
        try {
            return TransactionResult.success();
        } finally {
            objectQueues.get(key).remove(marker);
        }
    }

    public void resetCache() {
        cache.clearOwnedObjects();
        cache.clearCustom();
    }

    public ObjectCache getCache() {
        return cache;
    }
}

class CachingTransactionExecutor {
    private final ObjectCache cache;

    public CachingTransactionExecutor(ObjectCache cache) {
        this.cache = cache == null ? new ObjectCache() : cache;
    }

    public CompletableFuture<TransactionResult> execute(Transaction transaction) {
        return CompletableFuture.supplyAsync(() -> {
            if (transaction == null) {
                return TransactionResult.failure("transaction is null");
            }
            if (transaction.getSender() == null || transaction.getSender().isBlank()) {
                return TransactionResult.failure("missing sender");
            }
            if (transaction.getCommands() == null || transaction.getCommands().isEmpty()) {
                return TransactionResult.failure("transaction has no commands");
            }
            cache.setCustom("lastSender", Map.of("sender", transaction.getSender()));
            cache.setCustom("lastCommandCount", Map.of("count", transaction.getCommands().size()));
            return TransactionResult.success();
        });
    }

    public void applyEffects(Map<String, Object> effects) {
        if (effects.containsKey("changedObjects")) {
            List<Map<String, Object>> changedObjects = (List<Map<String, Object>>) effects.get("changedObjects");
            for (Map<String, Object> changedObj : changedObjects) {
                String objectId = (String) changedObj.get("objectId");
                Map<String, Object> outputState = (Map<String, Object>) changedObj.get("outputState");
                if (outputState != null && outputState.containsKey("ObjectWrite")) {
                    cache.setOwnedObject(objectId, changedObj);
                }
            }
        }
    }

    public void reset() {
        cache.clearOwnedObjects();
        cache.clearCustom();
    }

    public ObjectCache getCache() {
        return cache;
    }
}

public class TransactionExecutor {
    private final List<Transaction> queue = new ArrayList<>();

    public TransactionExecutor() {
    }

    public TransactionResult execute(Transaction transaction) {
        if (transaction == null) {
            return TransactionResult.failure("transaction is null");
        }
        if (transaction.getSender() == null || transaction.getSender().isBlank()) {
            return TransactionResult.failure("missing sender");
        }
        if (transaction.getCommands() == null || transaction.getCommands().isEmpty()) {
            return TransactionResult.failure("transaction has no commands");
        }
        queue.add(transaction);
        return TransactionResult.success();
    }

    public void reset() {
        queue.clear();
    }
}

class Transaction {
    private List<Map<String, Object>> commands;
    private String sender;
    private BigInteger gasPrice;
    private BigInteger gasBudget;

    public Transaction() {
        this.commands = new ArrayList<>();
    }

    public List<Map<String, Object>> getCommands() {
        return commands;
    }

    public String getSender() {
        return sender;
    }

    public void setSender(String sender) {
        this.sender = sender;
    }

    public BigInteger getGasPrice() {
        return gasPrice;
    }

    public void setGasPrice(BigInteger gasPrice) {
        this.gasPrice = gasPrice;
    }

    public BigInteger getGasBudget() {
        return gasBudget;
    }

    public void setGasBudget(BigInteger gasBudget) {
        this.gasBudget = gasBudget;
    }
}
