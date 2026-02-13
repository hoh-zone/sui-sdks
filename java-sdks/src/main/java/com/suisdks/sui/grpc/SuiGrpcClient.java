package com.suisdks.sui.grpc;

import java.util.List;
import java.util.Map;
import java.util.Arrays;
import java.util.Objects;
import java.util.stream.Collectors;

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

    public Object dryRunTransactionBlock(String txBytesBase64) {
        return call("sui_dryRunTransactionBlock", List.of(txBytesBase64));
    }

    public Object devInspectTransactionBlock(String sender, Object txData) {
        return call("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Object getBalance(String owner, String coinType) {
        return call("suix_getBalance", List.of(owner, coinType));
    }

    public Object getAllBalances(String owner) {
        return call("suix_getAllBalances", List.of(owner));
    }

    public Object getAllCoins(String owner, String cursor, long limit) {
        return call("suix_getAllCoins", List.of(owner, cursor, limit));
    }

    public Object getCoinMetadata(String coinType) {
        return call("suix_getCoinMetadata", List.of(coinType));
    }

    public Object getCurrentSystemState() {
        return call("suix_getLatestSuiSystemState", List.of());
    }

    public Object getChainIdentifier() {
        return call("sui_getChainIdentifier", List.of());
    }

    public Object getReferenceGasPrice() {
        return call("suix_getReferenceGasPrice", List.of());
    }

    public Object getOwnedObjects(String owner, Map<String, Object> query, Object cursor, long limit) {
        return call("suix_getOwnedObjects", List.of(owner, query, cursor, limit));
    }

    public Object getDynamicFields(String parentId, Object cursor, long limit) {
        return call("suix_getDynamicFields", List.of(parentId, cursor, limit));
    }

    public Object getDynamicFieldObject(String parentId, Map<String, Object> name) {
        return call("suix_getDynamicFieldObject", List.of(parentId, name));
    }

    public Object getTransactionBlock(String digest, Map<String, Object> options) {
        return call("sui_getTransactionBlock", List.of(digest, options));
    }

    public Object multiGetTransactionBlocks(String[] digests, Map<String, Object> options) {
        List<String> digestList = Arrays.stream(digests).filter(Objects::nonNull).collect(Collectors.toList());
        return call("sui_multiGetTransactionBlocks", List.of(digestList, options));
    }

    public Object queryTransactionBlocks(Object query, Object cursor, long limit, boolean descendingOrder) {
        return call("suix_queryTransactionBlocks", List.of(query, cursor, limit, descendingOrder));
    }

    public Object getCheckpoint(String checkpointId) {
        return call("sui_getCheckpoint", List.of(checkpointId));
    }

    public Object getCheckpoints(Object cursor, long limit, boolean descendingOrder) {
        return call("sui_getCheckpoints", List.of(cursor, limit, descendingOrder));
    }

    public Object getCommitteeInfo(String epoch) {
        return call("suix_getCommitteeInfo", List.of(epoch));
    }

    public Object getProtocolConfig(String version) {
        return call("sui_getProtocolConfig", List.of(version));
    }

    public Object resolveNameServiceAddress(String name) {
        return call("suix_resolveNameServiceAddress", List.of(name));
    }

    public Object resolveNameServiceNames(String address, Object cursor, Long limit) {
        return call("suix_resolveNameServiceNames", List.of(address, cursor, limit));
    }

    public Object getEventsByTransaction(String txDigest) {
        return call("sui_getEvents", List.of(txDigest));
    }

    @Override
    public void close() {
        transport.close();
    }
}
