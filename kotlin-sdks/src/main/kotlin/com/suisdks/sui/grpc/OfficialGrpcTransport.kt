package com.suisdks.sui.grpc

import com.google.protobuf.Struct
import com.google.protobuf.util.JsonFormat
import io.grpc.CallOptions
import io.grpc.Channel
import io.grpc.ClientInterceptors
import io.grpc.Grpc
import io.grpc.InsecureChannelCredentials
import io.grpc.ManagedChannel
import io.grpc.MethodDescriptor
import io.grpc.TlsChannelCredentials
import io.grpc.protobuf.ProtoUtils
import io.grpc.stub.ClientCalls
import io.grpc.stub.MetadataUtils
import io.grpc.Metadata

data class GrpcSecurityOptions(
    val defaultHeaders: Map<String, String> = emptyMap(),
    val bearerToken: String? = null,
)

data class GrpcTlsOptions(
    val authorityOverride: String? = null,
)

class OfficialGrpcTransport(
    target: String,
    plaintext: Boolean,
    private val security: GrpcSecurityOptions = GrpcSecurityOptions(),
    private val tls: GrpcTlsOptions = GrpcTlsOptions(),
) : GrpcTransport {
    companion object {
        private val METHOD: MethodDescriptor<Struct, Struct> = MethodDescriptor.newBuilder<Struct, Struct>()
            .setType(MethodDescriptor.MethodType.UNARY)
            .setFullMethodName("sui.rpc.v2.Service/Call")
            .setRequestMarshaller(ProtoUtils.marshaller(Struct.getDefaultInstance()))
            .setResponseMarshaller(ProtoUtils.marshaller(Struct.getDefaultInstance()))
            .build()
    }

    private val channel: ManagedChannel = Grpc.newChannelBuilder(
        target,
        if (plaintext) {
            InsecureChannelCredentials.create()
        } else {
            TlsChannelCredentials.create()
        },
    ).apply {
        tls.authorityOverride?.let { overrideAuthority(it) }
    }.build()

    override fun unary(request: GrpcRequest): GrpcResponse {
        return try {
            val reqBuilder = Struct.newBuilder()
            val requestJson = "{\"method\":\"${request.method}\",\"params\":${JsonUtil.toJson(request.params)}}"
            JsonFormat.parser().ignoringUnknownFields().merge(requestJson, reqBuilder)

            val callChannel = withRequestHeaders(channel, request)
            val response = ClientCalls.blockingUnaryCall(callChannel, METHOD, CallOptions.DEFAULT, reqBuilder.build())
            val json = JsonFormat.printer().includingDefaultValueFields().print(response)
            val decoded = JsonUtil.fromJsonObject(json)
            if (decoded.containsKey("error") && decoded["error"] != null) {
                GrpcResponse(error = decoded["error"])
            } else {
                GrpcResponse(result = decoded["result"] ?: decoded)
            }
        } catch (e: Exception) {
            GrpcResponse(error = mapOf("message" to (e.message ?: "grpc error")))
        }
    }

    override fun close() {
        channel.shutdownNow()
    }

    private fun withRequestHeaders(base: Channel, request: GrpcRequest): Channel {
        val allHeaders = LinkedHashMap<String, String>()
        allHeaders.putAll(security.defaultHeaders)
        if (security.bearerToken != null) {
            allHeaders["authorization"] = "Bearer ${security.bearerToken}"
        }
        allHeaders.putAll(request.metadata)
        if (allHeaders.isEmpty()) {
            return base
        }

        val metadata = Metadata()
        allHeaders.forEach { (k, v) ->
            metadata.put(Metadata.Key.of(k, Metadata.ASCII_STRING_MARSHALLER), v)
        }
        return ClientInterceptors.intercept(base, MetadataUtils.newAttachHeadersInterceptor(metadata))
    }
}
