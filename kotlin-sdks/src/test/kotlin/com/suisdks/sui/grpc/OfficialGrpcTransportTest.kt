package com.suisdks.sui.grpc

import com.google.protobuf.Struct
import com.google.protobuf.util.JsonFormat
import io.grpc.CallOptions
import io.grpc.MethodDescriptor
import io.grpc.Server
import io.grpc.ServerServiceDefinition
import io.grpc.inprocess.InProcessChannelBuilder
import io.grpc.inprocess.InProcessServerBuilder
import io.grpc.protobuf.ProtoUtils
import io.grpc.stub.ClientCalls
import io.grpc.stub.ServerCalls
import kotlin.test.Test
import kotlin.test.assertNotNull

class OfficialGrpcTransportTest {
    @Test
    fun unaryCallThroughOfficialTransport() {
        val serverName = InProcessServerBuilder.generateName()

        val method = MethodDescriptor.newBuilder<Struct, Struct>()
            .setType(MethodDescriptor.MethodType.UNARY)
            .setFullMethodName("sui.rpc.v2.Service/Call")
            .setRequestMarshaller(ProtoUtils.marshaller(Struct.getDefaultInstance()))
            .setResponseMarshaller(ProtoUtils.marshaller(Struct.getDefaultInstance()))
            .build()

        val service = ServerServiceDefinition.builder("sui.rpc.v2.Service")
            .addMethod(method, ServerCalls.asyncUnaryCall<Struct, Struct> { req, observer ->
                try {
                    val inJson = JsonFormat.printer().print(req)
                    val out = Struct.newBuilder()
                    JsonFormat.parser().merge("{\"result\":{\"ok\":true,\"echo\":$inJson}}", out)
                    observer.onNext(out.build())
                    observer.onCompleted()
                } catch (e: Exception) {
                    observer.onError(e)
                }
            })
            .build()

        val server: Server = InProcessServerBuilder.forName(serverName)
            .directExecutor()
            .addService(service)
            .build()
            .start()

        try {
            val channel = InProcessChannelBuilder.forName(serverName).directExecutor().build()
            try {
                val client = SuiGrpcClient(object : GrpcTransport {
                    override fun unary(request: GrpcRequest): GrpcResponse {
                        return try {
                            val reqBuilder = Struct.newBuilder()
                            val requestJson = "{\"method\":\"${request.method}\",\"params\":${JsonUtil.toJson(request.params)}}"
                            JsonFormat.parser().ignoringUnknownFields().merge(requestJson, reqBuilder)
                            val response = ClientCalls.blockingUnaryCall(channel, method, CallOptions.DEFAULT, reqBuilder.build())
                            val json = JsonFormat.printer().includingDefaultValueFields().print(response)
                            val decoded = JsonUtil.fromJsonObject(json)
                            GrpcResponse(result = decoded["result"])
                        } catch (e: Exception) {
                            GrpcResponse(error = mapOf("message" to (e.message ?: "error")))
                        }
                    }
                })

                val out = client.call("sui_getObject", listOf("0x1", emptyMap<String, Any?>()))
                assertNotNull(out)
                client.close()
            } finally {
                channel.shutdownNow()
            }
        } finally {
            server.shutdownNow()
        }
    }
}
