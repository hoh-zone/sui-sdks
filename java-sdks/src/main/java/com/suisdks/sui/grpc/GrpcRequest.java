package com.suisdks.sui.grpc;

import java.util.List;
import java.util.Map;

public record GrpcRequest(String method, List<Object> params, Map<String, String> metadata) {
    public GrpcRequest(String method, List<Object> params) {
        this(method, params, Map.of());
    }
}
