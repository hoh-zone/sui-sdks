package com.suisdks.sui.transactions.intents;

import java.math.BigInteger;

public class CoinWithBalance {

    public static final String INTENT_NAME = "CoinWithBalance";
    private static final String SUI_TYPE = "0x2::sui::SUI";

    public static CoinWithBalanceIntentBuilder coinWithBalance(BigInteger balance, String type, boolean useGasCoin) {
        String coinType = "gas".equals(type) ? SUI_TYPE : type;
        return new CoinWithBalanceIntentBuilder(coinType, balance, useGasCoin);
    }

    public static CoinWithBalanceIntentBuilder coinWithBalance(BigInteger balance) {
        return coinWithBalance(balance, SUI_TYPE, true);
    }

    public static CoinWithBalanceIntentBuilder coinWithBalance(BigInteger balance, String type) {
        return coinWithBalance(balance, type, true);
    }

    public static class CoinWithBalanceIntentBuilder {

        private final String type;
        private final BigInteger balance;
        private final boolean useGasCoin;

        public CoinWithBalanceIntentBuilder(String type, BigInteger balance, boolean useGasCoin) {
            this.type = type;
            this.balance = balance;
            this.useGasCoin = useGasCoin;
        }

        public CoinWithBalanceIntent build() {
            return new CoinWithBalanceIntent(INTENT_NAME, type, balance, useGasCoin);
        }
    }

    public record CoinWithBalanceIntent(
            String name,
            String type,
            BigInteger balance,
            boolean useGasCoin
    ) {

        public static CoinWithBalanceIntent fromMap(java.util.Map<String, Object> data) {
            return new CoinWithBalanceIntent(
                    (String) data.get("name"),
                    (String) data.get("type"),
                    new BigInteger(data.get("balance").toString()),
                    (Boolean) data.getOrDefault("useGasCoin", true)
            );
        }

        public java.util.Map<String, Object> toMap() {
            java.util.Map<String, Object> map = new java.util.HashMap<>();
            map.put("name", name);
            map.put("type", type);
            map.put("balance", balance);
            map.put("useGasCoin", useGasCoin);
            return map;
        }
    }

    public static class CoinWithBalanceResolver {

        public java.util.Map<String, Object> resolveCoinBalance(
                java.util.List<CoinWithBalanceIntent> intents,
                String senderAddress
        ) {
            java.util.Set<String> coinTypes = new java.util.HashSet<>();
            java.util.Map<String, BigInteger> totalByType = new java.util.HashMap<>();

            for (CoinWithBalanceIntent intent : intents) {
                if (!"gas".equals(intent.type()) && intent.balance().compareTo(BigInteger.ZERO) > 0) {
                    coinTypes.add(intent.type());
                }
                totalByType.merge(intent.type(), intent.balance(), BigInteger::add);
            }

            java.util.Map<String, Object> result = new java.util.HashMap<>();
            result.put("coinTypes", coinTypes);
            result.put("totalByType", totalByType);
            result.put("sender", senderAddress);

            return result;
        }
    }
}