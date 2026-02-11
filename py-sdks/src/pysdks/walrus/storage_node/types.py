"""Storage node request/response types."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, Literal, Optional

SliverType = Literal["primary", "secondary"]


@dataclass
class RequestOptions:
    node_url: str
    timeout_sec: float = 30.0
    headers: Optional[Dict[str, str]] = None


@dataclass
class StorageConfirmation:
    serialized_message: str
    signature: str


@dataclass
class BlobStatus:
    type: Literal["nonexistent", "invalid", "permanent", "deletable"]
    payload: Optional[Dict[str, Any]] = None
