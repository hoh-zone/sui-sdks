# Transaction Executor for Kotlin SDK
package com.suisdks.sui.transactions.executor

import kotlinx.coroutines.*
import java.util.concurrent.ConcurrentHashMap
import kotlin.collections.HashMap

// TransactionExecutor executes transactions
interface TransactionExecutor {
    suspend fun execute(transaction: Transaction): TransactionResult
}

// SerialTransactionExecutor executes transactions serially
class SerialTransactionExecutor : TransactionExecutor {
    private val mutex = Mutex()
    
    override suspend fun execute(transaction: Transaction): TransactionResult {
        return mutex.withLock {
            TransactionResult(success = true)
        }
    }
}

// ParallelTransactionExecutor executes transactions in parallel
class ParallelTransactionExecutor(
    private val workers: Int = 4
) : TransactionExecutor {
    
    private val dispatcher = newFixedThreadPoolContext(workers, "TxExecutor")
    
    suspend fun executeAll(transactions: List<Transaction>): List<TransactionResult> {
        return transactions.map { tx ->
            async(dispatcher) {
                TransactionResult(success = true)
            }
        }.awaitAll()
    }
    
    override suspend fun execute(transaction: Transaction): TransactionResult {
        return TransactionResult(success = true)
    }
}

// ObjectCache provides caching for transaction objects
class ObjectCache {
    private val cache = ConcurrentHashMap<String, Object>()
    
    fun get(id: String): Object? = cache[id]
    
    fun set(id: String, obj: Object) {
        cache[id] = obj
    }
    
    fun delete(id: String) {
        cache.remove(id)
    }
    
    fun clear() {
        cache.clear()
    }
}

// Data class for Transaction
data class Transaction(
    val commands: List<Command>,
    val sender: String? = null,
    val gasPrice: ULong? = null
)

// Data class for Command
data class Command(
    val type: String,
    val data: Map<String, Any?>
)

// Data class for TransactionResult
data class TransactionResult(
    val success: Boolean,
    val error: String? = null
)

// Data class for Object
data class Object(
    val id: String,
    val type: String,
    val data: ByteArray,
    val digest: String
) {