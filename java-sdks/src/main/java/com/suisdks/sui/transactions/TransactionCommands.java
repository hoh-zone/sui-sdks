package com.suisdks.sui.transactions;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public final class TransactionCommands {
    private TransactionCommands() {
    }

    public static TransactionCommand moveCall(String target, List<Object> args) {
        Map<String, Object> data = new HashMap<>();
        data.put("target", target);
        data.put("arguments", args);
        return new TransactionCommand("MoveCall", data);
    }

    public static TransactionCommand splitCoins(Object coin, List<Object> amounts) {
        Map<String, Object> data = new HashMap<>();
        data.put("coin", coin);
        data.put("amounts", amounts);
        return new TransactionCommand("SplitCoins", data);
    }
}
