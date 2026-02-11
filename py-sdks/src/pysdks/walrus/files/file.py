"""Walrus file abstraction."""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from typing import Any, Dict, Optional


@dataclass
class WalrusFile:
    _contents: bytes
    identifier: Optional[str] = None
    tags: Dict[str, str] = field(default_factory=dict)

    @classmethod
    def from_bytes(cls, contents: bytes, identifier: Optional[str] = None, tags: Optional[Dict[str, str]] = None):
        return cls(_contents=contents, identifier=identifier, tags=tags or {})

    @classmethod
    def from_text(cls, contents: str, identifier: Optional[str] = None, tags: Optional[Dict[str, str]] = None):
        return cls(_contents=contents.encode("utf-8"), identifier=identifier, tags=tags or {})

    def bytes(self) -> bytes:
        return self._contents

    def text(self) -> str:
        return self._contents.decode("utf-8")

    def json(self) -> Any:
        return json.loads(self.text())

    def get_identifier(self) -> Optional[str]:
        return self.identifier

    def get_tags(self) -> Dict[str, str]:
        return dict(self.tags)
