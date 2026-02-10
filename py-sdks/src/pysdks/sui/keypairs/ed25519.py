"""Ed25519 keypair API (placeholder backend)."""

from __future__ import annotations

from dataclasses import dataclass

from ._placeholder import derive_public_key, generate_private_key, sign_with_public, verify_with_public

_LABEL = b"ed25519"


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
        return derive_public_key(self._private_key, _LABEL)

    def private_key_bytes(self) -> bytes:
        return self._private_key

    def sign(self, message: bytes) -> bytes:
        return sign_with_public(self.public_key_bytes(), message, _LABEL)

    def verify(self, message: bytes, signature: bytes) -> bool:
        return verify_with_public(self.public_key_bytes(), message, signature, _LABEL)

    @staticmethod
    def verify_with_public_key(public_key: bytes, message: bytes, signature: bytes) -> bool:
        return verify_with_public(public_key, message, signature, _LABEL)
