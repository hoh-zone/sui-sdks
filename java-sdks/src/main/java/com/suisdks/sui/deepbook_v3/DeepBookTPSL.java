package com.suisdks.sui.deepbook_v3;

import java.math.BigInteger;
import java.util.*;

public class DeepBookTPSL {
    private final DeepBookV3Client client;

    public DeepBookTPSL(DeepBookV3Client client) {
        this.client = client;
    }

    // ==================== Conditional Orders (TPSL) ====================

    public Map<String, Object> addConditionalOrder(String sender,
            String marginManagerKey, String poolKey,
            boolean triggerBelowPrice, String triggerPrice,
            String orderType, String orderSide, BigInteger quantity,
            BigInteger price, String clientOrderId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");

        List<Map<String, Object>> transactions = new ArrayList<>();
        
        // First: Set conditional order
        Map<String, Object> setOrder = new HashMap<>();
        setOrder.put("kind", "moveCall");
        setOrder.put("target", client.config().marginPackage() + "::margin_tpsl::set_conditional_order");
        Map<String, Object> orderArgs = new HashMap<>();
        orderArgs.put("marginManagerKey", marginManagerKey);
        orderArgs.put("triggerBelowPrice", triggerBelowPrice);
        orderArgs.put("triggerPrice", triggerPrice);
        orderArgs.put("orderType", orderType);
        orderArgs.put("orderSide", orderSide);
        orderArgs.put("quantity", quantity.toString());
        orderArgs.put("price", price.toString());
        orderArgs.put("clientOrderId", clientOrderId);
        setOrder.put("arguments", List.of(orderArgs));
        transactions.add(setOrder);

        txData.put("inputs", List.of());
        txData.put("transactions", transactions);

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> cancelConditionalOrder(String sender, String marginManagerKey,
            String conditionalOrderId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");

        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().marginPackage() + "::margin_tpsl::cancel_conditional_order");
        
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        args.put("conditionalOrderId", conditionalOrderId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> cancelAllConditionalOrders(String sender, String marginManagerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");

        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().marginPackage() + "::margin_tpsl::cancel_all_conditional_orders");
        
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> executeConditionalOrders(String managerAddress, String poolKey,
        int maxOrders, String gasPrice) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");

        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().marginPackage() + "::margin_tpsl::execute_conditional_orders");
        
        Map<String, Object> data = new HashMap<>();
        data.put("marginManagerAddress", managerAddress);
        data.put("poolKey", poolKey);
        data.put("maxOrders", maxOrders);
        
        Map<String, Object> params = new HashMap<>();
        params.put("data", data);
        params.put("gasPrice", gasPrice);
        
        // Add Move call input for maxOrders and gasPrice
        List<Map<String, Object>> transactions = new ArrayList<>();
        transactions.add(moveCall);
        txData.put("transactions", transactions);
        txData.put("inputs", List.of());

        return client.callRpc("suix_devInspectTransactionBlock", List.of(
            client.config().address(), txData
        ));
    }

    public Map<String, Object> getConditionalOrders(String poolKey, String marginManagerId) {
        return client.callRpc("suix_getDynamicField", List.of(
            poolKey + "::margin_manager::conditional_orders",
            Map.of(
                "type", "string",
                "key", marginManagerId
            )
        ));
    }

    public Map<String, Object> getConditionalOrder(String poolKey, String marginManagerId,
            String conditionalOrderId) {
        return client.callRpc("suix_getDynamicField", List.of(
            poolKey + "::margin_manager::conditional_order_" + conditionalOrderId,
            Map.of("type", "u64")
        ));
    }

    public String lowestTriggerAbovePrice(String poolKey, String marginManagerId) {
        Map<String, Object> state = getMarginManagerState(marginManagerId);
        Map<String, Object> data = (Map<String, Object>) state.get("data");
        
        if (data != null && data.containsKey("fields")) {
            Map<String, Object> fields = (Map<String, Object>) data.get("fields");
            return (String) fields.getOrDefault("lowest_trigger_above_price", "0");
        }
        
        return "0";
    }

    public String highestTriggerBelowPrice(String poolKey, String marginManagerId) {
        Map<String, Object> state = getMarginManagerState(marginManagerId);
        Map<String, Object> data = (Map<String, Object>) state.get("data");
        
        if (data != null && data.containsKey("fields")) {
            Map<String, Object> fields = (Map<String, Object>) data.get("fields");
            return (String) fields.getOrDefault("highest_trigger_below_price", "0");
        }
        
        return "0";
    }

    private Map<String, Object> getMarginManagerState(Object marginManagerId) {
        return client.callRpc("sui_getObject", Map.of(
            "objectId", marginManagerId,
            "options", Map.of("showContent", true)
        ));
    }
}