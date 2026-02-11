package com.suisdks.sui.transactions;

import com.suisdks.sui.transactions.intents.CoinWithBalance;

import java.math.BigInteger;
import java.util.*;

public class Transaction {
    private List<Map<String, Object>> commands = new ArrayList<>();
    private String sender;
    private BigInteger gasPrice;
    private GasData gasData;
    private List<Map<String, Object>> intents = new ArrayList<>();
    private int commandIndex = 0;

    public Transaction() {}

    public static Transaction fromBytes(byte[] bytes) {
        return new Transaction();
    }

    public Transaction setSender(String address) {
        this.sender = address;
        return this;
    }

    public Transaction setGasPrice(BigInteger price) {
        this.gasPrice = price;
        return this;
    }

    public Transaction setGasData(GasData gasData) {
        this.gasData = gasData;
        return this;
    }

    public String getSender() {
        return sender;
    }

    public BigInteger getGasPrice() {
        return gasPrice;
    }

    public GasData getGasData() {
        return gasData;
    }

    public List<Map<String, Object>> getCommands() {
        return commands;
    }

    public List<Map<String, Object>> getIntents() {
        return intents;
    }

    public Transaction addCommand(Map<String, Object> command) {
        commands.add(command);
        return this;
    }

    public Transaction addIntent(Map<String, Object> intent) {
        intents.add(intent);
        return this;
    }

    private int nextIndex() {
        return commandIndex++;
    }

    private Map<String, Object> makeTransactionResult(int index, Object value) {
        Map<String, Object> result = new HashMap<>();
        Map<String, Object> returnData = new HashMap<>();
        returnData.put("index", index);
        returnData.put("use", value);
        result.put("Return", returnData);
        return result;
    }

    public Transaction moveCall(String target, List<Object> arguments, List<String> typeArguments) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> moveCallData = new HashMap<>();
        moveCallData.put("target", target);
        moveCallData.put("arguments", arguments);
        moveCallData.put("typeArguments", typeArguments);
        command.put("MoveCall", moveCallData);
        return addCommand(command);
    }

    public Transaction transferObjects(List<Object> objects, String address) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> transferData = new HashMap<>();
        transferData.put("objects", objects);
        transferData.put("address", address);
        command.put("TransferObjects", transferData);
        return addCommand(command);
    }

    public Transaction splitCoins(Object coin, List<BigInteger> amounts) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> splitData = new HashMap<>();
        splitData.put("coin", coin);
        List<Map<String, String>> amountList = new ArrayList<>();
        for (BigInteger amount : amounts) {
            Map<String, String> amountData = new HashMap<>();
            amountData.put("address", "0x2::sui::SUI");
            amountData.put("value", amount.toString());
            amountList.add(amountData);
        }
        splitData.put("amounts", amountList);
        command.put("SplitCoins", splitData);
        return addCommand(command);
    }

    public Transaction mergeCoins(Object destination, List<Object> sources) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> mergeData = new HashMap<>();
        mergeData.put("destination", destination);
        mergeData.put("sources", sources);
        command.put("MergeCoins", mergeData);
        return addCommand(command);
    }

    public Transaction publish(List<String> modules, List<String> dependencies) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> publishData = new HashMap<>();
        publishData.put("modules", modules);
        if (dependencies != null) {
            publishData.put("dependencies", dependencies);
        }
        command.put("Publish", publishData);
        return addCommand(command);
    }

    public Transaction upgrade(String packageId, List<String> modules, List<String> dependencies,
                               String ticket, int policy) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> upgradeData = new HashMap<>();
        upgradeData.put("package", packageId);
        upgradeData.put("modules", modules);
        upgradeData.put("dependencies", dependencies != null ? dependencies : List.of());
        upgradeData.put("ticket", ticket);
        upgradeData.put("policy", policy);
        command.put("Upgrade", upgradeData);
        return addCommand(command);
    }

    public Transaction makeMoveVec(String type, List<Object> objects) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> makeVecData = new HashMap<>();
        makeVecData.put("type", type);
        makeVecData.put("objects", objects);
        command.put("MakeMoveVec", makeVecData);
        return addCommand(command);
    }

    public Transaction transferSui(BigInteger amount, String address) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> transferData = new HashMap<>();
        transferData.put("amount", amount);
        transferData.put("address", address);
        command.put("TransferSui", transferData);
        return addCommand(command);
    }

    public Transaction stakeCoin(String coin, String validator) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> stakeData = new HashMap<>();
        stakeData.put("coin", coin);
        stakeData.put("validator", validator);
        command.put("StakeCoin", stakeData);
        return addCommand(command);
    }

    public Transaction unstakeCoin(String coin) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> unstakeData = new HashMap<>();
        unstakeData.put("coin", coin);
        command.put("UnstakeCoin", unstakeData);
        return addCommand(command);
    }

    public Transaction requestAddStake(String coin, String validator) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> stakeData = new HashMap<>();
        stakeData.put("coin", coin);
        stakeData.put("validator", validator);
        command.put("RequestAddStake", stakeData);
        return addCommand(command);
    }

    public Transaction requestWithdrawStake(String coin) {
        Map<String, Object> command = new HashMap<>();
        Map<String, Object> withdrawData = new HashMap<>();
        withdrawData.put("coin", coin);
        command.put("RequestWithdrawStake", withdrawData);
        return addCommand(command);
    }

    public Transaction addIntentResolver(String intentName, IntentResolver resolver) {
        return this;
    }

    public Transaction addIntent(String intentName, Map<String, Object> inputs, Map<String, Object> data) {
        Map<String, Object> intent = new HashMap<>();
        Map<String, Object> intentData = new HashMap<>();
        intentData.put("name", intentName);
        intentData.put("inputs", inputs);
        intentData.put("data", data);
        intent.put("Intent", intentData);
        return addIntent(intent);
    }

    public byte[] serialize() {
        return new byte[0];
    }

    public static byte[] serialize(Transaction tx) {
        return tx.serialize();
    }

    public static class GasData {
        private BigInteger budget;
        private BigInteger price;
        private List<String> payment;

        public GasData(BigInteger budget, BigInteger price, List<String> payment) {
            this.budget = budget;
            this.price = price;
            this.payment = payment;
        }

        public BigInteger getBudget() {
            return budget;
        }

        public BigInteger getPrice() {
            return price;
        }

        public List<String> getPayment() {
            return payment;
        }
    }

    public interface IntentResolver {
        void resolve(Map<String, Object> transactionData);
    }
}