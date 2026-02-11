"""Seal key server helpers."""

from __future__ import annotations

import base64
import json
from dataclasses import dataclass
from typing import Dict, Iterable, List, Mapping, Optional
from urllib.request import Request, urlopen

from .error import InvalidKeyServerError
from .types import KeyServer, KeyServerConfig, SealCompatibleClient


@dataclass
class FetchKeysOptions:
    url: str
    request_signature: str
    transaction_bytes: bytes
    enc_key: bytes
    enc_key_pk: bytes
    enc_verification_key: bytes
    certificate: Dict[str, object]
    timeout_sec: float
    api_key_name: Optional[str] = None
    api_key: Optional[str] = None


def _b64e(data: bytes) -> str:
    return base64.b64encode(data).decode("ascii")


def _b64d(data: str) -> bytes:
    return base64.b64decode(data.encode("ascii"))


def retrieve_key_servers(
    *,
    object_ids: Iterable[str],
    client: SealCompatibleClient,
    configs: Mapping[str, KeyServerConfig],
) -> List[KeyServer]:
    servers: List[KeyServer] = []
    for object_id in object_ids:
        cfg = configs.get(object_id)
        if cfg is None:
            raise InvalidKeyServerError(f"missing key server config for {object_id}")
        obj = client.get_object(object_id, {"showContent": True})
        data = obj.get("data") or obj.get("result") or obj
        url = cfg.aggregator_url or data.get("url") or ""
        if not url:
            raise InvalidKeyServerError(f"key server {object_id} missing URL")
        servers.append(
            KeyServer(
                object_id=object_id,
                name=str(data.get("name") or object_id),
                url=url,
                pk=bytes.fromhex(data.get("pk_hex", "")) if data.get("pk_hex") else b"",
                weight=cfg.weight,
                server_type="Committee" if cfg.aggregator_url else "Independent",
            )
        )
    return servers


def verify_key_server(
    server: KeyServer,
    timeout_sec: float,
    api_key_name: Optional[str] = None,
    api_key: Optional[str] = None,
) -> bool:
    headers = {"Content-Type": "application/json"}
    if api_key_name and api_key:
        headers[api_key_name] = api_key
    req = Request(f"{server.url}/v1/service?service_id={server.object_id}", headers=headers, method="GET")
    with urlopen(req, timeout=timeout_sec) as resp:  # nosec B310
        return 200 <= resp.status < 300


def fetch_keys_for_all_ids(options: FetchKeysOptions) -> List[Dict[str, bytes]]:
    body = {
        "ptb": _b64e(options.transaction_bytes),
        "enc_key": _b64e(options.enc_key_pk),
        "enc_verification_key": _b64e(options.enc_verification_key),
        "request_signature": options.request_signature,
        "certificate": options.certificate,
    }
    headers = {"Content-Type": "application/json"}
    if options.api_key_name and options.api_key:
        headers[options.api_key_name] = options.api_key

    req = Request(
        f"{options.url}/v1/fetch_key",
        data=json.dumps(body).encode("utf-8"),
        method="POST",
        headers=headers,
    )

    with urlopen(req, timeout=options.timeout_sec) as resp:  # nosec B310
        payload = json.loads(resp.read().decode("utf-8"))

    rows = payload.get("success", {}).get("data") or payload.get("data") or payload
    out: List[Dict[str, bytes]] = []
    for row in rows:
        object_id = row.get("id") or row.get("objectId")
        if not object_id:
            continue
        key_b64 = row.get("key") or row.get("decryptedKey") or row.get("encryptedKey")
        if key_b64 is None:
            continue
        out.append({"full_id": str(object_id), "key": _b64d(str(key_b64))})
    return out
