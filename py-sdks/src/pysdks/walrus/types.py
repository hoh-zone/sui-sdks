"""Type definitions for walrus package."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, List, Literal, Optional

EncodingType = Literal["RedStuff", "RS2"]
SliverType = Literal["primary", "secondary"]


@dataclass
class WalrusPackageConfig:
    system_object_id: str
    staking_pool_id: str
    exchange_ids: List[str] = field(default_factory=list)


@dataclass
class BlobMetadataV1:
    encoding_type: EncodingType
    unencoded_length: int
    hashes: List[Dict[str, Any]] = field(default_factory=list)


@dataclass
class BlobMetadataWithId:
    blob_id: str
    metadata: BlobMetadataV1


@dataclass
class StorageConfirmation:
    serialized_message: str
    signature: str


@dataclass
class BlobStatus:
    type: Literal["nonexistent", "invalid", "permanent", "deletable"]
    payload: Optional[Dict[str, Any]] = None
