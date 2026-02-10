"""ULEB128 helpers with canonical checks and u32 bound."""

from __future__ import annotations

from typing import Tuple

MAX_ULEB128_VALUE = 0xFFFFFFFF


def encode_uleb128(value: int) -> bytes:
    if value < 0:
        raise ValueError("uleb128 cannot encode negative value")
    if value == 0:
        return b"\x00"

    out = bytearray()
    v = value
    while v > 0:
        b = v & 0x7F
        v >>= 7
        if v > 0:
            b |= 0x80
        out.append(b)
    return bytes(out)


def decode_uleb128(data: bytes) -> Tuple[int, int]:
    total = 0
    shift = 0

    for i, byte in enumerate(data):
        total |= (byte & 0x7F) << shift

        if (byte & 0x80) == 0:
            consumed = i + 1
            if total > MAX_ULEB128_VALUE:
                raise ValueError("uleb128 exceeds u32 range")
            canonical = encode_uleb128(total)
            if len(canonical) != consumed:
                raise ValueError("non-canonical uleb128 encoding")
            return total, consumed

        shift += 7
        if shift >= 64:
            raise ValueError("uleb128 overflow")

    raise ValueError("uleb128 buffer overflow")
