"""Shared helpers for walrus package."""

from __future__ import annotations

import base64
import hashlib
import os
from typing import Dict, Tuple

from .types import BlobMetadataV1


def blob_id_from_int(blob_id: int) -> str:
    raw = int(blob_id).to_bytes(32, byteorder="little", signed=False)
    return base64.urlsafe_b64encode(raw).decode("ascii").rstrip("=")


def blob_id_to_int(blob_id: str) -> int:
    pad = "=" * ((4 - len(blob_id) % 4) % 4)
    raw = base64.urlsafe_b64decode(blob_id + pad)
    if len(raw) != 32:
        raise ValueError("blob_id must decode to 32 bytes")
    return int.from_bytes(raw, byteorder="little", signed=False)


def compute_blob_metadata(blob: bytes, encoding_type: str = "RedStuff") -> Tuple[str, bytes, BlobMetadataV1, bytes]:
    digest = hashlib.sha256(blob).digest()
    blob_id = blob_id_from_int(int.from_bytes(digest, byteorder="little", signed=False))
    root_hash = hashlib.sha256(b"walrus-root:" + blob).digest()
    metadata = BlobMetadataV1(
        encoding_type=encoding_type,
        unencoded_length=len(blob),
        hashes=[{"primary_hash": digest.hex(), "secondary_hash": None}],
    )
    nonce = os.urandom(32)
    return blob_id, root_hash, metadata, nonce


def merge_headers(*parts: Dict[str, str] | None) -> Dict[str, str]:
    result: Dict[str, str] = {}
    for part in parts:
        if part:
            result.update(part)
    return result
