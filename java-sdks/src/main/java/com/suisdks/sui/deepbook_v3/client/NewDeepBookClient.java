package com.suisdks.sui.deepbook_v3.client;

import com.suisdks.sui.deepbook_v3.DeepBookConfig;
import com.suisdks.sui.jsonrpc.JsonRpcClient;

import java.math.BigInteger;
import java.util.*;

public class NewDeepBookClient {
    private final JsonRpcClient rpcClient;
    private DeepBookConfig config;

    public NewDeepBookClient(JsonRpcClient rpcClient, DeepBookConfig config) {
        this.rpcClient = rpcClient;
        this.config = config;
    }

    public static NewDeepBookClient forMainnet(JsonRpcClient rpcClient) {
        return new NewDeepBookClient(rpcClient, DeepBookConfig.mainnet());
    }

    public static NewDeepBookClient forTestnet(JsonRpcClient rpcClient) {
        return new NewDeepBookClient(rpcClient, DeepBookConfig.testnet());
    }

    public JsonRpcClient getRpcClient() {
        return rpcClient;
    }

    public String getEndpoint() {
        return rpcClient.getEndpoint();
    }

    // ==================== Helper Methods ====================

    private Map<String, Object> callRpc(String method, List<Object> params) {
        return rpcClient.callRpcInternal(method, params);
    }

    private Map<String, Object> devInspectTransactionBlock(String sender, Map<String, Object> transactionData) {
        return callRpc("suix_devInspectTransactionBlock", List.of(sender, transactionData));
    }

    // ==================== Trading Operations ====================

    public Map<String, Object> placeLimitOrder(String sender, String poolKey, String managerKey,
            boolean isBid, BigInteger price, BigInteger quantity, String clientOrderId, String restriction,
            Long expiration, String selfMatching) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("managerKey", managerKey);
        data.put("isBid", isBid);
        data.put("price", price.toString());
        data.put("quantity", quantity.toString());
        data.put("clientOrderId", clientOrderId);
        data.put("restriction", restriction);
        if (expiration != null) {
            data.put("expiration", expiration);
        }
        if (selfMatching != null) {
            data.put("selfMatching", selfMatching);
        }

        return devInspectTransactionBlock(sender, data);
    }

    public Map<String, Object> placeMarketOrder(String sender, String poolKey, String managerKey,
            boolean isBid, BigInteger quantity, String clientOrderId) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("managerKey", managerKey);
        data.put("isBid", isBid);
        data.put("quantity", quantity.toString());
        data.put("clientOrderId", clientOrderId);

        return devInspectTransactionBlock(sender, data);
    }

    public Map<String, Object> modifyOrder(String sender, String poolKey, String managerKey,
            String orderId, BigInteger newQuantity) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("managerKey", managerKey);
        data.put("orderId", orderId);
        data.put("quantity", newQuantity.toString());

        return devInspectTransactionBlock(sender, data);
    }

    public Map<String, Object> cancelOrder(String sender, String poolKey, String managerKey, String orderId) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("managerKey", managerKey);
        data.put("orderId", orderId);

        return devInspectTransactionBlock(sender, data);
    }

    public Map<String, Object> cancelAllOrders(String sender, String poolKey, String managerKey) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("managerKey", managerKey);

        return devInspectTransactionBlock(sender, data);
    }

    public Map<String, Object> withdrawSettledAmounts(String sender, String poolKey, String managerKey) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("managerKey", managerKey);

        return devInspectTransactionBlock(sender, data);
    }

    // ==================== Swap Operations ====================

    public Map<String, Object> swapExactBaseForQuote(String sender, String poolKey,
            String baseAsset, String quoteAsset, BigInteger baseAmount, BigInteger minQuoteAmount) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("baseAsset", baseAsset);
        data.put("quoteAsset", quoteAsset);
        data.put("baseAmount", baseAmount.toString());
        if (minQuoteAmount != null) {
            data.put("minQuoteAmount", minQuoteAmount.toString());
        }

        return devInspectTransactionBlock(sender, data);
    }

    public Map<String, Object> swapExactQuoteForBase(String sender, String poolKey,
            String quoteAsset, String baseAsset, BigInteger quoteAmount) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("quoteAsset", quoteAsset);
        data.put("baseAsset", baseAsset);
        data.put("quoteAmount", quoteAmount.toString());

        return devInspectTransactionBlock(sender, data);
    }

    // ==================== Pool Queries ====================

    public Map<String, Object> poolTradeParams(String poolKey) {
        return this.rpcClient.getObject(poolKey, Map.of("showContent", true));
    }

    public Map<String, Object> poolBookParams(String poolKey) {
        return this.rpcClient.getObject(poolKey, Map.of("showContent", true));
    }

    public Map<String, Object> vaultBalances(String poolKey) {
        return this.rpcClient.getObject(poolKey, Map.of("showContent", true));
    }

    public Map<String, Object> midPrice(String poolKey) {
        Map<String, Object> pool = poolTradeParams(poolKey);
        Map<String, Object> poolData = (Map<String, Object>) pool.get("data");
        if (poolData != null && poolData.containsKey("fields")) {
            Map<String, Object> fields = (Map<String, Object>) poolData.get("fields");
            return Map.of("midPrice", fields.getOrDefault("mid_price", "0"));
        }
        return Map.of("midPrice", "0");
    }

    public Map<String, Object> quoteQuantityOut(String poolKey, BigInteger baseQuantity) {
        Map<String, Object> params = new java.util.HashMap<>();
        Map<String, Object> typeData = new java.util.HashMap<>();
        typeData.put("type", "u64");
        Map<String, Object> valueData = new java.util.HashMap<>();
        valueData.put("poolKey", poolKey);
        valueData.put("baseQuantity", baseQuantity.toString());
        typeData.put("value", valueData);
        return callRpc("suix_getDynamicField", List.of(poolKey, typeData));
    }

    public Map<String, Object> baseQuantityOut(String poolKey, BigInteger quoteQuantity) {
        Map<String, Object> typeData = new java.util.HashMap<>();
        typeData.put("type", "u64");
        Map<String, Object> valueData = new java.util.HashMap<>();
        valueData.put("poolKey", poolKey);
        valueData.put("quoteQuantity", quoteQuantity.toString());
        typeData.put("value", valueData);
        return callRpc("suix_getDynamicField", List.of(poolKey, typeData));
    }

    // ==================== Order Book Queries ====================

    public Map<String, Object> getLevel2Range(String sender, String poolKey,
            BigInteger priceLow, BigInteger priceHigh, boolean isBid) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("priceLow", priceLow.toString());
        data.put("priceHigh", priceHigh.toString());
        data.put("isBid", isBid);

        return devInspectTransactionBlock(sender, data);
    }

    public Map<String, Object> getLevel2TicksFromMid(String sender, String poolKey,
            int ticks, boolean isBid) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("ticks", ticks);
        data.put("isBid", isBid);

        return devInspectTransactionBlock(sender, data);
    }

    // ==================== Order Queries ====================

    public Map<String, Object> getOrder(String poolKey, String orderId) {
        return this.rpcClient.getObject(orderId, Map.of("showContent", true));
    }

    public Map<String, Object> getOrders(String poolKey, List<String> orderIds) {
        return callRpc("sui_multiGetObjects", List.of(orderIds, Map.of("showContent", true)));
    }

    public Map<String, Object> accountOpenOrders(String poolKey, String managerKey) {
        Map<String, Object> data = new java.util.HashMap<>();
        data.put("poolKey", poolKey);
        data.put("managerKey", managerKey);
        
        Map<String, Object> params = new java.util.HashMap<>();
        params.put("MoveCall", data);
        
        return callRpc("suix_devInspectTransactionBlock", List.of(config.address(), params));
    }
}
