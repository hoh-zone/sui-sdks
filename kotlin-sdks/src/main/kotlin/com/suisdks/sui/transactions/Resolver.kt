package com.suisdks.sui.transactions

import java.util.concurrent.CompletableFuture

data class ResolveContext(
    val transaction: Transaction,
    val unresolvedInputs: MutableList<Map<String, Any?>> = mutableListOf(),
)

fun interface ResolvePlugin {
    fun apply(context: ResolveContext)
}

class ResolverPluginError(
    val index: Int,
    val pluginName: String,
    val causeError: Throwable,
) : RuntimeException("resolver plugin failed at index $index ($pluginName): ${causeError.message}", causeError) {
    fun toMap(): Map<String, Any?> = mapOf(
        "error_type" to this::class.simpleName,
        "index" to index,
        "plugin_name" to pluginName,
        "cause_type" to causeError::class.simpleName,
        "cause_message" to causeError.message,
        "message" to message,
    )
}

class Resolver {
    private val plugins: MutableList<ResolvePlugin> = mutableListOf()

    fun addPlugin(plugin: ResolvePlugin) {
        plugins.add(plugin)
    }

    fun resolve(tx: Transaction): ResolveContext {
        val context = ResolveContext(transaction = tx)
        tx.data.inputs.forEach { input ->
            if (input["\$kind"] == "UnresolvedObject") {
                context.unresolvedInputs.add(input)
            }
        }
        plugins.forEachIndexed { index, plugin ->
            val pluginName = plugin::class.simpleName ?: "anonymous"
            try {
                plugin.apply(context)
            } catch (e: Throwable) {
                throw ResolverPluginError(index = index, pluginName = pluginName, causeError = e)
            }
        }
        return context
    }
}

fun interface AsyncResolvePlugin {
    fun apply(context: ResolveContext): CompletableFuture<Unit>
}

class AsyncResolver {
    private val plugins: MutableList<AsyncResolvePlugin> = mutableListOf()

    fun addPlugin(plugin: AsyncResolvePlugin) {
        plugins.add(plugin)
    }

    fun resolve(tx: Transaction): CompletableFuture<ResolveContext> {
        val context = ResolveContext(transaction = tx)
        tx.data.inputs.forEach { input ->
            if (input["\$kind"] == "UnresolvedObject") {
                context.unresolvedInputs.add(input)
            }
        }

        var chain = CompletableFuture.completedFuture(context)
        plugins.forEachIndexed { index, plugin ->
            chain = chain.thenCompose { ctx ->
                val pluginName = plugin::class.simpleName ?: "anonymous"
                try {
                    plugin.apply(ctx).handle { _, err ->
                        if (err != null) {
                            throw ResolverPluginError(
                                index = index,
                                pluginName = pluginName,
                                causeError = err.cause ?: err,
                            )
                        }
                        ctx
                    }
                } catch (e: Throwable) {
                    throw ResolverPluginError(index = index, pluginName = pluginName, causeError = e)
                }
            }
        }
        return chain
    }
}
