"""Seal client implementation."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, List, Optional

from .bcs import EncryptedObject
from .decrypt import decrypt
from .dem import AesGcm256, Hmac256Ctr
from .encrypt import DemType, KemType, encrypt
from .error import InvalidClientOptionsError, InvalidThresholdError, TooManyFailedFetchKeyRequestsError
from .key_server import FetchKeysOptions, fetch_keys_for_all_ids, retrieve_key_servers, verify_key_server
from .types import (
    BonehFranklinBLS12381DerivedKey,
    DecryptOptions,
    EncryptOptions,
    FetchKeysOptions as ClientFetchKeysOptions,
    GetDerivedKeysOptions,
    KeyServer,
    KeyServerConfig,
    SealClientOptions,
)
from .utils import count, create_full_id


@dataclass
class SealClient:
    sui_client: object
    server_configs: List[KeyServerConfig]
    verify_key_servers: bool = True
    timeout_sec: float = 10.0
    _key_servers: Optional[Dict[str, KeyServer]] = None
    _cached_keys: Dict[str, bytes] = field(default_factory=dict)

    def __post_init__(self):
        object_ids = [cfg.object_id for cfg in self.server_configs]
        if len(set(object_ids)) != len(object_ids):
            raise InvalidClientOptionsError("duplicate object ids")
        for cfg in self.server_configs:
            if bool(cfg.api_key_name) != bool(cfg.api_key):
                raise InvalidClientOptionsError("api_key_name and api_key must be set together")

    @classmethod
    def from_options(cls, options: SealClientOptions) -> "SealClient":
        return cls(
            sui_client=options.sui_client,
            server_configs=options.server_configs,
            verify_key_servers=options.verify_key_servers,
            timeout_sec=options.timeout_sec,
        )

    def get_key_servers(self) -> Dict[str, KeyServer]:
        if self._key_servers is None:
            cfg_map = {cfg.object_id: cfg for cfg in self.server_configs}
            servers = retrieve_key_servers(
                object_ids=[cfg.object_id for cfg in self.server_configs],
                client=self.sui_client,
                configs=cfg_map,
            )
            if self.verify_key_servers:
                for server in servers:
                    cfg = cfg_map[server.object_id]
                    if not verify_key_server(server, self.timeout_sec, cfg.api_key_name, cfg.api_key):
                        raise InvalidClientOptionsError(f"key server {server.object_id} verification failed")
            self._key_servers = {server.object_id: server for server in servers}
        return self._key_servers

    def _weighted_key_servers(self) -> List[KeyServer]:
        servers = self.get_key_servers()
        weighted: List[KeyServer] = []
        for cfg in self.server_configs:
            server = servers[cfg.object_id]
            weighted.extend([server] * cfg.weight)
        return weighted

    def encrypt(self, options: EncryptOptions) -> Dict[str, bytes]:
        dem = AesGcm256(options.data, options.aad) if options.dem_type == DemType.AesGcm256 else Hmac256Ctr(options.data, options.aad)
        return encrypt(
            key_servers=self._weighted_key_servers(),
            kem_type=KemType(options.kem_type),
            threshold=options.threshold,
            package_id=options.package_id,
            id=options.id,
            encryption_input=dem,
        )

    def decrypt(self, options: DecryptOptions) -> bytes:
        encrypted_object = EncryptedObject.parse(options.data)
        services = [service for service, _ in encrypted_object.services]
        total_weight = sum(cfg.weight for cfg in self.server_configs)

        if any((self._weight(service) > 0 and self._weight(service) != count(services, service)) for service in services):
            raise InvalidClientOptionsError("client key servers must be subset of encrypted services")
        if encrypted_object.threshold > total_weight:
            raise InvalidThresholdError(f"threshold {encrypted_object.threshold} exceeds total server weight {total_weight}")

        self.fetch_keys(
            ClientFetchKeysOptions(
                ids=[encrypted_object.id],
                tx_bytes=options.tx_bytes,
                session_key=options.session_key,
                threshold=encrypted_object.threshold,
            )
        )
        return decrypt(
            encrypted_object=encrypted_object,
            keys=self._cached_keys,
            check_le_encoding=options.check_le_encoding,
        )

    def fetch_keys(self, options: ClientFetchKeysOptions) -> None:
        servers = self.get_key_servers()
        if self._has_cached_threshold(options.ids, options.session_key.get_package_id(), options.threshold):
            return
        total_weight = 0
        failures = []

        for cfg in self.server_configs:
            server = servers[cfg.object_id]
            try:
                request = options.session_key.create_request_params(options.tx_bytes)
                rows = fetch_keys_for_all_ids(
                    FetchKeysOptions(
                        url=server.url,
                        request_signature=str(request["request_signature"]),
                        transaction_bytes=options.tx_bytes,
                        enc_key=request["enc_key"],
                        enc_key_pk=request["enc_key_pk"],
                        enc_verification_key=request["enc_verification_key"],
                        certificate=options.session_key.get_certificate(),
                        timeout_sec=self.timeout_sec,
                        api_key_name=cfg.api_key_name,
                        api_key=cfg.api_key,
                    )
                )

                for row in rows:
                    object_id = row["full_id"]
                    for plain_id in options.ids:
                        full = create_full_id(options.session_key.get_package_id(), plain_id)
                        cache_key = f"{full}:{object_id}"
                        self._cached_keys[cache_key] = row["key"]

                total_weight += cfg.weight
            except Exception as exc:
                failures.append(exc)

        if total_weight < options.threshold:
            raise TooManyFailedFetchKeyRequestsError(
                f"insufficient key server weight: {total_weight} < {options.threshold}, failures={len(failures)}"
            )

    def _has_cached_threshold(self, ids: List[str], package_id: str, threshold: int) -> bool:
        for object_id in ids:
            full = create_full_id(package_id, object_id)
            weight = 0
            for cfg in self.server_configs:
                if f"{full}:{cfg.object_id}" in self._cached_keys:
                    weight += cfg.weight
            if weight < threshold:
                return False
        return True

    def get_derived_keys(self, options: GetDerivedKeysOptions) -> List[BonehFranklinBLS12381DerivedKey]:
        self.fetch_keys(
            ClientFetchKeysOptions(
                ids=[options.id],
                tx_bytes=options.tx_bytes,
                session_key=options.session_key,
                threshold=options.threshold,
            )
        )
        full = create_full_id(options.session_key.get_package_id(), options.id)
        out = []
        for cfg in self.server_configs:
            cache_key = f"{full}:{cfg.object_id}"
            maybe = self._cached_keys.get(cache_key)
            if maybe is not None:
                out.append(BonehFranklinBLS12381DerivedKey(maybe))
        return out

    def _weight(self, object_id: str) -> int:
        for cfg in self.server_configs:
            if cfg.object_id == object_id:
                return cfg.weight
        return 0
