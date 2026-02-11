package com.suisdks.sui.deepbook_v3;

import java.util.Map;

public class DeepBookConfig {
    private final String network;
    private final String deepbookPackage;
    private final String address;

    public DeepBookConfig(String network, String deepbookPackage, String address) {
        this.network = network;
        this.deepbookPackage = deepbookPackage;
        this.address = address;
    }

    public static DeepBookConfig mainnet() {
        return new DeepBookConfig(
            "mainnet",
            "0x1bf2db9e6c4f647011f6091efb275e28efc0426c6c8e54908bb2dd743d4e2ec",
            "0x0"
        );
    }

    public static DeepBookConfig testnet() {
        return new DeepBookConfig(
            "testnet",
            "0x1bf2db9e6c4f647011f6091efb275e28efc0426c6c8e54908bb2dd743d4e2ec",
            "0x0"
        );
    }

    public String network() {
        return network;
    }

    public String deepbookPackage() {
        return deepbookPackage;
    }

    public String address() {
        return address;
    }
}