package com.suisdks.sui.transactions

import com.suisdks.sui.client.SuiClient

interface Plugin {
    val name: String
    
    suspend fun beforeTransaction(tx: Transaction, kind: TransactionKind) {}
    suspend fun afterTransaction(tx: Transaction, result: Any?, error: Throwable?) {}
    suspend fun build(tx: Transaction) {}
}

enum class TransactionKind {
    MOVE_CALL,
    PROGRAMMABLE_TRANSACTION
}

class NamedPackagesPlugin(
    private val packages: Map<String, String>
) : Plugin {
    override val name: String = "NamedPackagesPlugin"

    fun resolve(name: String): String = packages[name] ?: name

    override suspend fun build(tx: Transaction) {
        // Replace named packages in transaction
        tx.replacePackages(packages)
    }
}

class ValidatorPlugin(
    private val validator: (Transaction) -> Boolean
) : Plugin {
    override val name: String = "ValidatorPlugin"

    override suspend fun beforeTransaction(tx: Transaction, kind: TransactionKind) {
        if (!validator(tx)) {
            throw TransactionValidationException("Transaction validation failed")
        }
    }
}

class TransactionValidationException(message: String) : Exception(message)

class PluginManager {
    private val plugins = mutableListOf<Plugin>()

    fun register(plugin: Plugin) {
        plugins.add(plugin)
    }

    fun unregister(plugin: Plugin) {
        plugins.remove(plugin)
    }

    fun unregisterByName(name: String) {
        plugins.removeIf { it.name == name }
    }

    fun get(name: String): Plugin? = plugins.find { it.name == name }

    fun list(): List<String> = plugins.map { it.name }

    suspend fun beforeTransaction(tx: Transaction, kind: TransactionKind) {
        plugins.forEach { plugin ->
            plugin.beforeTransaction(tx, kind)
        }
    }

    suspend fun afterTransaction(tx: Transaction, result: Any?, error: Throwable?) {
        plugins.forEach { plugin ->
            plugin.afterTransaction(tx, result, error)
        }
    }

    suspend fun build(tx: Transaction) {
        plugins.forEach { plugin ->
            plugin.build(tx)
        }
    }
}

fun Transaction.replacePackages(packages: Map<String, String>) {
    // Implementation to replace package addresses in transaction
    // This is a placeholder that would be implemented based on Transaction internals
}
