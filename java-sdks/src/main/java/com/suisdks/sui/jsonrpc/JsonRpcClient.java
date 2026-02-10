package com.suisdks.sui.jsonrpc;

import java.util.List;
import java.util.Map;

public final class JsonRpcClient {
    private final JsonRpcTransport transport;

    public JsonRpcClient(JsonRpcTransport transport) {
        this.transport = transport;
    }

    public static JsonRpcClient fromNetwork(String network) {
        String endpoint = switch (network) {
            case "mainnet" -> "https://fullnode.mainnet.sui.io:443";
            case "testnet" -> "https://fullnode.testnet.sui.io:443";
            case "devnet" -> "https://fullnode.devnet.sui.io:443";
            default -> throw new IllegalArgumentException("unsupported network: " + network);
        };
        return new JsonRpcClient(new HttpJsonRpcTransport(endpoint));
    }

    public Map<String, Object> call(String method, List<Object> params) {
        Map<String, Object> payload = transport.request(method, params);
        if (payload.containsKey("error") && payload.get("error") != null) {
            throw new IllegalStateException("jsonrpc error: " + payload.get("error"));
        }
        if (payload.containsKey("result")) {
            return Map.of("result", payload.get("result"));
        }
        return payload;
    }

    public Map<String, Object> getObject(String objectId, Map<String, Object> options) {
        return call("sui_getObject", List.of(objectId, options));
    }

    public Map<String, Object> executeTransactionBlock(String txBytes, List<String> signatures, Map<String, Object> options) {
        return call("sui_executeTransactionBlock", List.of(txBytes, signatures, options));
    }
}
