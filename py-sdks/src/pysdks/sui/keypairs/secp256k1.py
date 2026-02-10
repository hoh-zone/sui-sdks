"""Secp256k1 keypair API (cryptography backend)."""

from __future__ import annotations

from dataclasses import dataclass

from ._cryptography_backend import ecdsa_public_key_bytes, ecdsa_sign, ecdsa_verify, generate_private_key

_CURVE = "secp256k1"


@dataclass
class Secp256k1Keypair:
    _private_key: bytes

    @classmethod
    def generate(cls) -> "Secp256k1Keypair":
        return cls(_private_key=generate_private_key())

    @classmethod
    def from_private_key_bytes(cls, private_key: bytes) -> "Secp256k1Keypair":
        if len(private_key) != 32:
            raise ValueError("private key must be 32 bytes")
        return cls(_private_key=private_key)

    def public_key_bytes(self) -> bytes:
        return ecdsa_public_key_bytes(self._private_key, _CURVE)

    def private_key_bytes(self) -> bytes:
        return self._private_key

    def sign(self, message: bytes) -> bytes:
        return ecdsa_sign(self._private_key, message, _CURVE)

    @staticmethod
    def verify_with_public_key(public_key: bytes, message: bytes, signature: bytes) -> bool:
        return ecdsa_verify(public_key, message, signature, _CURVE)
