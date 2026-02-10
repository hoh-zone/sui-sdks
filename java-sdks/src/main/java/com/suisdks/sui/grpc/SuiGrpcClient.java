package com.suisdks.sui.grpc;

import java.util.List;
import java.util.Map;

public final class SuiGrpcClient implements AutoCloseable {
    private final GrpcTransport transport;

    public SuiGrpcClient(GrpcTransport transport) {
        this.transport = transport;
    }

    public static SuiGrpcClient fromOfficialGrpc(String target, boolean plaintext) {
        return new SuiGrpcClient(new OfficialGrpcTransport(target, plaintext));
    }

    public Object call(String method, List<Object> params) {
        GrpcResponse r = transport.unary(new GrpcRequest(method, params));
        if (r.hasError()) {
            throw new IllegalStateException("gRPC error: " + r.error());
        }
        return r.result();
    }

    public Object getObject(String objectId, Map<String, Object> options) {
        return call("sui_getObject", List.of(objectId, options));
    }

    public Object executeTransactionBlock(String txBytesBase64, List<String> signatures, Map<String, Object> options) {
        return call("sui_executeTransactionBlock", List.of(txBytesBase64, signatures, options));
    }

    @Override
    public void close() {
        transport.close();
    }
}
