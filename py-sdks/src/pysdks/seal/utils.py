"""Seal helpers."""

from __future__ import annotations

import hashlib
from typing import Iterable, List


MAX_U8 = 255


def create_full_id(package_id: str, object_id: str) -> str:
    return f"{package_id}:{object_id}"


def flatten(parts: Iterable[bytes]) -> bytes:
    return b"".join(parts)


def xor_unchecked(left: bytes, right: bytes) -> bytes:
    return bytes((l ^ right[i]) for i, l in enumerate(left))


def sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def count(values: List[str], needle: str) -> int:
    return sum(1 for v in values if v == needle)
