package com.suisdks.sui.deepbook_v3;

import java.math.BigInteger;
import java.util.*;

public class DeepBookQueryOperations {
    private final DeepBookV3Client client;

    public DeepBookQueryOperations(DeepBookV3Client client) {
        this.client = client;
    }

    // ==================== Order Book Queries ====================

    public Map<String, Object> getLevel2Range(String sender, String poolKey,
            BigInteger priceLow, BigInteger priceHigh, boolean isBid) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::deepbook::get_level2_range");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("priceLow", priceLow.toString());
        args.put("priceHigh", priceHigh.toString());
        args.put("isBid", isBid);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getLevel2TicksFromMid(String sender, String poolKey,
            int ticks, boolean isBid) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::deepbook::get_level2_ticks_from_mid");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("ticks", ticks);
        args.put("isBid", isBid);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> accountOpenOrders(String sender, String poolKey, String managerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::deepbook::account_open_orders");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getOrder(String accountKey) {
        Map<String, Object> options = new HashMap<>();
        options.put("showContent", true);
        
        return client.callRpc("sui_getObject", Map.of(
            "objectId", accountKey,
            "options", options
        ));
    }

    public Map<String, Object> getOrders(List<String> accountKeys) {
        Map<String, Object> options = new HashMap<>();
        options.put("showContent", true);
        
        return client.callRpc("sui_multiGetObjects", List.of(accountKeys, options));
    }
}