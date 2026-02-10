"""Multisig baseline helpers."""

from __future__ import annotations

import base64
import json
from dataclasses import dataclass
from typing import Any, List, Sequence, Tuple

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

    def total_weight(self) -> int:
        return sum(self.weights)


@dataclass
class MultisigSignature:
    signatures: List[bytes]
    bitmap: List[int]

    def validate(self) -> None:
        if len(self.signatures) != len(self.bitmap):
            raise ValueError("signatures and bitmap length mismatch")
        if len(self.signatures) > MAX_SIGNER_IN_MULTISIG:
            raise ValueError("too many signatures")
        seen = set()
        for idx in self.bitmap:
            if idx in seen:
                raise ValueError("duplicate bitmap index")
            seen.add(idx)
            if idx < 0:
                raise ValueError("bitmap index must be non-negative")

    def to_base64(self) -> str:
        self.validate()
        payload = {
            "bitmap": self.bitmap,
            "signatures": [base64.b64encode(sig).decode("utf-8") for sig in self.signatures],
        }
        return base64.b64encode(json.dumps(payload, separators=(",", ":")).encode("utf-8")).decode("utf-8")

    @staticmethod
    def from_base64(serialized: str) -> "MultisigSignature":
        raw = base64.b64decode(serialized)
        payload = json.loads(raw.decode("utf-8"))
        return MultisigSignature(
            signatures=[base64.b64decode(sig_b64) for sig_b64 in payload.get("signatures", [])],
            bitmap=[int(v) for v in payload.get("bitmap", [])],
        )


class MultisigSigner:
    def __init__(self, pubkey: MultisigPublicKey, scheme: SignatureScheme = SignatureScheme.ED25519):
        self.pubkey = pubkey
        self.scheme = scheme

    def verify(self, message: bytes, multisig: MultisigSignature) -> bool:
        try:
            multisig.validate()
        except ValueError:
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

    def is_threshold_met(self, multisig: MultisigSignature) -> bool:
        try:
            multisig.validate()
        except ValueError:
            return False

        total_weight = 0
        for idx in multisig.bitmap:
            if idx < 0 or idx >= len(self.pubkey.weights):
                return False
            total_weight += self.pubkey.weights[idx]
        return total_weight >= self.pubkey.threshold

    def build(
        self,
        message: bytes,
        indexed_signatures: Sequence[Tuple[int, bytes]],
        require_threshold: bool = True,
    ) -> MultisigSignature:
        if len(indexed_signatures) == 0:
            raise ValueError("no signatures provided")
        bitmap = [int(idx) for idx, _ in indexed_signatures]
        signatures = [sig for _, sig in indexed_signatures]
        multisig = MultisigSignature(signatures=signatures, bitmap=bitmap)
        multisig.validate()

        if not self.verify(message, multisig):
            raise ValueError("invalid multisig signatures")
        if require_threshold and not self.is_threshold_met(multisig):
            raise ValueError("threshold not met")
        return multisig

    def sign(
        self,
        message: bytes,
        indexed_signers: Sequence[Tuple[int, Any]],
        require_threshold: bool = True,
    ) -> MultisigSignature:
        if len(indexed_signers) == 0:
            raise ValueError("no signers provided")
        indexed_signatures: List[Tuple[int, bytes]] = []
        for idx, signer in indexed_signers:
            if not hasattr(signer, "sign"):
                raise ValueError("signer must implement sign(message: bytes) -> bytes")
            indexed_signatures.append((int(idx), signer.sign(message)))
        return self.build(message, indexed_signatures=indexed_signatures, require_threshold=require_threshold)
