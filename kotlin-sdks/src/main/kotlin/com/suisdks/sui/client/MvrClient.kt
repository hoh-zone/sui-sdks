package com.suisdks.sui.client

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

class MvrClient(
    private val url: String,
    private val cache: MutableMap<String, CachedResponse> = mutableMapOf()
) {
    private val json = Json { ignoreUnknownKeys = true }
    private val httpClient = HttpClient.newBuilder().build()

    @Serializable
    data class ResolvePackageRequest(val packageId: String)

    @Serializable
    data class ResolvePackageResponse(val packageId: String)

    @Serializable
    data class ResolveTypeRequest(val type: String)

    @Serializable
    data class ResolveTypeResponse(val type: String)

    @Serializable
    data class ResolveRequest(
        val packages: List<String>? = null,
        val types: List<String>? = null
    )

    @Serializable
    data class ResolveResponse(
        val packages: Map<String, PackageInfo>? = null,
        val types: Map<String, TypeInfo>? = null
    )

    @Serializable
    data class PackageInfo(val packageId: String)

    @Serializable
    data class TypeInfo(val type: String)

    data class CachedResponse(
        val data: Any,
        val timestamp: Long,
        val ttl: Long = 300_000L
    ) {
        fun isExpired(): Boolean = System.currentTimeMillis() - timestamp > ttl
    }

    suspend fun resolvePackage(pkg: String): Result<String> = withContext(Dispatchers.IO) {
        val cacheKey = "mvr.package.$pkg"
        
        cache[cacheKey]?.let { cached ->
            if (!cached.isExpired()) {
                return@withContext Result.success((cached.data as ResolvePackageResponse).packageId)
            }
        }

        val request = HttpRequest.newBuilder()
            .uri(URI.create("$url/v1/packages/$pkg"))
            .header("Content-Type", "application/json")
            .GET()
            .build()

        try {
            val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
            if (response.statusCode() == 200) {
                val resp = json.decodeFromString<ResolvePackageResponse>(response.body())
                cache[cacheKey] = CachedResponse(resp)
                Result.success(resp.packageId)
            } else {
                Result.failure(Exception("MVR request failed: ${response.statusCode()}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun resolveType(typeStr: String): Result<String> = withContext(Dispatchers.IO) {
        val cacheKey = "mvr.type.$typeStr"
        
        cache[cacheKey]?.let { cached ->
            if (!cached.isExpired()) {
                return@withContext Result.success((cached.data as ResolveTypeResponse).type)
            }
        }

        val request = HttpRequest.newBuilder()
            .uri(URI.create("$url/v1/types"))
            .header("Content-Type", "application/json")
            .POST(HttpRequest.BodyPublishers.ofString(json.encodeToString(ResolveTypeRequest(typeStr))))
            .build()

        try {
            val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
            if (response.statusCode() == 200) {
                val resp = json.decodeFromString<ResolveTypeResponse>(response.body())
                cache[cacheKey] = CachedResponse(resp)
                Result.success(resp.type)
            } else {
                Result.failure(Exception("MVR request failed: ${response.statusCode()}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun resolve(pkgs: List<String>? = null, types: List<String>? = null): Result<ResolveResponse> = 
        withContext(Dispatchers.IO) {
        val request = HttpRequest.newBuilder()
            .uri(URI.create("$url/v1/resolve"))
            .header("Content-Type", "application/json")
            .POST(HttpRequest.BodyPublishers.ofString(
                json.encodeToString(ResolveRequest(pkgs, types))
            ))
            .build()

        try {
            val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
            if (response.statusCode() == 200) {
                Result.success(json.decodeFromString<ResolveResponse>(response.body()))
            } else {
                Result.failure(Exception("MVR request failed: ${response.statusCode()}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    companion object {
        val MAINNET_URL = "https://mainnet.mvr.mystenlabs.com"
        val TESTNET_URL = "https://testnet.mvr.mystenlabs.com"
        
        fun forMainnet(): MvrClient = MvrClient(MAINNET_URL)
        fun forTestnet(): MvrClient = MvrClient(TESTNET_URL)
    }
}
