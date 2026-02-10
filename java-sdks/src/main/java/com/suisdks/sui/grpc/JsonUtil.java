package com.suisdks.sui.grpc;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;
import java.lang.reflect.Type;
import java.util.Map;

final class JsonUtil {
    private static final Gson GSON = new Gson();
    private static final Type MAP_TYPE = new TypeToken<Map<String, Object>>() {
    }.getType();

    private JsonUtil() {
    }

    static String toJson(Object value) {
        return GSON.toJson(value);
    }

    static Map<String, Object> fromJsonObject(String json) {
        return GSON.fromJson(json, MAP_TYPE);
    }
}
