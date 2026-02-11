"""Seal type declarations."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, List, Literal, Optional, Protocol

KeyCacheKey = str


class SealCompatibleClient(Protocol):
    def get_object(self, object_id: str, options: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        ...


@dataclass
class KeyServerConfig:
    object_id: str
    weight: int
    api_key_name: Optional[str] = None
    api_key: Optional[str] = None
    aggregator_url: Optional[str] = None


@dataclass
class SealClientOptions:
    sui_client: SealCompatibleClient
    server_configs: List[KeyServerConfig]
    verify_key_servers: bool = True
    timeout_sec: float = 10.0


@dataclass
class EncryptOptions:
    threshold: int
    package_id: str
    id: str
    data: bytes
    aad: bytes = b""
    kem_type: int = 0
    dem_type: int = 0


@dataclass
class DecryptOptions:
    data: bytes
    session_key: Any
    tx_bytes: bytes
    check_share_consistency: bool = False
    check_le_encoding: bool = False


@dataclass
class FetchKeysOptions:
    ids: List[str]
    tx_bytes: bytes
    session_key: Any
    threshold: int


@dataclass
class GetDerivedKeysOptions:
    id: str
    tx_bytes: bytes
    session_key: Any
    threshold: int
    kem_type: int = 0


@dataclass
class KeyServer:
    object_id: str
    name: str
    url: str
    key_type: Literal[0] = 0
    pk: bytes = b""
    server_type: Literal["Independent", "Committee"] = "Independent"
    weight: int = 1


@dataclass
class DerivedKey:
    key: bytes

    def __str__(self) -> str:
        return self.key.hex()


@dataclass
class BonehFranklinBLS12381DerivedKey(DerivedKey):
    pass
