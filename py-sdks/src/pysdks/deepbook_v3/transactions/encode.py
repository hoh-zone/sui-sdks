"""Encoding helpers used by DeepBook transaction builders."""

from __future__ import annotations

from typing import Iterable

from pysdks.bcs import BCSWriter


def encode_bool(value: bool) -> bytes:
    return b"\x01" if value else b"\x00"


def encode_u64(value: int) -> bytes:
    w = BCSWriter()
    w.write_u64(value)
    return w.to_bytes()


def encode_u128(value: int | str) -> bytes:
    n = int(value)
    if n < 0 or n >= 1 << 128:
        raise ValueError("u128 out of range")
    return n.to_bytes(16, "little")


def encode_vec_u128(values: Iterable[int | str]) -> bytes:
    values = list(values)
    w = BCSWriter()
    w.write_uleb128(len(values))
    for value in values:
        w.write_bytes(encode_u128(value))
    return w.to_bytes()


def encode_address(value: str) -> bytes:
    text = value[2:] if value.startswith("0x") else value
    if len(text) > 64:
        raise ValueError("address out of range")
    try:
        return bytes.fromhex(text.zfill(64))
    except ValueError as e:
        raise ValueError("invalid address") from e
