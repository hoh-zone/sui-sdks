package com.suisdks.sui.deepbook_v3;

import java.math.BigInteger;
import java.util.*;

public class DeepBookMarginTrading {
    private final DeepBookV3Client client;

    public DeepBookMarginTrading(DeepBookV3Client client) {
        this.client = client;
    }

    // ==================== Margin Pool Operations ====================

    public Map<String, Object> getMarginPool(String coinType) {
        Map<String, Object> params = new HashMap<>();
        params.put("type", "string");
        
        return client.callRpc("suix_getDynamicField", List.of(
            client.config().registryPackage() + "::margin_registry::MarginPools",
            params, List.of(coinType)
        ));
    }

    public Map<String, Object> getMarginPoolSupply(String coinType) {
        Map<String, Object> result = getMarginPool(coinType);
        Map<String, Object> data = (Map<String, Object>) result.get("result");
        
        if (data != null && data.containsKey("fields")) {
            Map<String, Object> fields = (Map<String, Object>) data.get("fields");
            return Map.of(
                "totalSupply", fields.getOrDefault("total_supply", "0"),
                "totalBorrow", fields.getOrDefault("total_borrow", "0")
            );
        }
        
        return Map.of("totalSupply", "0", "totalBorrow", "0");
    }

    public Map<String, Object> getMarginPoolInterestRate(String coinType) {
        Map<String, Object> params = new HashMap<>();
        params.put("type", "string");
        
        return client.callRpc("suix_getDynamicField", List.of(
            client.config().registryPackage() + "::margin_pool::InterestRates",
            params, List.of(coinType)
        ));
    }

    // ==================== Margin Manager Operations ====================

    public Map<String, Object> createMarginManager(String sender, String poolKey, BigInteger stakeAmount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "new_margin_manager",
            Map.of(
                "poolKey", poolKey,
                "depositAmount", stakeAmount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> depositBase(String sender, String marginManagerKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "deposit_base",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> depositQuote(String sender, String marginManagerKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "deposit_quote",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> depositDeep(String sender, String marginManagerKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "deposit_deep",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> withdrawBase(String sender, String marginManagerKey, BigInteger amount, String recipient) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "withdraw_base",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString(),
                "recipient", recipient
            )
        );

        return client.callRpc("suixInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> withdrawQuote(String sender, String marginManagerKey, BigInteger amount, String recipient) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "withdraw_quote",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString(),
                "recipient", recipient
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> withdrawDeep(String sender, String marginManagerKey, BigInteger amount, String recipient) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "withdraw_deep",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString(),
                "recipient", recipient
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> borrowBase(String sender, String marginManagerKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "borrow_base",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> borrowQuote(String sender, String marginManagerKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "borrow_quote",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> repayBase(String sender, String marginManagerKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "repay_base",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> repayQuote(String sender, String marginManagerKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().marginPackage(),
            "margin_manager",
            "repay_quote",
            Map.of(
                "marginManagerKey", marginManagerKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getMarginManagerState(String marginManagerKey) {
        return client.callRpc("suix_getObject", Map.of(
            "objectId", marginManagerKey,
            "options", Map.of("showContent", true)
        ));
    }

    private Map<String, Object> buildTransaction(String pkg, String module, String fun, Map<String, Object> args) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", pkg + "::" + module + "::" + fun);
        
        List<Object> argList = new ArrayList<>();
        for (Map.Entry<String, Object> entry : args.entrySet()) {
            argList.add(entry);
        }
        moveCall.put("arguments", argList);
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return txData;
    }

    // ==================== Liquidation Methods ====================

    public Map<String, Object> forceLiquidate(String sender, String marginManagerKey, String liquidatorKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::margin_manager::force_liquidate");
        
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        args.put("liquidatorKey", liquidatorKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> calculateLiquidation(String sender, String marginManagerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::margin_manager::calculate_liquidation");
        
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> isOverCollateralized(String sender, String marginManagerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::margin_manager::is_over_collateralized");
        
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getLiquidationPrice(String sender, String marginManagerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::margin_manager::get_liquidation_price");
        
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }
}