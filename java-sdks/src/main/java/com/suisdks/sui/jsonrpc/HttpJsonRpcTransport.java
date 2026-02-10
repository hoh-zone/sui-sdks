package com.suisdks.sui.jsonrpc;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.lang.reflect.Type;
import java.net.HttpURLConnection;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public final class HttpJsonRpcTransport implements JsonRpcTransport {
    private static final Gson GSON = new Gson();
    private static final Type MAP_TYPE = new TypeToken<Map<String, Object>>() {
    }.getType();

    private final String endpoint;

    public HttpJsonRpcTransport(String endpoint) {
        this.endpoint = endpoint;
    }

    @Override
    public Map<String, Object> request(String method, List<Object> params) {
        try {
            URL u = new URL(endpoint);
            HttpURLConnection conn = (HttpURLConnection) u.openConnection();
            conn.setRequestMethod("POST");
            conn.setRequestProperty("Content-Type", "application/json");
            conn.setDoOutput(true);

            Map<String, Object> payload = new HashMap<>();
            payload.put("jsonrpc", "2.0");
            payload.put("id", 1);
            payload.put("method", method);
            payload.put("params", params == null ? List.of() : params);

            try (OutputStream os = conn.getOutputStream()) {
                os.write(GSON.toJson(payload).getBytes(StandardCharsets.UTF_8));
            }

            try (InputStreamReader reader = new InputStreamReader(conn.getInputStream(), StandardCharsets.UTF_8)) {
                return GSON.fromJson(reader, MAP_TYPE);
            }
        } catch (Exception e) {
            throw new IllegalStateException("JSON-RPC request failed: " + method, e);
        }
    }
}
