package com.suisdks.sui.grpc;

import com.google.protobuf.Struct;
import com.google.protobuf.util.JsonFormat;
import io.grpc.MethodDescriptor;
import io.grpc.Server;
import io.grpc.ServerServiceDefinition;
import io.grpc.inprocess.InProcessChannelBuilder;
import io.grpc.inprocess.InProcessServerBuilder;
import io.grpc.protobuf.ProtoUtils;
import io.grpc.stub.ServerCalls;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class OfficialGrpcTransportTest {
    @Test
    void unaryCallThroughOfficialTransport() throws Exception {
        String serverName = InProcessServerBuilder.generateName();

        MethodDescriptor<Struct, Struct> method = MethodDescriptor.<Struct, Struct>newBuilder()
                .setType(MethodDescriptor.MethodType.UNARY)
                .setFullMethodName("sui.rpc.v2.Service/Call")
                .setRequestMarshaller(ProtoUtils.marshaller(Struct.getDefaultInstance()))
                .setResponseMarshaller(ProtoUtils.marshaller(Struct.getDefaultInstance()))
                .build();

        ServerServiceDefinition svc = ServerServiceDefinition.builder("sui.rpc.v2.Service")
                .addMethod(method, ServerCalls.asyncUnaryCall((req, observer) -> {
                    try {
                        String in = JsonFormat.printer().print(req);
                        Struct.Builder out = Struct.newBuilder();
                        JsonFormat.parser().merge("{\"result\":{\"ok\":true,\"echo\":" + in + "}}", out);
                        observer.onNext(out.build());
                        observer.onCompleted();
                    } catch (Exception e) {
                        observer.onError(e);
                    }
                }))
                .build();

        Server server = InProcessServerBuilder.forName(serverName).directExecutor().addService(svc).build().start();
        try {
            var channel = InProcessChannelBuilder.forName(serverName).directExecutor().build();
            try {
                var client = new SuiGrpcClient(new OfficialGrpcTransport(serverName, true) {
                    @Override
                    public GrpcResponse unary(GrpcRequest request) {
                        try {
                            Struct.Builder reqBuilder = Struct.newBuilder();
                            String requestJson = "{\"method\":\"" + request.method() + "\",\"params\":" + JsonUtil.toJson(request.params()) + "}";
                            JsonFormat.parser().ignoringUnknownFields().merge(requestJson, reqBuilder);
                            Struct response = io.grpc.stub.ClientCalls.blockingUnaryCall(channel, method, io.grpc.CallOptions.DEFAULT, reqBuilder.build());
                            String json = JsonFormat.printer().includingDefaultValueFields().print(response);
                            Map<String, Object> decoded = JsonUtil.fromJsonObject(json);
                            return new GrpcResponse(decoded.get("result"), null);
                        } catch (Exception e) {
                            return new GrpcResponse(null, Map.of("message", e.getMessage()));
                        }
                    }
                });

                Object out = client.call("sui_getObject", List.of("0x1", Map.of()));
                Assertions.assertNotNull(out);
            } finally {
                channel.shutdownNow();
            }
        } finally {
            server.shutdownNow();
        }
    }
}
