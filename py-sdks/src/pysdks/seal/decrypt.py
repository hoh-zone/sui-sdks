"""Seal decryption logic."""

from __future__ import annotations

from typing import Dict

from .bcs import EncryptedObjectData
from .dem import AesGcm256, Hmac256Ctr
from .error import InvalidCiphertextError
from .shamir import Share, combine


def decrypt(
    *,
    encrypted_object: EncryptedObjectData,
    keys: Dict[str, bytes],
    check_le_encoding: bool = False,
) -> bytes:
    del check_le_encoding

    full_prefix = f"{encrypted_object.package_id}:{encrypted_object.id}:"

    selected = []
    for i, (object_id, share_index) in enumerate(encrypted_object.services):
        maybe = keys.get(full_prefix + object_id)
        if maybe is not None:
            selected.append(Share(index=share_index, share=maybe))

    if len(selected) < encrypted_object.threshold:
        raise InvalidCiphertextError("not enough shares")

    base_key = combine(selected[: encrypted_object.threshold])

    if "Aes256Gcm" in encrypted_object.ciphertext:
        return AesGcm256.decrypt(base_key, encrypted_object.ciphertext)
    if "Hmac256Ctr" in encrypted_object.ciphertext:
        return Hmac256Ctr.decrypt(base_key, encrypted_object.ciphertext)
    raise InvalidCiphertextError("invalid ciphertext mode")
