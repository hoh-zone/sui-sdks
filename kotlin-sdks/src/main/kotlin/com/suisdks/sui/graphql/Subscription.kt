package com.suisdks.sui.graphql

import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.serialization.json.JsonObject

class Subscription<T>(
    private val query: String,
    private val variables: Map<String, Any>,
    private val mapper: (JsonObject) -> T
) {
    private val channel = Channel<T>(Channel.UNLIMITED)
    private var active = false

    suspend fun start(): Flow<T> {
        active = true
        return channel.receiveAsFlow()
    }

    suspend fun next(): T? {
        return try {
            channel.receive()
        } catch (e: Exception) {
            null
        }
    }

    fun stop() {
        active = false
        channel.close()
    }

    fun isActive(): Boolean = active

    internal suspend fun emit(data: JsonObject) {
        if (active) {
            channel.send(mapper(data))
        }
    }
}

class SubscriptionManager {
    private val subscriptions = mutableMapOf<String, Subscription<*>>()

    fun <T> create(
        name: String,
        query: String,
        variables: Map<String, Any>,
        mapper: (JsonObject) -> T
    ): Subscription<T> {
        val subscription = Subscription(query, variables, mapper)
        subscriptions[name] = subscription
        return subscription
    }

    fun get(name: String): Subscription<*>? = subscriptions[name]

    fun stop(name: String) {
        subscriptions[name]?.stop()
        subscriptions.remove(name)
    }

    fun stopAll() {
        subscriptions.values.forEach { it.stop() }
        subscriptions.clear()
    }

    fun listActive(): List<String> = 
        subscriptions.filter { it.value.isActive() }.keys.toList()
}

inline fun <reified T> subscription(
    query: String,
    variables: Map<String, Any>,
    crossinline mapper: (JsonObject) -> T
): Subscription<T> {
    return Subscription(query, variables) { mapper(it) }
}
