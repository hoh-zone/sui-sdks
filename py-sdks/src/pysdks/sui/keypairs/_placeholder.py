"""Insecure placeholder signing backend.

This fallback exists to keep local SDK tests runnable without extra crypto deps.
It is NOT cryptographically secure and must be replaced by real crypto backends.
"""

from __future__ import annotations

import hashlib
import hmac
import secrets

_SIGNATURE_SIZE = 64


def generate_private_key() -> bytes:
    return secrets.token_bytes(32)


def derive_public_key(private_key: bytes, label: bytes, *, size: int = 32, prefix: int | None = None) -> bytes:
    if size <= 0:
        raise ValueError("public key size must be positive")

    digest = hashlib.sha256(label + private_key).digest()
    if prefix is None:
        if size <= len(digest):
            return digest[:size]
        return digest + b"\x00" * (size - len(digest))

    if not 0 <= prefix <= 0xFF:
        raise ValueError("invalid prefix")

    body_size = size - 1
    body = digest[:body_size] if body_size <= len(digest) else digest + b"\x00" * (body_size - len(digest))
    return bytes([prefix]) + body


def sign_with_public(public_key: bytes, message: bytes, label: bytes) -> bytes:
    # Keep signature length stable (64 bytes) across schemes to mirror Sui keypair APIs.
    mac1 = hmac.new(public_key, label + message, hashlib.sha256).digest()
    mac2 = hmac.new(public_key, b"\x01" + label + message, hashlib.sha256).digest()
    return mac1 + mac2


def verify_with_public(public_key: bytes, message: bytes, signature: bytes, label: bytes) -> bool:
    if len(signature) != _SIGNATURE_SIZE:
        return False

    expected = sign_with_public(public_key, message, label)
    return hmac.compare_digest(signature, expected)
