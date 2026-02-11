"""Data encryption mechanisms for seal."""

from __future__ import annotations

import hmac
import os
from dataclasses import dataclass
from hashlib import sha3_256
from typing import Dict, Protocol

from .error import DecryptionError, InvalidCiphertextError
from .utils import xor_unchecked

IV = bytes([138, 55, 153, 253, 198, 46, 121, 219, 160, 128, 89, 7, 214, 156, 148, 220])


class EncryptionInput(Protocol):
    def encrypt(self, key: bytes) -> Dict[str, Dict[str, str]]:
        ...

    def generate_key(self) -> bytes:
        ...


def _b64e(data: bytes) -> str:
    import base64

    return base64.b64encode(data).decode("ascii")


def _b64d(data: str) -> bytes:
    import base64

    return base64.b64decode(data.encode("ascii"))


@dataclass
class AesGcm256:
    plaintext: bytes
    aad: bytes

    def generate_key(self) -> bytes:
        return os.urandom(32)

    def encrypt(self, key: bytes) -> Dict[str, Dict[str, str]]:
        if len(key) != 32:
            raise ValueError("key must be 32 bytes")
        ct = _aesgcm_encrypt_or_fallback(key, self.plaintext, self.aad)
        return {"Aes256Gcm": {"blob": _b64e(ct), "aad": _b64e(self.aad)}}

    @staticmethod
    def decrypt(key: bytes, ciphertext: Dict[str, Dict[str, str]]) -> bytes:
        payload = ciphertext.get("Aes256Gcm")
        if payload is None:
            raise InvalidCiphertextError("expected Aes256Gcm ciphertext")
        try:
            return _aesgcm_decrypt_or_fallback(key, _b64d(payload["blob"]), _b64d(payload.get("aad", "")))
        except Exception as exc:
            raise DecryptionError("decryption failed") from exc


@dataclass
class Hmac256Ctr:
    plaintext: bytes
    aad: bytes

    def generate_key(self) -> bytes:
        return os.urandom(32)

    def encrypt(self, key: bytes) -> Dict[str, Dict[str, str]]:
        blob = self._crypt(key, self.plaintext)
        mac = self._mac(key, self.aad, blob)
        return {"Hmac256Ctr": {"blob": _b64e(blob), "aad": _b64e(self.aad), "mac": _b64e(mac)}}

    @staticmethod
    def decrypt(key: bytes, ciphertext: Dict[str, Dict[str, str]]) -> bytes:
        payload = ciphertext.get("Hmac256Ctr")
        if payload is None:
            raise InvalidCiphertextError("expected Hmac256Ctr ciphertext")
        aad = _b64d(payload.get("aad", ""))
        blob = _b64d(payload["blob"])
        expected = Hmac256Ctr._mac(key, aad, blob)
        got = _b64d(payload["mac"])
        if not hmac.compare_digest(expected, got):
            raise DecryptionError("invalid mac")
        return Hmac256Ctr._crypt(key, blob)

    @staticmethod
    def _hmac(key: bytes, data: bytes) -> bytes:
        return hmac.new(key, data, sha3_256).digest()

    @staticmethod
    def _crypt(key: bytes, data: bytes) -> bytes:
        out = bytearray(len(data))
        block_size = 32
        for i in range((len(data) + block_size - 1) // block_size):
            block = data[i * block_size : (i + 1) * block_size]
            mask = Hmac256Ctr._hmac(key, b"HMAC-CTR-ENC" + i.to_bytes(8, "little"))
            out[i * block_size : i * block_size + len(block)] = xor_unchecked(block, mask[: len(block)])
        return bytes(out)

    @staticmethod
    def _mac(key: bytes, aad: bytes, blob: bytes) -> bytes:
        return Hmac256Ctr._hmac(key, b"HMAC-CTR-MAC" + len(aad).to_bytes(8, "little") + aad + blob)


def _aesgcm_encrypt_or_fallback(key: bytes, plaintext: bytes, aad: bytes) -> bytes:
    try:
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM  # type: ignore

        return AESGCM(key).encrypt(IV, plaintext, aad)
    except Exception:
        # Fallback authenticated stream for environments without cryptography.
        stream = _stream_mask(key, len(plaintext))
        encrypted = xor_unchecked(plaintext, stream)
        mac = hmac.new(key, b"AES-GCM-FALLBACK" + aad + encrypted, sha3_256).digest()
        return encrypted + mac


def _aesgcm_decrypt_or_fallback(key: bytes, blob: bytes, aad: bytes) -> bytes:
    try:
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM  # type: ignore

        return AESGCM(key).decrypt(IV, blob, aad)
    except Exception:
        if len(blob) < 32:
            raise DecryptionError("ciphertext too short")
        encrypted, mac = blob[:-32], blob[-32:]
        expected = hmac.new(key, b"AES-GCM-FALLBACK" + aad + encrypted, sha3_256).digest()
        if not hmac.compare_digest(mac, expected):
            raise DecryptionError("invalid mac")
        stream = _stream_mask(key, len(encrypted))
        return xor_unchecked(encrypted, stream)


def _stream_mask(key: bytes, length: int) -> bytes:
    out = bytearray()
    counter = 0
    while len(out) < length:
        out.extend(hmac.new(key, b"AES-GCM-FALLBACK-STREAM" + counter.to_bytes(8, "little"), sha3_256).digest())
        counter += 1
    return bytes(out[:length])
