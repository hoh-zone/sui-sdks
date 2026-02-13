package com.suisdks.sui.jsonrpc;

import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;

public final class JsonRpcClient implements SuiRpcMethods {
    private final JsonRpcTransport transport;
    private final String endpoint;

    public JsonRpcClient(JsonRpcTransport transport) {
        this(transport, null);
    }

    public JsonRpcClient(JsonRpcTransport transport, String endpoint) {
        this.transport = transport;
        this.endpoint = endpoint;
    }

    public static JsonRpcClient fromNetwork(String network) {
        String endpoint = switch (network) {
            case "mainnet" -> "https://fullnode.mainnet.sui.io:443";
            case "testnet" -> "https://fullnode.testnet.sui.io:443";
            case "devnet" -> "https://fullnode.devnet.sui.io:443";
            default -> throw new IllegalArgumentException("unsupported network: " + network);
        };
        return new JsonRpcClient(new HttpJsonRpcTransport(endpoint), endpoint);
    }

    public String getEndpoint() {
        return endpoint;
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

    public Map<String, Object> callRpcInternal(String method, List<Object> params) {
        return call(method, params);
    }

    public JsonRpcTransport getTransport() {
        return transport;
    }

    private static List<Object> asList(String[] values) {
        if (values == null) {
            return List.of();
        }
        return Arrays.stream(values).filter(Objects::nonNull).collect(Collectors.toList());
    }

    // Additional TS-parity helpers:
    public Map<String, Object> getRpcApiVersion() {
        return call("rpc.discover", List.of());
    }

    public Map<String, Object> getTotalSupply(String coinType) {
        return call("suix_getTotalSupply", List.of(coinType));
    }

    public Map<String, Object> getMoveFunctionArgTypes(String packageId, String moduleName, String functionName) {
        return call("sui_getMoveFunctionArgTypes", List.of(packageId, moduleName, functionName));
    }

    public Map<String, Object> getNormalizedMoveModulesByPackage(String packageId) {
        return call("sui_getNormalizedMoveModulesByPackage", List.of(packageId));
    }

    public Map<String, Object> getNormalizedMoveModule(String packageId, String moduleName) {
        return call("sui_getNormalizedMoveModule", List.of(packageId, moduleName));
    }

    public Map<String, Object> getNormalizedMoveFunction(String packageId, String moduleName, String functionName) {
        return call("sui_getNormalizedMoveFunction", List.of(packageId, moduleName, functionName));
    }

    public Map<String, Object> getNormalizedMoveStruct(String packageId, String moduleName, String structName) {
        return call("sui_getNormalizedMoveStruct", List.of(packageId, moduleName, structName));
    }

    public Map<String, Object> getTransactionBlock(String digest, Map<String, Object> options) {
        return call("sui_getTransactionBlock", List.of(digest, options == null ? Map.of() : options));
    }

    public Map<String, Object> multiGetObjects(String[] objectIds, Map<String, Object> options) {
        return getObjects(objectIds, options);
    }

    public Map<String, Object> getCoins(String owner, String coinType, String cursor, long limit) {
        return call("suix_getCoins", List.of(owner, coinType, cursor, limit));
    }

    public Map<String, Object> multiGetTransactionBlocks(String[] digests, Map<String, Object> options) {
        return call("sui_multiGetTransactionBlocks", List.of(asList(digests), options == null ? Map.of() : options));
    }

    public Map<String, Object> queryTransactionBlocks(String query, Map<String, Object> cursor, long limit, boolean descending) {
        return call("suix_queryTransactionBlocks", List.of(query, cursor, limit, descending));
    }

    public Map<String, Object> getTotalTransactionBlocks() {
        return call("sui_getTotalTransactionBlocks", List.of());
    }

    public Map<String, Object> getLatestSuiSystemState() {
        return call("suix_getLatestSuiSystemState", List.of());
    }

    public Map<String, Object> signAndExecuteTransaction(String txBytes, String[] signatures, Map<String, Object> options) {
        return executeTransactionBlock(txBytes, signatures, options);
    }

    public Map<String, Object> waitForTransaction(String digest, Map<String, Object> options) {
        return waitForTransaction(digest, options, 60_000L, 2_000L);
    }

    public Map<String, Object> waitForTransaction(
            String digest,
            Map<String, Object> options,
            long timeoutMs,
            long pollIntervalMs
    ) {
        long deadline = System.currentTimeMillis() + timeoutMs;
        while (System.currentTimeMillis() < deadline) {
            try {
                return getTransactionBlock(digest, options);
            } catch (RuntimeException ignored) {
                try {
                    Thread.sleep(Math.max(1L, pollIntervalMs));
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    throw new IllegalStateException("Interrupted while waiting for transaction " + digest, e);
                }
            }
        }
        throw new IllegalStateException("Timed out waiting for transaction " + digest + " after " + timeoutMs + "ms");
    }

    public Map<String, Object> getDynamicFieldObject(String parentId, Map<String, Object> name) {
        return call("suix_getDynamicFieldObject", List.of(parentId, name));
    }

    public Map<String, Object> getEventsByTransaction(String txDigest) {
        return call("sui_getEvents", List.of(txDigest));
    }

    public Map<String, Object> getNetworkMetrics() {
        return call("suix_getNetworkMetrics", List.of());
    }

    public Map<String, Object> getAddressMetrics() {
        return call("suix_getLatestAddressMetrics", List.of());
    }

    public Map<String, Object> getEpochMetrics(Map<String, Object> cursor, long limit, boolean descending) {
        return call("suix_getEpochMetrics", List.of(cursor, limit, descending));
    }

    public Map<String, Object> getAllEpochAddressMetrics(Map<String, Object> cursor, long limit, boolean descending) {
        return call("suix_getAllEpochAddressMetrics", List.of(cursor, limit, descending));
    }

    public Map<String, Object> getEpochs(Map<String, Object> cursor, long limit, boolean descending) {
        return call("suix_getEpochs", List.of(cursor, limit, descending));
    }

    public Map<String, Object> getMoveCallMetrics() {
        return call("suix_getMoveCallMetrics", List.of());
    }

    public Map<String, Object> verifyZkLoginSignature(Map<String, Object> payload) {
        return call("sui_verifyZkLoginSignature", List.of(payload));
    }

    @Override
    public Map<String, Object> getObject(String objectId, Map<String, Object> options) {
        return call("sui_getObject", List.of(objectId, options == null ? Map.of() : options));
    }

    @Override
    public Map<String, Object> getObjects(String[] objectIds, Map<String, Object> options) {
        return call("sui_multiGetObjects", List.of(asList(objectIds), options == null ? Map.of() : options));
    }

    @Override
    public Map<String, Object> executeTransactionBlock(String txBytes, String[] signatures, Map<String, Object> options) {
        return call(
                "sui_executeTransactionBlock",
                List.of(txBytes, asList(signatures), options == null ? Map.of() : options)
        );
    }

    public Map<String, Object> executeTransactionBlock(String txBytes, List<String> signatures, Map<String, Object> options) {
        return call(
                "sui_executeTransactionBlock",
                List.of(txBytes, signatures == null ? List.of() : signatures, options == null ? Map.of() : options)
        );
    }

    @Override
    public Map<String, Object> devInspectTransactionBlock(String senderAddress, String txBytes, Map<String, Object> options) {
        return call("suix_devInspectTransactionBlock", List.of(senderAddress, txBytes, options == null ? Map.of() : options));
    }

    @Override
    public Map<String, Object> dryRunTransactionBlock(String txBytes, Map<String, Object> options) {
        return call("sui_dryRunTransactionBlock", List.of(txBytes));
    }

    @Override
    public Map<String, Object> getTransaction(String digest, Map<String, Object> options) {
        return call("sui_getTransactionBlock", List.of(digest, options == null ? Map.of() : options));
    }

    @Override
    public Map<String, Object> getTotalTransactionBlocks(String query, Map<String, Object> cursor, long limit) {
        return call("suix_queryTransactionBlocks", List.of(query, cursor, limit, false));
    }

    @Override
    public Map<String, Object> getBalance(String owner, String coinType) {
        return call("suix_getBalance", List.of(owner, coinType));
    }

    @Override
    public Map<String, Object> getAllBalances(String owner) {
        return call("suix_getAllBalances", List.of(owner));
    }

    @Override
    public Map<String, Object> getAllCoins(String owner, String cursor, long limit) {
        return call("suix_getAllCoins", List.of(owner, cursor, limit));
    }

    @Override
    public Map<String, Object> getCoinMetadata(String coinType) {
        return call("suix_getCoinMetadata", List.of(coinType));
    }

    @Override
    public Map<String, Object> getCurrentSystemState() {
        return call("suix_getLatestSuiSystemState", List.of());
    }

    @Override
    public Map<String, Object> getChainIdentifier() {
        return call("sui_getChainIdentifier", List.of());
    }

    @Override
    public Map<String, Object> getReferenceGasPrice() {
        return call("suix_getReferenceGasPrice", List.of());
    }

    @Override
    public Map<String, Object> getStakes(String owner) {
        return call("suix_getStakes", List.of(owner));
    }

    @Override
    public Map<String, Object> getStakesByIds(String[] stakeIds) {
        return call("suix_getStakesByIds", List.of(asList(stakeIds)));
    }

    @Override
    public Map<String, Object> getCurrentEpoch() {
        return call("suix_getCurrentEpoch", List.of());
    }

    @Override
    public Map<String, Object> getValidatorsApy() {
        return call("suix_getValidatorsApy", List.of());
    }

    @Override
    public Map<String, Object> getLatestSystemState() {
        return call("suix_getLatestSuiSystemState", List.of());
    }

    @Override
    public Map<String, Object> getCommitteeInfo(long epoch) {
        return call("suix_getCommitteeInfo", List.of(String.valueOf(epoch)));
    }

    @Override
    public Map<String, Object> getEvents(String query, Map<String, Object> cursor, long limit) {
        return call("suix_queryEvents", List.of(query, cursor, limit, false));
    }

    @Override
    public Map<String, Object> queryEvents(String query, Map<String, Object> cursor, long limit) {
        return call("suix_queryEvents", List.of(query, cursor, limit, false));
    }

    @Override
    public Map<String, Object> getDynamicFields(String parentId, Map<String, Object> cursor, long limit) {
        return call("suix_getDynamicFields", List.of(parentId, cursor, limit));
    }

    @Override
    public Map<String, Object> getOwnedObjects(String owner, Map<String, Object> options, Map<String, Object> cursor, long limit) {
        return call("suix_getOwnedObjects", List.of(owner, options == null ? Map.of() : options, cursor, limit));
    }

    @Override
    public Map<String, Object> listOwnedObjects(String owner, Map<String, Object> cursor, long limit) {
        return call("suix_getOwnedObjects", List.of(owner, Map.of(), cursor, limit));
    }

    @Override
    public Map<String, Object> getNormalizedMoveModules(String packageId) {
        return call("sui_getNormalizedMoveModulesByPackage", List.of(packageId));
    }

    @Override
    public Map<String, Object> getMoveFunction(String packageId, String moduleName, String functionName) {
        return call("sui_getNormalizedMoveFunction", List.of(packageId, moduleName, functionName));
    }

    @Override
    public Map<String, Object> getRawObject(String objectId) {
        return call("sui_getObject", List.of(objectId, Map.of("showBcs", true)));
    }

    @Override
    public Map<String, Object> tryGetPastObject(String objectId, long version) {
        return call("sui_tryGetPastObject", List.of(objectId, version, Map.of()));
    }

    @Override
    public Map<String, Object> getProtocolConfig() {
        return call("sui_getProtocolConfig", List.of());
    }

    @Override
    public Map<String, Object> resolveNameServiceAddress(String name) {
        return call("suix_resolveNameServiceAddress", List.of(name));
    }

    @Override
    public Map<String, Object> resolveNameServiceNames(String address) {
        return call("suix_resolveNameServiceNames", List.of(address, null, null));
    }

    @Override
    public Map<String, Object> getCheckpoint(long digestID) {
        return call("sui_getCheckpoint", List.of(String.valueOf(digestID)));
    }

    @Override
    public Map<String, Object> getCheckpoints(Map<String, Object> cursor, long limit, boolean descending) {
        return call("sui_getCheckpoints", List.of(cursor, limit, descending));
    }

    @Override
    public Map<String, Object> getLatestCheckpointSequenceNumber() {
        return call("sui_getLatestCheckpointSequenceNumber", List.of());
    }

    @Override
    public Map<String, Object> getPackage(String packageId) {
        return call(
                "sui_getObject",
                List.of(packageId, Map.of(
                        "showType", true,
                        "showOwner", true,
                        "showPreviousTransaction", true,
                        "showDisplay", false,
                        "showContent", true,
                        "showBcs", true,
                        "showStorageRebate", true
                ))
        );
    }
}
