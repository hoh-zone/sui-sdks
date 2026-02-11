"""Binary container format helpers for seal package.

This is a JSON-based compatibility container for the Python SDK baseline.
"""

from __future__ import annotations

import base64
import json
from dataclasses import dataclass
from typing import Any, Dict, List, Tuple


def _b64e(data: bytes) -> str:
    return base64.b64encode(data).decode("ascii")


def _b64d(data: str) -> bytes:
    return base64.b64decode(data.encode("ascii"))


@dataclass
class EncryptedObjectData:
    version: int
    package_id: str
    id: str
    services: List[Tuple[str, int]]
    threshold: int
    encrypted_shares: Dict[str, Any]
    ciphertext: Dict[str, Any]


class EncryptedObject:
    @staticmethod
    def serialize(value: EncryptedObjectData) -> bytes:
        encoded = {
            "version": value.version,
            "packageId": value.package_id,
            "id": value.id,
            "services": [[sid, idx] for sid, idx in value.services],
            "threshold": value.threshold,
            "encryptedShares": {
                "BonehFranklinBLS12381": {
                    "nonce": _b64e(value.encrypted_shares["BonehFranklinBLS12381"]["nonce"]),
                    "encryptedShares": [
                        _b64e(x) for x in value.encrypted_shares["BonehFranklinBLS12381"]["encryptedShares"]
                    ],
                    "encryptedRandomness": _b64e(
                        value.encrypted_shares["BonehFranklinBLS12381"]["encryptedRandomness"]
                    ),
                }
            },
            "ciphertext": value.ciphertext,
        }
        return json.dumps(encoded, separators=(",", ":")).encode("utf-8")

    @staticmethod
    def parse(data: bytes) -> EncryptedObjectData:
        payload = json.loads(data.decode("utf-8"))
        mode = payload["encryptedShares"]["BonehFranklinBLS12381"]
        return EncryptedObjectData(
            version=int(payload["version"]),
            package_id=str(payload["packageId"]),
            id=str(payload["id"]),
            services=[(str(s[0]), int(s[1])) for s in payload["services"]],
            threshold=int(payload["threshold"]),
            encrypted_shares={
                "BonehFranklinBLS12381": {
                    "nonce": _b64d(mode["nonce"]),
                    "encryptedShares": [_b64d(x) for x in mode["encryptedShares"]],
                    "encryptedRandomness": _b64d(mode["encryptedRandomness"]),
                }
            },
            ciphertext=payload["ciphertext"],
        )
