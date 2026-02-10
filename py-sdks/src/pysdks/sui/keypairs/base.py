"""Keypair base types."""

from __future__ import annotations

from dataclasses import dataclass
from enum import IntEnum


class SignatureScheme(IntEnum):
    ED25519 = 0
    SECP256K1 = 1
    SECP256R1 = 2


@dataclass
class SerializedSignature:
    scheme: SignatureScheme
    signature: bytes
    public_key: bytes
