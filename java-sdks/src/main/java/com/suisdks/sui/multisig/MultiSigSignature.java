package com.suisdks.sui.multisig;

import java.util.List;

public record MultiSigSignature(List<byte[]> signatures, List<Integer> bitmap) {
}
