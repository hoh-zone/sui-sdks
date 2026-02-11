"""Walrus blob abstraction."""

from __future__ import annotations

from dataclasses import dataclass
from typing import List, Optional

from .file import WalrusFile


@dataclass
class WalrusBlob:
    blob_id: str
    data: bytes

    def bytes(self) -> bytes:
        return self.data

    def files(self, identifiers: Optional[List[str]] = None) -> List[WalrusFile]:
        file = WalrusFile.from_bytes(self.data, identifier=self.blob_id)
        if identifiers is None:
            return [file]
        return [file] if self.blob_id in identifiers else []
