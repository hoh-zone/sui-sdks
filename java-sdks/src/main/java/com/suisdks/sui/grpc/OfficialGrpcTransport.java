package com.suisdks.sui.grpc;

import com.google.protobuf.Struct;
import com.google.protobuf.util.JsonFormat;
import io.grpc.CallOptions;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.Metadata;
import io.grpc.MethodDescriptor;
import io.grpc.stub.ClientCalls;
import java.util.Map;

public final class OfficialGrpcTransport implements GrpcTransport {
    private static final MethodDescriptor<Struct, Struct> METHOD = MethodDescriptor.<Struct, Struct>newBuilder()
            .setType(MethodDescriptor.MethodType.UNARY)
            .setFullMethodName("sui.rpc.v2.Service/Call")
            .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(Struct.getDefaultInstance()))
            .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(Struct.getDefaultInstance()))
            .build();

    private final ManagedChannel channel;

    public OfficialGrpcTransport(String target, boolean plaintext) {
        ManagedChannelBuilder<?> builder = ManagedChannelBuilder.forTarget(target);
        if (plaintext) {
            builder.usePlaintext();
        }
        this.channel = builder.build();
    }

    @Override
    public GrpcResponse unary(GrpcRequest request) {
        try {
            Struct.Builder reqBuilder = Struct.newBuilder();
            String requestJson = "{\"method\":\"" + request.method() + "\",\"params\":" + JsonUtil.toJson(request.params()) + "}";
            JsonFormat.parser().ignoringUnknownFields().merge(requestJson, reqBuilder);

            Metadata metadata = new Metadata();
            for (Map.Entry<String, String> e : request.metadata().entrySet()) {
                Metadata.Key<String> key = Metadata.Key.of(e.getKey(), Metadata.ASCII_STRING_MARSHALLER);
                metadata.put(key, e.getValue());
            }

            Struct response = ClientCalls.blockingUnaryCall(channel, METHOD, CallOptions.DEFAULT, reqBuilder.build());
            String json = JsonFormat.printer().includingDefaultValueFields().print(response);
            Map<String, Object> decoded = JsonUtil.fromJsonObject(json);
            if (decoded.containsKey("error") && decoded.get("error") != null) {
                return new GrpcResponse(null, decoded.get("error"));
            }
            return new GrpcResponse(decoded.getOrDefault("result", decoded), null);
        } catch (Exception e) {
            return new GrpcResponse(null, Map.of("message", e.getMessage()));
        }
    }

    @Override
    public void close() {
        channel.shutdownNow();
    }
}
