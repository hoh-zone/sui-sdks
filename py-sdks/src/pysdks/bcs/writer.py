"""BCS byte writer."""

from __future__ import annotations

from dataclasses import dataclass, field

from .uleb import encode_uleb128


@dataclass
class BCSWriter:
    _buf: bytearray = field(default_factory=bytearray)

    def write_u8(self, value: int) -> None:
        if not 0 <= value <= 0xFF:
            raise ValueError("u8 out of range")
        self._buf.append(value)

    def write_u16(self, value: int) -> None:
        self._write_int(value, 2)

    def write_u32(self, value: int) -> None:
        self._write_int(value, 4)

    def write_u64(self, value: int) -> None:
        self._write_int(value, 8)

    def write_bool(self, value: bool) -> None:
        self.write_u8(1 if value else 0)

    def write_bytes(self, value: bytes) -> None:
        self._buf.extend(value)

    def write_uleb128(self, value: int) -> None:
        self._buf.extend(encode_uleb128(value))

    def to_bytes(self) -> bytes:
        return bytes(self._buf)

    def _write_int(self, value: int, n: int) -> None:
        if value < 0:
            raise ValueError("negative integer not allowed")
        max_value = (1 << (8 * n)) - 1
        if value > max_value:
            raise ValueError(f"u{8*n} out of range")
        self._buf.extend(value.to_bytes(n, "little"))
