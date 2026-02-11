package com.suisdks.sui.deepbook_v3;

import java.math.BigInteger;
import java.util.*;

public class DeepBookBalanceManager {
    private final DeepBookV3Client client;

    public DeepBookBalanceManager(DeepBookV3Client client) {
        this.client = client;
    }

    protected DeepBookV3Client client() {
        return client;
    }

    // ==================== Balance Manager Operations ====================

    public Map<String, Object> createBalanceManager(String sender, boolean share) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager", // module
            "create_balance_manager", // function
            Map.of()
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> depositIntoManager(String sender, String managerKey,
            String coinKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "deposit_into_manager",
            Map.of(
                "managerKey", managerKey,
                "coinKey", coinKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> withdrawFromManager(String sender, String managerKey,
            String coinKey, BigInteger amount, String recipient) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "withdraw_from_manager",
            Map.of(
                "managerKey", managerKey,
                "coinKey", coinKey,
                "amount", amount.toString(),
                "recipient", recipient
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> checkManagerBalance(String managerKey, String coinKey) {
        return client.callRpc("suix_getObject", List.of(
            "objectId", managerKey,
            "options", Map.of("showContent", true)
        ));
    }

    public Map<String, Object> generateProof(String managerKey) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "generate_proof",
            Map.of("managerKey", managerKey)
        );

        return client.callRpc("suix_devInspectTransactionBlock",
            List.of(client.config().address(), txData));
    }

    public Map<String, Object> mintTradeCap(String sender, String managerKey) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "mint_trade_cap",
            Map.of("managerKey", managerKey)
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> mintDepositCap(String sender, String managerKey) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "mint_deposit_cap",
            Map.of("managerKey", managerKey)
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> mintWithdrawalCap(String sender, String managerKey) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "mint_withdrawal_cap",
            Map.of("managerKey", managerKey)
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> depositWithCap(String sender, String managerKey,
            String tradeCap, String coinKey, BigInteger amount) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "deposit_with_cap",
            Map.of(
                "managerKey", managerKey,
                "tradeCap", tradeCap,
                "coinKey", coinKey,
                "amount", amount.toString()
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> withdrawWithCap(String sender, String managerKey,
            String withdrawalCap, String coinKey, BigInteger amount, String recipient) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "withdraw_with_cap",
            Map.of(
                "managerKey", managerKey,
                "withdrawalCap", withdrawalCap,
                "coinKey", coinKey,
                "amount", amount.toString(),
                "recipient", recipient
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> revokeTradeCap(String sender, String managerKey, String tradeCap) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "revoke_trade_cap",
            Map.of("managerKey", managerKey, "tradeCap", tradeCap)
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> setBalanceManagerReferral(String sender, String managerKey,
            String referral, String tradeCap) {
        Map<String, Object> txData = buildTransaction(
            client.config().deepbookPackage(),
            "balance_manager",
            "set_balance_manager_referral",
            Map.of(
                "managerKey", managerKey,
                "referral", referral,
                "tradeCap", tradeCap
            )
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> unsetBalanceManagerReferral(String sender, String poolKey,
            String referral, String tradeCap) {
        Map<String, Object> data = new HashMap<>();
        data.put("poolKey", poolKey);
        data.put("referral", referral);
        data.put("tradeCap", tradeCap);

        Map<String, Object> txData = buildTransactionWithArgs(
            client.config().deepbookPackage(),
            "balance_manager",
            "unset_balance_manager_referral",
            data
        );

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
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

    private Map<String, Object> buildTransactionWithArgs(String pkg, String module, String fun, Map<String, Object> args) {
        return buildTransaction(pkg, module, fun, args);
    }

    // ==================== Trade Cap Methods ====================

    public Map<String, Object> createTradeCap(String sender, String managerKey,
            String trader, double limit) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::balance_manager::create_trade_cap");
        
        Map<String, Object> args = new HashMap<>();
        args.put("managerKey", managerKey);
        args.put("trader", trader);
        args.put("limit", limit);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> updateTradeCap(String sender, String capId, double newLimit) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::balance_manager::update_trade_cap");
        
        Map<String, Object> args = new HashMap<>();
        args.put("capId", capId);
        args.put("newLimit", newLimit);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> addTraderToCap(String sender, String capId, String trader) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::balance_manager::add_trader");
        
        Map<String, Object> args = new HashMap<>();
        args.put("capId", capId);
        args.put("trader", trader);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> removeTraderFromCap(String sender, String capId, String trader) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::balance_manager::remove_trader");
        
        Map<String, Object> args = new HashMap<>();
        args.put("capId", capId);
        args.put("trader", trader);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }
}