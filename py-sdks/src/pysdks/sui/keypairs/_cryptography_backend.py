"""Cryptography-backed helpers for Sui keypairs."""

from __future__ import annotations

import secrets


def generate_private_key() -> bytes:
    return secrets.token_bytes(32)


def ed25519_public_key_bytes(private_key: bytes) -> bytes:
    ed25519, _, serialization, _, _ = _load_crypto()
    if len(private_key) != 32:
        raise ValueError("private key must be 32 bytes")
    pub = ed25519.Ed25519PrivateKey.from_private_bytes(private_key).public_key()
    return pub.public_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PublicFormat.Raw,
    )


def ed25519_sign(private_key: bytes, message: bytes) -> bytes:
    ed25519, _, _, _, _ = _load_crypto()
    if len(private_key) != 32:
        raise ValueError("private key must be 32 bytes")
    return ed25519.Ed25519PrivateKey.from_private_bytes(private_key).sign(message)


def ed25519_verify(public_key: bytes, message: bytes, signature: bytes) -> bool:
    ed25519, InvalidSignature, _, _, _ = _load_crypto()
    if len(public_key) != 32:
        return False
    try:
        ed25519.Ed25519PublicKey.from_public_bytes(public_key).verify(signature, message)
        return True
    except InvalidSignature:
        return False


def ecdsa_public_key_bytes(private_key: bytes, curve_name: str) -> bytes:
    _, _, serialization, _, _ = _load_crypto()
    key = _ecdsa_private_from_scalar(private_key, curve_name)
    pub = key.public_key()
    return pub.public_bytes(
        encoding=serialization.Encoding.X962,
        format=serialization.PublicFormat.CompressedPoint,
    )


def ecdsa_sign(private_key: bytes, message: bytes, curve_name: str) -> bytes:
    _, _, _, ec, utils = _load_crypto()
    from cryptography.hazmat.primitives import hashes  # type: ignore

    key = _ecdsa_private_from_scalar(private_key, curve_name)
    der_sig = key.sign(message, ec.ECDSA(hashes.SHA256()))
    r, s = utils.decode_dss_signature(der_sig)
    return r.to_bytes(32, "big") + s.to_bytes(32, "big")


def ecdsa_verify(public_key: bytes, message: bytes, signature: bytes, curve_name: str) -> bool:
    _, InvalidSignature, _, ec, utils = _load_crypto()
    from cryptography.hazmat.primitives import hashes  # type: ignore

    if len(public_key) != 33 or len(signature) != 64:
        return False
    curve = _curve_from_name(curve_name)
    try:
        pub = ec.EllipticCurvePublicKey.from_encoded_point(curve, public_key)
    except ValueError:
        return False
    r = int.from_bytes(signature[:32], "big")
    s = int.from_bytes(signature[32:], "big")
    der_sig = utils.encode_dss_signature(r, s)
    try:
        pub.verify(der_sig, message, ec.ECDSA(hashes.SHA256()))
        return True
    except InvalidSignature:
        return False


def _ecdsa_private_from_scalar(private_key: bytes, curve_name: str) -> ec.EllipticCurvePrivateKey:
    _, _, _, ec, _ = _load_crypto()
    if len(private_key) != 32:
        raise ValueError("private key must be 32 bytes")
    scalar = int.from_bytes(private_key, "big")
    if scalar <= 0:
        raise ValueError("private key scalar must be greater than zero")
    curve = _curve_from_name(curve_name)
    return ec.derive_private_key(scalar, curve)


def _curve_from_name(curve_name: str):
    _, _, _, ec, _ = _load_crypto()
    if curve_name == "secp256k1":
        return ec.SECP256K1()
    if curve_name == "prime256v1":
        return ec.SECP256R1()
    raise ValueError(f"unsupported curve: {curve_name}")


def _load_crypto():
    try:
        from cryptography.exceptions import InvalidSignature  # type: ignore
        from cryptography.hazmat.primitives import serialization  # type: ignore
        from cryptography.hazmat.primitives.asymmetric import ec, ed25519, utils  # type: ignore
    except ModuleNotFoundError as exc:
        raise RuntimeError(
            "cryptography dependency is required for keypair/signature operations. "
            "Install with: pip install cryptography>=42.0.0"
        ) from exc
    return ed25519, InvalidSignature, serialization, ec, utils
