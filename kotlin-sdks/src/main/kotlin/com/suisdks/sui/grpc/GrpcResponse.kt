package com.suisdks.sui.grpc

data class GrpcResponse(
    val result: Any? = null,
    val error: Any? = null,
) {
    fun hasError(): Boolean = error != null
}
