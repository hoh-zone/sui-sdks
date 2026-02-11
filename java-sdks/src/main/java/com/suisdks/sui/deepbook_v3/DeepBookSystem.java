package com.suisdks.sui.deepbook_v3;

import java.math.BigInteger;
import java.util.*;

public class DeepBookSystem {
    private final DeepBookV3Client client;

    public DeepBookSystem(DeepBookV3Client client) {
        this.client = client;
    }

    public DeepBookV3Client client() {
        return client;
    }

    // ==================== Flash Loans ====================

    public Map<String, Object> borrowFlashLoan(String sender, String poolKey,
            boolean isBase, BigInteger amount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        String function = isBase ? "borrow_flashloan_base" : "borrow_flashloan_quote";
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::" + function);
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("amount", amount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> returnFlashLoan(String sender, String poolKey,
            boolean isBase, BigInteger amount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        String function = isBase ? "return_flashloan_base" : "return_flashloan_quote";
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::" + function);
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("amount", amount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== Governance ====================

    public Map<String, Object> createProposal(String sender, String poolKey,
            double takerFee, double makerFee, double stakeRequired) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::submit_proposal");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("takerFee", takerFee);
        args.put("makerFee", makerFee);
        args.put("stakeRequired", stakeRequired);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> vote(String sender, String poolKey,
            String balanceManagerKey, String proposalId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::vote");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("balanceManagerKey", balanceManagerKey);
        args.put("proposalId", proposalId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> executeProposal(String sender, String proposalId) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::execute_proposal");
        
        Map<String, Object> args = new HashMap<>();
        args.put("proposalId", proposalId);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== DEEP Token ====================

    public Map<String, Object> stakeDeep(String sender, String poolKey,
            String balanceManagerKey, BigInteger amount) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::stake");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("balanceManagerKey", balanceManagerKey);
        args.put("amount", amount.toString());
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> unstakeDeep(String sender, String poolKey,
            String balanceManagerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::unstake");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("balanceManagerKey", balanceManagerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> claimRebates(String sender, String poolKey,
            String balanceManagerKey) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::pool::claim_rebates");
        
        Map<String, Object> args = new HashMap<>();
        args.put("poolKey", poolKey);
        args.put("balanceManagerKey", balanceManagerKey);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    // ==================== Pyth Oracle ====================

    public Map<String, Object> getPythPriceUpdate(String sender, List<String> priceIds) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", "pyth::oracle::get_price_feeds");
        
        Map<String, Object> args = new HashMap<>();
        args.put("priceIds", priceIds);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("sui_getObject", txData);
    }

    public Map<String, Object> verifyPythPrice(String sender, String priceUpdateData) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", "pyth::price_feed::update_price_feeds");
        
        Map<String, Object> args = new HashMap<>();
        args.put("priceUpdateData", priceUpdateData);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("sui_getObject", txData);
    }

    // ==================== DEBATE Functions ====================

    public Map<String, Object> debateVote(String sender, String debateId, boolean vote) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::debate::vote");
        
        Map<String, Object> args = new HashMap<>();
        args.put("debateId", debateId);
        args.put("vote", vote);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }

    public Map<String, Object> debateCreate(String sender, String topic, String description) {
        Map<String, Object> txData = new HashMap<>();
        txData.put("kind", "programmableTransaction");
        
        Map<String, Object> moveCall = new HashMap<>();
        moveCall.put("kind", "moveCall");
        moveCall.put("target", client.config().deepbookPackage() + "::debate::create");
        
        Map<String, Object> args = new HashMap<>();
        args.put("topic", topic);
        args.put("description", description);
        moveCall.put("arguments", List.of(args));
        
        txData.put("inputs", List.of());
        txData.put("transactions", List.of(moveCall));

        return client.callRpc("suix_devInspectTransactionBlock", List.of(sender, txData));
    }
}