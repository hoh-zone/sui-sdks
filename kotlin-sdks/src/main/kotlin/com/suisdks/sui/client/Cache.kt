package com.suisdks.sui.client

import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import java.util.concurrent.TimeUnit

interface Cache {
    suspend fun <T> get(key: String): T?
    suspend fun <T> set(key: String, value: T, ttl: Long = TimeUnit.MINUTES.toMillis(5))
    suspend fun delete(key: String)
    suspend fun clear()
}

class InMemoryCache(
    private val maxSize: Int = 1000
) : Cache {
    private val mutex = Mutex()
    private val cache = LinkedHashMap<String, CacheEntry>(16, 0.75f, true)

    data class CacheEntry(
        val value: Any,
        val expiresAt: Long
    ) {
        fun isExpired(): Boolean = System.currentTimeMillis() > expiresAt
    }

    override suspend fun <T> get(key: String): T? = mutex.withLock {
        val entry = cache[key] ?: return null
        if (entry.isExpired()) {
            cache.remove(key)
            return null
        }
        @Suppress("UNCHECKED_CAST")
        return entry.value as T
    }

    override suspend fun <T> set(key: String, value: T, ttl: Long) = mutex.withLock {
        val expiresAt = System.currentTimeMillis() + ttl
        cache[key] = CacheEntry(value, expiresAt)
        
        while (cache.size > maxSize) {
            val iterator = cache.keys.iterator()
            if (iterator.hasNext()) {
                iterator.next()
                iterator.remove()
            }
        }
    }

    override suspend fun delete(key: String) = mutex.withLock {
        cache.remove(key)
    }

    override suspend fun clear() = mutex.withLock {
        cache.clear()
    }
}

class ScopedCache(
    private val parent: Cache,
    private val scope: String
) : Cache {
    private fun scopedKey(key: String) = "$scope:$key"

    override suspend fun <T> get(key: String): T? = parent.get(scopedKey(key))

    override suspend fun <T> set(key: String, value: T, ttl: Long) = 
        parent.set(scopedKey(key), value, ttl)

    override suspend fun delete(key: String) = parent.delete(scopedKey(key))

    override suspend fun clear() {
        // Cannot clear scope efficiently in generic cache
    }
}

class ClientCache(
    private val cache: Cache = InMemoryCache()
) {
    private val scopes = mutableMapOf<String, ScopedCache>()
    private val mutex = Mutex()

    suspend fun <T> get(key: String): T? = cache.get(key)

    suspend fun <T> set(key: String, value: T, ttl: Long = TimeUnit.MINUTES.toMillis(5)) = 
        cache.set(key, value, ttl)

    suspend fun delete(key: String) = cache.delete(key)

    suspend fun clear() = cache.clear()

    suspend fun scope(name: String): ScopedCache = mutex.withLock {
        scopes.getOrPut(name) { ScopedCache(cache, name) }
    }
}
