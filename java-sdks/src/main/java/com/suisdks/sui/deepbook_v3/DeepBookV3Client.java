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
}