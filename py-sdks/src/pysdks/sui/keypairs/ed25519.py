"""Ed25519 keypair API (cryptography backend)."""

from __future__ import annotations

from dataclasses import dataclass

from ._cryptography_backend import ed25519_public_key_bytes, ed25519_sign, ed25519_verify, generate_private_key


@dataclass
class Ed25519Keypair:
    _private_key: bytes

    @classmethod
    def generate(cls) -> "Ed25519Keypair":
        return cls(_private_key=generate_private_key())

    @classmethod
    def from_private_key_bytes(cls, private_key: bytes) -> "Ed25519Keypair":
        if len(private_key) != 32:
            raise ValueError("private key must be 32 bytes")
        return cls(_private_key=private_key)

    def public_key_bytes(self) -> bytes:
        return ed25519_public_key_bytes(self._private_key)

    def private_key_bytes(self) -> bytes:
        return self._private_key

    def sign(self, message: bytes) -> bytes:
        return ed25519_sign(self._private_key, message)

    def verify(self, message: bytes, signature: bytes) -> bool:
        return ed25519_verify(self.public_key_bytes(), message, signature)

    @staticmethod
    def verify_with_public_key(public_key: bytes, message: bytes, signature: bytes) -> bool:
        return ed25519_verify(public_key, message, signature)
