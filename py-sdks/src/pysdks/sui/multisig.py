"""Multisig baseline helpers."""

from __future__ import annotations

from dataclasses import dataclass
from typing import List

from .keypairs import SignatureScheme
from .verify import verify_raw_signature

MAX_SIGNER_IN_MULTISIG = 10
MIN_SIGNER_IN_MULTISIG = 1


@dataclass
class MultisigPublicKey:
    public_keys: List[bytes]
    weights: List[int]
    threshold: int

    def __post_init__(self):
        if len(self.public_keys) != len(self.weights):
            raise ValueError("public_keys and weights length mismatch")
        if len(self.public_keys) < MIN_SIGNER_IN_MULTISIG:
            raise ValueError(f"min number of signers in a multisig is {MIN_SIGNER_IN_MULTISIG}")
        if len(self.public_keys) > MAX_SIGNER_IN_MULTISIG:
            raise ValueError(f"max number of signers in a multisig is {MAX_SIGNER_IN_MULTISIG}")

        if self.threshold <= 0:
            raise ValueError("invalid threshold")
        if any(w <= 0 for w in self.weights):
            raise ValueError("weights must be positive")

        total_weight = sum(self.weights)
        if self.threshold > total_weight:
            raise ValueError("unreachable threshold")


@dataclass
class MultisigSignature:
    signatures: List[bytes]
    bitmap: List[int]


class MultisigSigner:
    def __init__(self, pubkey: MultisigPublicKey, scheme: SignatureScheme = SignatureScheme.ED25519):
        self.pubkey = pubkey
        self.scheme = scheme

    def verify(self, message: bytes, multisig: MultisigSignature) -> bool:
        if len(multisig.signatures) != len(multisig.bitmap):
            return False
        if len(multisig.signatures) > MAX_SIGNER_IN_MULTISIG:
            return False

        seen = set()
        total_weight = 0

        for sig, idx in zip(multisig.signatures, multisig.bitmap):
            if idx in seen:
                return False
            seen.add(idx)
            if idx < 0 or idx >= len(self.pubkey.public_keys):
                return False

            pk = self.pubkey.public_keys[idx]
            if not verify_raw_signature(message, sig, pk, self.scheme):
                return False

            total_weight += self.pubkey.weights[idx]

        return total_weight >= self.pubkey.threshold
