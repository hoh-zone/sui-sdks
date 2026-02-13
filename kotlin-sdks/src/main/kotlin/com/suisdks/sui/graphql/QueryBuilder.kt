package com.suisdks.sui.graphql

import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive

class QueryBuilder {
    private val selections = mutableListOf<String>()
    private val arguments = mutableMapOf<String, Any>()
    private val fragments = mutableMapOf<String, String>()

    fun select(vararg fields: String): QueryBuilder {
        selections.addAll(fields)
        return this
    }

    fun arg(name: String, value: Any): QueryBuilder {
        arguments[name] = value
        return this
    }

    fun fragment(name: String, definition: String): QueryBuilder {
        fragments[name] = definition
        return this
    }

    fun build(operationName: String? = null): String {
        val argsPart = if (arguments.isNotEmpty()) {
            "(" + arguments.entries.joinToString(", ") { (name, value) ->
                val serialized = when (value) {
                    is String -> "\"$value\""
                    is Number -> value.toString()
                    is Boolean -> value.toString()
                    else -> value.toString()
                }
                "$name: $serialized"
            } + ")"
        } else ""

        val selectionsPart = selections.joinToString("\n    ")
        
        val fragmentsPart = if (fragments.isNotEmpty()) {
            "\n" + fragments.entries.joinToString("\n") { (name, def) ->
                "fragment $name on ${def}"
            }
        } else ""

        return if (operationName != null) {
            "query $operationName$argsPart {\n    $selectionsPart\n}$fragmentsPart"
        } else {
            "query$argsPart {\n    $selectionsPart\n}$fragmentsPart"
        }
    }
}

class TypedQueryBuilder<T> {
    private val queryBuilder = QueryBuilder()
    private var resultMapper: ((JsonObject) -> T)? = null

    fun select(vararg fields: String): TypedQueryBuilder<T> {
        queryBuilder.select(*fields)
        return this
    }

    fun arg(name: String, value: Any): TypedQueryBuilder<T> {
        queryBuilder.arg(name, value)
        return this
    }

    fun map(mapper: (JsonObject) -> T): TypedQueryBuilder<T> {
        this.resultMapper = mapper
        return this
    }

    fun buildQuery(operationName: String? = null): String {
        return queryBuilder.build(operationName)
    }

    fun mapResult(result: JsonObject): T {
        return resultMapper?.invoke(result) ?: throw IllegalStateException("Result mapper not set")
    }
}

inline fun <reified T> typedQuery(block: TypedQueryBuilder<T>.() -> Unit): TypedQueryBuilder<T> {
    return TypedQueryBuilder<T>().apply(block)
}

fun query(block: QueryBuilder.() -> Unit): String {
    return QueryBuilder().apply(block).build()
}

object NamedQueries {
    private val queries = mutableMapOf<String, String>()

    fun register(name: String, query: String) {
        queries[name] = query
    }

    fun get(name: String): String? = queries[name]

    fun list(): List<String> = queries.keys.toList()

    fun unregister(name: String) {
        queries.remove(name)
    }

    fun clear() {
        queries.clear()
    }
}

fun coinQuery(owner: String, coinType: String? = null): String = query {
    arg("owner", owner)
    coinType?.let { arg("coinType", it) }
    select(
        "coins { data { coinObjectId, balance }, hasNextPage, cursor }"
    )
}

fun objectQuery(id: String): String = query {
    arg("id", id)
    select("object { objectId, version, digest, owner, content }")
}

fun transactionBlockQuery(digest: String): String = query {
    arg("digest", digest)
    select(
        "transactionBlock { digest, effects { status, gasUsed }, rawTransaction }"
    )
}
