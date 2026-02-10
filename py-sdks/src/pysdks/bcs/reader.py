"""BCS byte reader."""

from __future__ import annotations

from dataclasses import dataclass

from .uleb import decode_uleb128


@dataclass
class BCSReader:
    data: bytes
    pos: int = 0

    def remaining(self) -> int:
        return len(self.data) - self.pos

    def read_u8(self) -> int:
        if self.remaining() < 1:
            raise ValueError("bcs: out of range")
        v = self.data[self.pos]
        self.pos += 1
        return v

    def read_u16(self) -> int:
        return int.from_bytes(self.read_bytes(2), "little")

    def read_u32(self) -> int:
        return int.from_bytes(self.read_bytes(4), "little")

    def read_u64(self) -> int:
        return int.from_bytes(self.read_bytes(8), "little")

    def read_bytes(self, n: int) -> bytes:
        if n < 0 or self.remaining() < n:
            raise ValueError("bcs: out of range")
        out = self.data[self.pos : self.pos + n]
        self.pos += n
        return out

    def read_bool(self) -> bool:
        v = self.read_u8()
        if v == 0:
            return False
        if v == 1:
            return True
        raise ValueError(f"invalid bool byte: {v}")

    def read_uleb128(self) -> int:
        value, consumed = decode_uleb128(self.data[self.pos :])
        self.pos += consumed
        return value
