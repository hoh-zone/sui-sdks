"""Signature verification helpers."""

from __future__ import annotations

import base64
from dataclasses import dataclass

from .keypairs import SignatureScheme
from .keypairs.ed25519 import Ed25519Keypair
from .keypairs.secp256k1 import Secp256k1Keypair
from .keypairs.secp256r1 import Secp256r1Keypair

_SIGNATURE_SIZE = 64
_SCHEME_PUBKEY_SIZE = {
    SignatureScheme.ED25519: 32,
    SignatureScheme.SECP256K1: 33,
    SignatureScheme.SECP256R1: 33,
}


@dataclass
class ParsedSerializedSignature:
    scheme: SignatureScheme
    signature: bytes
    public_key: bytes


def _verifier(scheme: SignatureScheme):
    if scheme == SignatureScheme.ED25519:
        return Ed25519Keypair.verify_with_public_key
    if scheme == SignatureScheme.SECP256K1:
        return Secp256k1Keypair.verify_with_public_key
    if scheme == SignatureScheme.SECP256R1:
        return Secp256r1Keypair.verify_with_public_key
    raise ValueError(f"unsupported scheme: {scheme}")


def verify_raw_signature(message: bytes, signature: bytes, public_key: bytes, scheme: SignatureScheme) -> bool:
    return _verifier(scheme)(public_key, message, signature)


def verify_personal_message(message: bytes, signature: bytes, public_key: bytes, scheme: SignatureScheme) -> bool:
    prefixed = b"\x19Sui Signed Message:\n" + str(len(message)).encode() + b"\n" + message
    return verify_raw_signature(prefixed, signature, public_key, scheme)


def to_serialized_signature(scheme: SignatureScheme, signature: bytes, public_key: bytes) -> str:
    expected_pk_len = _SCHEME_PUBKEY_SIZE.get(scheme)
    if expected_pk_len is None:
        raise ValueError(f"unsupported scheme: {scheme}")
    if len(signature) != _SIGNATURE_SIZE:
        raise ValueError(f"invalid signature length: {len(signature)}")
    if len(public_key) != expected_pk_len:
        raise ValueError(f"invalid public key length for {scheme.name}: {len(public_key)}")

    data = bytes([int(scheme)]) + signature + public_key
    return base64.b64encode(data).decode()


def parse_serialized_signature(serialized: str) -> ParsedSerializedSignature:
    raw = base64.b64decode(serialized)
    if len(raw) < 1 + _SIGNATURE_SIZE:
        raise ValueError("serialized signature too short")

    scheme = SignatureScheme(raw[0])
    pk_size = _SCHEME_PUBKEY_SIZE.get(scheme)
    if pk_size is None:
        raise ValueError(f"unsupported scheme: {scheme}")

    expected_len = 1 + _SIGNATURE_SIZE + pk_size
    if len(raw) != expected_len:
        raise ValueError(f"invalid serialized signature length for {scheme.name}: {len(raw)}")

    signature = raw[1 : 1 + _SIGNATURE_SIZE]
    public_key = raw[1 + _SIGNATURE_SIZE :]
    return ParsedSerializedSignature(scheme=scheme, signature=signature, public_key=public_key)
