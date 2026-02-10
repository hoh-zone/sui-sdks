package com.suisdks.sui.grpc;

public record GrpcResponse(Object result, Object error) {
    public boolean hasError() {
        return error != null;
    }
}
