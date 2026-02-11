"""Seal encryption logic."""

from __future__ import annotations

import os
from enum import IntEnum
from typing import Dict, List

from .bcs import EncryptedObject, EncryptedObjectData
from .dem import EncryptionInput
from .error import UserError
from .shamir import split
from .types import KeyServer
from .utils import MAX_U8


class KemType(IntEnum):
    BonehFranklinBLS12381DemCCA = 0


class DemType(IntEnum):
    AesGcm256 = 0
    Hmac256Ctr = 1


def encrypt(
    *,
    key_servers: List[KeyServer],
    kem_type: KemType,
    threshold: int,
    package_id: str,
    id: str,
    encryption_input: EncryptionInput,
) -> Dict[str, bytes]:
    if (
        threshold <= 0
        or threshold >= MAX_U8
        or len(key_servers) < threshold
        or len(key_servers) >= MAX_U8
        or kem_type != KemType.BonehFranklinBLS12381DemCCA
    ):
        raise UserError(
            f"invalid key servers or threshold {threshold} for {len(key_servers)} key servers"
        )

    base_key = encryption_input.generate_key()
    shares = split(base_key, threshold, len(key_servers))

    ciphertext = encryption_input.encrypt(base_key)

    services = [(server.object_id, shares[i].index) for i, server in enumerate(key_servers)]
    encrypted_shares = {
        "BonehFranklinBLS12381": {
            "nonce": os.urandom(96),
            "encryptedShares": [share.share for share in shares],
            "encryptedRandomness": os.urandom(32),
        }
    }

    encrypted_object = EncryptedObject.serialize(
        EncryptedObjectData(
            version=0,
            package_id=package_id,
            id=id,
            services=services,
            threshold=threshold,
            encrypted_shares=encrypted_shares,
            ciphertext=ciphertext,
        )
    )

    return {"encrypted_object": encrypted_object, "key": base_key}
