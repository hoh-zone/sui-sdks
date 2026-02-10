package com.suisdks.sui.grpc

data class GrpcRequest(
    val method: String,
    val params: List<Any?> = emptyList(),
    val metadata: Map<String, String> = emptyMap(),
)
