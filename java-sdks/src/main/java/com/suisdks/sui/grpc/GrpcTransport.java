package com.suisdks.sui.grpc;

public interface GrpcTransport extends AutoCloseable {
    GrpcResponse unary(GrpcRequest request);

    @Override
    default void close() {
    }
}
