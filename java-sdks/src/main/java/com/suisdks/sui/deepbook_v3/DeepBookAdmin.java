package com.suisdks.sui.deepbook_v3;

import java.math.BigInteger;
import java.util.*;

public class DeepBookAdmin {
    private final DeepBookV3Client client;

    public DeepBookAdmin(DeepBookV3Client client) {
        this.client = client;
    }

    public DeepBookV3Client client() {
        return client;
    }

    // ==================== Admin Operations ====================

    public Map<String, Object> createPoolAdmin(String sender, String baseCoinKey, String quoteCoinKey,
            double tickSize, double lotSize, double minSize) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::create_pool_admin");
        
        Map<String, Object> args = new HashMap<>();
        args.put("baseCoinKey", baseCoinKey);
        args.put("quoteCoinKey", quoteCoinKey);
        args.put("tickSize", tickSize);
        args.put("lotSize", lotSize);
        args.put("minSize", minSize);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> setTickSize(String sender, String poolKey, double tickSize) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::set_tick_size");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("tickSize", tickSize);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> setLotSize(String sender, String poolKey, double lotSize) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::set_lot_size");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("lotSize", lotSize);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> setBasePricePoint(String sender, String poolKey, BigInteger price) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::set_base_price_point");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("price", price.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> setQuotePricePoint(String sender, String poolKey, BigInteger price) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::set_quote_price_point");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("price", price.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> updatePoolAllowedVersions(String sender, String poolKey, String registryId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::update_pool_allowed_versions");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("registryId", registryId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> withdrawAll(String sender, String poolKey, String recipient) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::withdraw_all");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("recipient", recipient);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> removePool(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::remove_pool");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> burnDeep(String sender, String poolKey, String treasuryId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::burn_deep");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("treasuryId", treasuryId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }
}