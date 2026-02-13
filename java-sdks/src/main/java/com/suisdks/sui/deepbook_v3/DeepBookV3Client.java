package com.suisdks.sui.deepbook_v3;

import com.suisdks.sui.jsonrpc.HttpJsonRpcTransport;
import com.suisdks.sui.jsonrpc.JsonRpcTransport;

import java.math.BigInteger;
import java.util.*;

public class DeepBookV3Client {
    private final JsonRpcTransport transport;
    private final DeepBookConfig config;

    public DeepBookV3Client(String endpoint, DeepBookConfig config) {
        this.transport = new HttpJsonRpcTransport(endpoint);
        this.config = config;
    }

    public static DeepBookV3Client forMainnet() {
        return new DeepBookV3Client("https://fullnode.mainnet.sui.io", DeepBookConfig.mainnet());
    }

    public static DeepBookV3Client forTestnet() {
        return new DeepBookV3Client("https://fullnode.testnet.sui.io", DeepBookConfig.testnet());
    }

    Map<String, Object> callRpc(String method, Object params) {
        try {
            return transport.request(method, params == null ? List.of() : (params instanceof List ? (List<Object>)params : List.of(params)));
        } catch (Exception e) {
            throw new RuntimeException("RPC call failed: " + method, e);
        }
    }

    private Map<String, Object> inspectMoveCall(String sender, String target, Map<String, Object> args) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        txData.put("inputs", List.of());
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", target);
        moveCall.put("arguments", List.of(args));
        txData.put("transactions", List.of(moveCall));
        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    private String defaultSender() {
        return config.address();
    }

    public DeepBookConfig config() {
        return config;
    }

    // ==================== Trading Operations ====================

    public Map<String, Object> placeLimitOrder(String sender, String poolKey, String managerKey,
            boolean isBid, BigInteger price, BigInteger quantity) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        txData.put("inputs", List.of());
        
        List<Object> transactions = new ArrayList<>();
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::deepbook::place_limit_order");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("isBid", isBid);
        args.put("price", price.toString());
        args.put("quantity", quantity.toString());
        moveCall.put("arguments", List.of(args));
        
        transactions.add(moveCall);
        txData.put("transactions", transactions);

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> placeMarketOrder(String sender, String poolKey, String managerKey,
            boolean isBid, BigInteger quantity) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        txData.put("inputs", List.of());
        
        List<Object> transactions = new ArrayList<>();
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::deepbook::place_market_order");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("isBid", isBid);
        args.put("quantity", quantity.toString());
        moveCall.put("arguments", List.of(args));
        
        transactions.add(moveCall);
        txData.put("transactions", transactions);

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> cancelOrder(String sender, String poolKey, String managerKey, String orderId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::deepbook::cancel_order");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("orderId", orderId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> cancelAllOrders(String sender, String poolKey, String managerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::deepbook::cancel_all_orders");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> modifyOrder(String sender, String poolKey, String managerKey,
            String orderId, BigInteger newQuantity) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::deepbook::modify_order");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("orderId", orderId);
        args.put("quantity", newQuantity.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== Pool Queries ====================

    public Map<String, Object> getPoolObject(String poolKey) {
        Map<String, Object> options = new HashMap<>();
        options.put("showContent", true);
        options.put("showOwner", true);
        
        Map<String, Object> payload = new HashMap<>();
        payload.put("objectId", poolKey);
        payload.put("options", options);
        
        return callRpc("sui_getObject", payload);
    }

    public Map<String, Object> midPrice(String poolKey) {
        Map<String, Object> pool = getPoolObject(poolKey);
        Map<String, Object> data = (Map<String, Object>) pool.get("data");
        
        if (data != null && data.containsKey("fields")) {
            Map<String, Object> fields = (Map<String, Object>) data.get("fields");
            return Map.of("midPrice", fields.getOrDefault("mid_price", "0"));
        }
        
        return Map.of("midPrice", "0");
    }

    public Map<String, Object> poolTradeParams(String poolKey) {
        return getPoolObject(poolKey);
    }

    public Map<String, Object> vaultBalances(String poolKey) {
        return getPoolObject(poolKey);
    }

    // ==================== Swap Operations with Fees ====================

    public Map<String, Object> swapExactBaseForQuoteWithFee(String sender, String poolKey,
            String baseAsset, String quoteAsset, BigInteger baseAmount, BigInteger minQuoteAmount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        List<Object> transactions = new ArrayList<>();
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_quote_quantity_out_input_fee");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("baseAsset", baseAsset);
        args.put("quoteAsset", quoteAsset);
        args.put("baseAmount", baseAmount.toString());
        args.put("minQuoteAmount", minQuoteAmount.toString());
        moveCall.put("arguments", List.of(args));
        
        transactions.add(moveCall);
        txData.put("inputs", List.of());
        txData.put("transactions", transactions);

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getQuoteQuantityInputFee(String sender, String poolKey, BigInteger amount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_quote_quantity_out_input_fee");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("amount", amount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getBaseQuantityInputFee(String sender, String poolKey, BigInteger amount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_base_quantity_out_input_fee");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("amount", amount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getQuantityInputFee(String sender, String poolKey, BigInteger baseAmount, BigInteger quoteAmount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_quantity_out_input_fee");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("baseAmount", baseAmount.toString());
        args.put("quoteAmount", quoteAmount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getBaseQuantityIn(String sender, String poolKey, BigInteger targetQuoteAmount, boolean payWithDeep) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_base_quantity_in");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("targetQuoteAmount", targetQuoteAmount.toString());
        args.put("payWithDeep", payWithDeep);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getQuoteQuantityIn(String sender, String poolKey, BigInteger targetBaseAmount, boolean payWithDeep) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_quote_quantity_in");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("targetBaseAmount", targetBaseAmount.toString());
        args.put("payWithDeep", payWithDeep);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getPoolDeepPrice(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_order_deep_price");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getPoolTradeParamsNext(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::pool_trade_params_next");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getPoolBookParams(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::pool_book_params");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== Swap Operations ====================

    public Map<String, Object> swapExactBaseForQuote(String sender, String poolKey,
            String baseAsset, String quoteAsset, BigInteger baseAmount, BigInteger minQuoteAmount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::deepbook::swap_exact_base_for_quote");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("baseAsset", baseAsset);
        args.put("quoteAsset", quoteAsset);
        args.put("baseAmount", baseAmount.toString());
        args.put("minQuoteAmount", minQuoteAmount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> swapExactQuoteForBase(String sender, String poolKey,
            String quoteAsset, String baseAsset, BigInteger quoteAmount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::deepbook::swap_exact_quote_for_base");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("quoteAsset", quoteAsset);
        args.put("baseAsset", baseAsset);
        args.put("quoteAmount", quoteAmount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== Order Queries ====================

    public Map<String, Object> getOrder(String orderId) {
        Map<String, Object> options = new HashMap<>();
        options.put("showContent", true);
        
        return callRpc("sui_getObject", Map.of("objectId", orderId, "options", options));
    }

    public Map<String, Object> getOrders(List<String> orderIds) {
        Map<String, Object> options = new HashMap<>();
        options.put("showContent", true);
        
        return callRpc("sui_multiGetObjects", List.of(orderIds, options));
    }

    // ==================== Order Pre-validation ====================

    public Map<String, Object> canPlaceLimitOrder(String sender, String poolKey, String managerKey,
            boolean isBid, BigInteger price, BigInteger quantity, boolean payWithDeep, int expiration) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::can_place_limit_order");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("isBid", isBid);
        args.put("price", price.toString());
        args.put("quantity", quantity.toString());
        args.put("payWithDeep", payWithDeep);
        args.put("expiration", expiration);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> canPlaceMarketOrder(String sender, String poolKey, String managerKey,
            boolean isBid, BigInteger quantity, boolean payWithDeep) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::can_place_market_order");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("isBid", isBid);
        args.put("quantity", quantity.toString());
        args.put("payWithDeep", payWithDeep);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== Account Queries ====================

    public Map<String, Object> getAccount(String sender, String poolKey, String managerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::account");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getLockedBalance(String sender, String poolKey, String managerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::locked_balance");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getAccountOrderDetails(String sender, String poolKey, String managerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::get_account_order_details");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> accountExists(String sender, String poolKey, String managerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::account_exists");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== Pool Metadata ====================

    public Map<String, Object> isWhitelisted(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::whitelisted");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> isStablePool(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::stable_pool");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> isRegisteredPool(String sender, String poolKey, String registryId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::registered_pool");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("registryId", registryId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getQuorum(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::quorum");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getPoolID(String sender, String poolKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::pool::id");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> getBalanceManagerIDs(String sender, String owner, String registryId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", config.deepbookPackage() + "::registry::get_balance_manager_ids");
        
        Map<String, Object> args = new HashMap<>();
        args.put("owner", owner);
        args.put("registryId", registryId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> cancelOrders(String sender, String poolKey, String managerKey, List<String> orderIds) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("orderIds", orderIds);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::cancel_orders", args);
    }

    public Map<String, Object> withdrawSettledAmounts(String sender, String poolKey, String managerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::withdraw_settled_amounts", args);
    }

    public Map<String, Object> getQuoteQuantityOut(String sender, String poolKey, BigInteger baseQuantity) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("baseQuantity", baseQuantity.toString());
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::get_quote_quantity_out", args);
    }

    public Map<String, Object> getBaseQuantityOut(String sender, String poolKey, BigInteger quoteQuantity) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("quoteQuantity", quoteQuantity.toString());
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::get_base_quantity_out", args);
    }

    public Map<String, Object> getQuantityOut(String sender, String poolKey, BigInteger baseQuantity, BigInteger quoteQuantity) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("baseQuantity", baseQuantity.toString());
        args.put("quoteQuantity", quoteQuantity.toString());
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::get_quantity_out", args);
    }

    public Map<String, Object> accountOpenOrders(String sender, String poolKey, String managerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::account_open_orders", args);
    }

    public Map<String, Object> checkMarketOrderParams(String sender, String poolKey, BigInteger quantity) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("quantity", quantity.toString());
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::check_market_order_params", args);
    }

    public Map<String, Object> checkLimitOrderParams(
            String sender,
            String poolKey,
            BigInteger price,
            BigInteger quantity,
            boolean isBid,
            boolean payWithDeep,
            int expiration
    ) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("price", price.toString());
        args.put("quantity", quantity.toString());
        args.put("isBid", isBid);
        args.put("payWithDeep", payWithDeep);
        args.put("expiration", expiration);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::check_limit_order_params", args);
    }

    public Map<String, Object> getOrderDeepRequired(String sender, String poolKey, BigInteger baseQuantity, BigInteger price) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("baseQuantity", baseQuantity.toString());
        args.put("price", price.toString());
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::get_order_deep_required", args);
    }

    // TS-like convenience overloads (sender defaults to config.address()):
    public Map<String, Object> whitelisted(String poolKey) {
        return isWhitelisted(defaultSender(), poolKey);
    }

    public Map<String, Object> getQuoteQuantityOut(String poolKey, BigInteger baseQuantity) {
        return getQuoteQuantityOut(defaultSender(), poolKey, baseQuantity);
    }

    public Map<String, Object> getBaseQuantityOut(String poolKey, BigInteger quoteQuantity) {
        return getBaseQuantityOut(defaultSender(), poolKey, quoteQuantity);
    }

    public Map<String, Object> getQuantityOut(String poolKey, BigInteger baseQuantity, BigInteger quoteQuantity) {
        return getQuantityOut(defaultSender(), poolKey, baseQuantity, quoteQuantity);
    }

    public Map<String, Object> accountOpenOrders(String poolKey, String managerKey) {
        return accountOpenOrders(defaultSender(), poolKey, managerKey);
    }

    public Map<String, Object> getLevel2Range(String poolKey, BigInteger priceLow, BigInteger priceHigh, boolean isBid) {
        return getLevel2Range(defaultSender(), poolKey, priceLow, priceHigh, isBid);
    }

    public Map<String, Object> getLevel2TicksFromMid(String poolKey, int ticks) {
        return getLevel2TicksFromMid(defaultSender(), poolKey, ticks);
    }

    public Map<String, Object> poolBookParams(String poolKey) {
        return getPoolBookParams(defaultSender(), poolKey);
    }

    public Map<String, Object> account(String poolKey, String managerKey) {
        return getAccount(defaultSender(), poolKey, managerKey);
    }

    public Map<String, Object> lockedBalance(String poolKey, String managerKey) {
        return getLockedBalance(defaultSender(), poolKey, managerKey);
    }

    public Map<String, Object> getPoolDeepPrice(String poolKey) {
        return getPoolDeepPrice(defaultSender(), poolKey);
    }

    public Map<String, Object> getBalanceManagerIds(String owner, String registryId) {
        return getBalanceManagerIDs(defaultSender(), owner, registryId);
    }

    public Map<String, Object> stablePool(String poolKey) {
        return stablePool(defaultSender(), poolKey);
    }

    public Map<String, Object> registeredPool(String poolKey, String registryId) {
        return registeredPool(defaultSender(), poolKey, registryId);
    }

    public Map<String, Object> getQuoteQuantityOutInputFee(String poolKey, BigInteger baseQuantity) {
        return getQuoteQuantityOutInputFee(defaultSender(), poolKey, baseQuantity);
    }

    public Map<String, Object> getBaseQuantityOutInputFee(String poolKey, BigInteger quoteQuantity) {
        return getBaseQuantityOutInputFee(defaultSender(), poolKey, quoteQuantity);
    }

    public Map<String, Object> getQuantityOutInputFee(String poolKey, BigInteger baseQuantity, BigInteger quoteQuantity) {
        return getQuantityOutInputFee(defaultSender(), poolKey, baseQuantity, quoteQuantity);
    }

    public Map<String, Object> getBaseQuantityIn(String poolKey, BigInteger targetQuoteAmount, boolean payWithDeep) {
        return getBaseQuantityIn(defaultSender(), poolKey, targetQuoteAmount, payWithDeep);
    }

    public Map<String, Object> getQuoteQuantityIn(String poolKey, BigInteger targetBaseAmount, boolean payWithDeep) {
        return getQuoteQuantityIn(defaultSender(), poolKey, targetBaseAmount, payWithDeep);
    }

    public Map<String, Object> getAccountOrderDetails(String poolKey, String managerKey) {
        return getAccountOrderDetails(defaultSender(), poolKey, managerKey);
    }

    public Map<String, Object> accountExists(String poolKey, String managerKey) {
        return accountExists(defaultSender(), poolKey, managerKey);
    }

    public Map<String, Object> poolTradeParamsNext(String poolKey) {
        return poolTradeParamsNext(defaultSender(), poolKey);
    }

    public Map<String, Object> quorum(String poolKey) {
        return quorum(defaultSender(), poolKey);
    }

    public Map<String, Object> poolId(String poolKey) {
        return poolId(defaultSender(), poolKey);
    }

    public Map<String, Object> checkMarketOrderParams(String poolKey, BigInteger quantity) {
        return checkMarketOrderParams(defaultSender(), poolKey, quantity);
    }

    public Map<String, Object> checkLimitOrderParams(
            String poolKey,
            BigInteger price,
            BigInteger quantity,
            boolean isBid,
            boolean payWithDeep,
            int expiration
    ) {
        return checkLimitOrderParams(defaultSender(), poolKey, price, quantity, isBid, payWithDeep, expiration);
    }

    public Map<String, Object> getOrderDeepRequired(String poolKey, BigInteger baseQuantity, BigInteger price) {
        return getOrderDeepRequired(defaultSender(), poolKey, baseQuantity, price);
    }

    // Referral and margin query helpers:
    public Map<String, Object> balanceManagerReferralOwner(String sender, String referralId) {
        Map<String, Object> args = new HashMap<>();
        args.put("referralId", referralId);
        return inspectMoveCall(sender, config.deepbookPackage() + "::balance_manager::balance_manager_referral_owner", args);
    }

    public Map<String, Object> balanceManagerReferralPoolId(String sender, String referralId) {
        Map<String, Object> args = new HashMap<>();
        args.put("referralId", referralId);
        return inspectMoveCall(sender, config.deepbookPackage() + "::balance_manager::balance_manager_referral_pool_id", args);
    }

    public Map<String, Object> getBalanceManagerReferralId(String sender, String managerKey, String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("managerKey", managerKey);
        args.put("poolKey", poolKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::balance_manager::get_balance_manager_referral_id", args);
    }

    public Map<String, Object> getPoolReferralBalances(String sender, String poolKey, String referral) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("referral", referral);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::pool_referral_balances", args);
    }

    public Map<String, Object> poolReferralMultiplier(String sender, String poolKey, String referral) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("referral", referral);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::pool_referral_multiplier", args);
    }

    public Map<String, Object> getMarginPoolId(String sender, String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::registry::get_margin_pool_id", args);
    }

    public Map<String, Object> isDeepbookPoolAllowed(String sender, String coinKey, String deepbookPoolId) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        args.put("deepbookPoolId", deepbookPoolId);
        return inspectMoveCall(sender, config.deepbookPackage() + "::margin_pool::deepbook_pool_allowed", args);
    }

    public Map<String, Object> getMarginPoolTotalSupply(String sender, String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::margin_pool::total_supply", args);
    }

    public Map<String, Object> getMarginPoolSupplyShares(String sender, String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::margin_pool::supply_shares", args);
    }

    public Map<String, Object> getMarginPoolTotalBorrow(String sender, String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::margin_pool::total_borrow", args);
    }

    public Map<String, Object> getMarginPoolBorrowShares(String sender, String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::margin_pool::borrow_shares", args);
    }

    public Map<String, Object> getMarginPoolInterestRate(String sender, String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::margin_pool::interest_rate", args);
    }

    public Map<String, Object> getMarginManagerIdsForOwner(String sender, String owner) {
        Map<String, Object> args = new HashMap<>();
        args.put("owner", owner);
        return inspectMoveCall(sender, config.deepbookPackage() + "::registry::get_margin_manager_ids", args);
    }

    public Map<String, Object> getBaseMarginPoolId(String sender, String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::registry::base_margin_pool_id", args);
    }

    public Map<String, Object> getQuoteMarginPoolId(String sender, String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(sender, config.deepbookPackage() + "::registry::quote_margin_pool_id", args);
    }

    public Map<String, Object> checkManagerBalance(String managerKey, String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("managerKey", managerKey);
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::balance_manager::balance", args);
    }

    public Map<String, Object> getOrderNormalized(String orderId) {
        // Baseline normalized response wrapper.
        return Map.of("order", getOrder(orderId), "normalized", true);
    }

    public Map<String, Object> getPriceInfoObject(String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_admin::price_info_object", args);
    }

    public Map<String, Object> getPriceInfoObjects(List<String> coinKeys) {
        Map<String, Object> out = new HashMap<>();
        for (String coinKey : coinKeys) {
            out.put(coinKey, getPriceInfoObject(coinKey));
        }
        return out;
    }

    public Map<String, Object> getPriceInfoObjectAge(String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_admin::price_info_object_age", args);
    }

    public Map<String, Object> getMarginPoolLastUpdateTimestamp(String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_pool::last_update_timestamp", args);
    }

    public Map<String, Object> getMarginPoolSupplyCap(String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_pool::supply_cap", args);
    }

    public Map<String, Object> getMarginPoolMaxUtilizationRate(String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_pool::max_utilization_rate", args);
    }

    public Map<String, Object> getMarginPoolProtocolSpread(String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_pool::protocol_spread", args);
    }

    public Map<String, Object> getMarginPoolMinBorrow(String coinKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_pool::min_borrow", args);
    }

    public Map<String, Object> getUserSupplyShares(String coinKey, String supplierCapId) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        args.put("supplierCapId", supplierCapId);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_pool::user_supply_shares", args);
    }

    public Map<String, Object> getUserSupplyAmount(String coinKey, String supplierCapId) {
        Map<String, Object> args = new HashMap<>();
        args.put("coinKey", coinKey);
        args.put("supplierCapId", supplierCapId);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_pool::user_supply_amount", args);
    }

    public Map<String, Object> getMarginManagerOwner(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::owner_by_pool_key", args);
    }

    public Map<String, Object> getMarginManagerDeepbookPool(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::deepbook_pool", args);
    }

    public Map<String, Object> getMarginManagerMarginPoolId(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::margin_pool_id", args);
    }

    public Map<String, Object> getMarginManagerBorrowedShares(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::borrowed_shares", args);
    }

    public Map<String, Object> getMarginManagerBorrowedBaseShares(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::borrowed_base_shares", args);
    }

    public Map<String, Object> getMarginManagerBorrowedQuoteShares(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::borrowed_quote_shares", args);
    }

    public Map<String, Object> getMarginManagerHasBaseDebt(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::has_base_debt", args);
    }

    public Map<String, Object> getMarginManagerBalanceManagerId(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::balance_manager", args);
    }

    public Map<String, Object> getMarginManagerAssets(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::calculate_assets", args);
    }

    public Map<String, Object> getMarginManagerDebts(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::calculate_debts", args);
    }

    public Map<String, Object> getMarginManagerState(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::manager_state", args);
    }

    public Map<String, Object> getMarginManagerStates(List<String> marginManagerKeys) {
        Map<String, Object> out = new HashMap<>();
        for (String key : marginManagerKeys) {
            out.put(key, getMarginManagerState(key));
        }
        return out;
    }

    public Map<String, Object> getMarginManagerBaseBalance(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::base_balance", args);
    }

    public Map<String, Object> getMarginManagerQuoteBalance(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::quote_balance", args);
    }

    public Map<String, Object> getMarginManagerDeepBalance(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_manager::deep_balance", args);
    }

    public Map<String, Object> getMarginAccountOrderDetails(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::pool_proxy::account_order_details", args);
    }

    public Map<String, Object> getConditionalOrderIds(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_tpsl::conditional_order_ids", args);
    }

    public Map<String, Object> getLowestTriggerAbovePrice(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_tpsl::lowest_trigger_above_price", args);
    }

    public Map<String, Object> getHighestTriggerBelowPrice(String marginManagerKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("marginManagerKey", marginManagerKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::margin_tpsl::highest_trigger_below_price", args);
    }

    public Map<String, Object> isPoolEnabledForMargin(String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::pool_enabled", args);
    }

    public Map<String, Object> getMinWithdrawRiskRatio(String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::min_withdraw_risk_ratio", args);
    }

    public Map<String, Object> getMinBorrowRiskRatio(String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::min_borrow_risk_ratio", args);
    }

    public Map<String, Object> getLiquidationRiskRatio(String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::liquidation_risk_ratio", args);
    }

    public Map<String, Object> getTargetLiquidationRiskRatio(String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::target_liquidation_risk_ratio", args);
    }

    public Map<String, Object> getUserLiquidationReward(String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::user_liquidation_reward", args);
    }

    public Map<String, Object> getPoolLiquidationReward(String poolKey) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::pool_liquidation_reward", args);
    }

    public Map<String, Object> getAllowedMaintainers() {
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::allowed_maintainers", new HashMap<>());
    }

    public Map<String, Object> getAllowedPauseCaps() {
        return inspectMoveCall(defaultSender(), config.deepbookPackage() + "::registry::allowed_pause_caps", new HashMap<>());
    }

    // TS-style aliases and additional parity entry points:
    public Map<String, Object> getQuoteQuantityOutInputFee(String sender, String poolKey, BigInteger baseQuantity) {
        return getQuoteQuantityInputFee(sender, poolKey, baseQuantity);
    }

    public Map<String, Object> getBaseQuantityOutInputFee(String sender, String poolKey, BigInteger quoteQuantity) {
        return getBaseQuantityInputFee(sender, poolKey, quoteQuantity);
    }

    public Map<String, Object> getQuantityOutInputFee(String sender, String poolKey, BigInteger baseQuantity, BigInteger quoteQuantity) {
        return getQuantityInputFee(sender, poolKey, baseQuantity, quoteQuantity);
    }

    public Map<String, Object> poolTradeParamsNext(String sender, String poolKey) {
        return getPoolTradeParamsNext(sender, poolKey);
    }

    public Map<String, Object> poolBookParams(String sender, String poolKey) {
        return getPoolBookParams(sender, poolKey);
    }

    public Map<String, Object> quorum(String sender, String poolKey) {
        return getQuorum(sender, poolKey);
    }

    public Map<String, Object> poolId(String sender, String poolKey) {
        return getPoolID(sender, poolKey);
    }

    public Map<String, Object> stablePool(String sender, String poolKey) {
        return isStablePool(sender, poolKey);
    }

    public Map<String, Object> registeredPool(String sender, String poolKey, String registryId) {
        return isRegisteredPool(sender, poolKey, registryId);
    }

    public Map<String, Object> getPoolIdByAssets(String sender, String baseType, String quoteType, String registryId) {
        Map<String, Object> args = new HashMap<>();
        args.put("registryId", registryId);
        args.put("baseType", baseType);
        args.put("quoteType", quoteType);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::get_pool_id_by_asset", args);
    }

    public Map<String, Object> getLevel2Range(
            String sender,
            String poolKey,
            BigInteger priceLow,
            BigInteger priceHigh,
            boolean isBid
    ) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("priceLow", priceLow.toString());
        args.put("priceHigh", priceHigh.toString());
        args.put("isBid", isBid);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::get_level2_range", args);
    }

    public Map<String, Object> getLevel2TicksFromMid(String sender, String poolKey, int ticks) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("ticks", ticks);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::get_level2_ticks_from_mid", args);
    }

    public Map<String, Object> swapExactQuantity(
            String sender,
            String poolKey,
            String baseAsset,
            String quoteAsset,
            BigInteger baseAmount,
            BigInteger quoteAmount,
            BigInteger minOut,
            boolean isBaseToCoin
    ) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("baseAsset", baseAsset);
        args.put("quoteAsset", quoteAsset);
        args.put("baseAmount", baseAmount.toString());
        args.put("quoteAmount", quoteAmount.toString());
        args.put("minOut", minOut.toString());
        args.put("isBaseToCoin", isBaseToCoin);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::swap_exact_quantity", args);
    }

    public Map<String, Object> swapExactBaseForQuoteWithManager(
            String sender,
            String poolKey,
            String managerKey,
            BigInteger baseAmount,
            BigInteger minQuoteAmount
    ) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("baseAmount", baseAmount.toString());
        args.put("minQuoteAmount", minQuoteAmount.toString());
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::swap_exact_base_for_quote_with_manager", args);
    }

    public Map<String, Object> swapExactQuoteForBaseWithManager(
            String sender,
            String poolKey,
            String managerKey,
            BigInteger quoteAmount,
            BigInteger minBaseAmount
    ) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("quoteAmount", quoteAmount.toString());
        args.put("minBaseAmount", minBaseAmount.toString());
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::swap_exact_quote_for_base_with_manager", args);
    }

    public Map<String, Object> swapExactQuantityWithManager(
            String sender,
            String poolKey,
            String managerKey,
            BigInteger baseAmount,
            BigInteger quoteAmount,
            BigInteger minOut,
            boolean isBaseToCoin
    ) {
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("managerKey", managerKey);
        args.put("baseAmount", baseAmount.toString());
        args.put("quoteAmount", quoteAmount.toString());
        args.put("minOut", minOut.toString());
        args.put("isBaseToCoin", isBaseToCoin);
        return inspectMoveCall(sender, config.deepbookPackage() + "::pool::swap_exact_quantity_with_manager", args);
    }
}
