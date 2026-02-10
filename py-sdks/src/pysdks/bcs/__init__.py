"""BCS utilities inspired by Rust diem/bcs semantics."""

from .uleb import MAX_ULEB128_VALUE, decode_uleb128, encode_uleb128
from .reader import BCSReader
from .writer import BCSWriter

__all__ = [
    "MAX_ULEB128_VALUE",
    "encode_uleb128",
    "decode_uleb128",
    "BCSReader",
    "BCSWriter",
]
