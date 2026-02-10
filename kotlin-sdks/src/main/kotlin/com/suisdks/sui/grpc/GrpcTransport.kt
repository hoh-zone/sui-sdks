package com.suisdks.sui.grpc

interface GrpcTransport : AutoCloseable {
    fun unary(request: GrpcRequest): GrpcResponse
    override fun close() {}
}
