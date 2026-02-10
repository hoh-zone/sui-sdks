package com.suisdks.sui.grpc

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken

internal object JsonUtil {
    private val gson = Gson()

    fun toJson(value: Any?): String = gson.toJson(value)

    fun fromJsonObject(json: String): Map<String, Any?> {
        val type = object : TypeToken<Map<String, Any?>>() {}.type
        return gson.fromJson(json, type)
    }
}
