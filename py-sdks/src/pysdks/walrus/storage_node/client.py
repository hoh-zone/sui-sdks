"""Walrus storage node HTTP client."""

from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Any, Dict, Optional
from urllib.error import HTTPError
from urllib.request import Request, urlopen

from ..types import BlobMetadataV1, BlobMetadataWithId
from ..utils import merge_headers
from .error import ConnectionTimeoutError, LegallyUnavailableError, NotFoundError, StorageNodeAPIError
from .types import BlobStatus, SliverType


@dataclass
class StorageNodeClient:
    timeout_sec: float = 30.0

    def get_blob_metadata(self, blob_id: str, node_url: str, timeout_sec: Optional[float] = None) -> BlobMetadataWithId:
        payload = self._request("GET", f"{node_url}/v1/blobs/{blob_id}/metadata", timeout_sec=timeout_sec)
        data = self._parse_json_or_raw(payload)
        if isinstance(data, dict) and "blob_id" in data and "metadata" in data:
            metadata = data["metadata"]
            return BlobMetadataWithId(
                blob_id=data["blob_id"],
                metadata=BlobMetadataV1(
                    encoding_type=metadata.get("encoding_type", "RedStuff"),
                    unencoded_length=int(metadata.get("unencoded_length", 0)),
                    hashes=list(metadata.get("hashes", [])),
                ),
            )
        raise StorageNodeAPIError(500, "invalid metadata payload")

    def get_blob_status(self, blob_id: str, node_url: str, timeout_sec: Optional[float] = None) -> BlobStatus:
        payload = self._request("GET", f"{node_url}/v1/blobs/{blob_id}/status", timeout_sec=timeout_sec)
        data = json.loads(payload.decode("utf-8"))
        raw = data.get("success", {}).get("data", "nonexistent")
        if raw == "nonexistent":
            return BlobStatus(type="nonexistent")
        if isinstance(raw, dict):
            if "invalid" in raw:
                return BlobStatus(type="invalid", payload=raw["invalid"])
            if "permanent" in raw:
                return BlobStatus(type="permanent", payload=raw["permanent"])
            if "deletable" in raw:
                return BlobStatus(type="deletable", payload=raw["deletable"])
        raise StorageNodeAPIError(500, "invalid status payload")

    def store_blob_metadata(
        self,
        blob_id: str,
        metadata: Dict[str, Any],
        node_url: str,
        timeout_sec: Optional[float] = None,
    ) -> Dict[str, Any]:
        body = json.dumps(metadata).encode("utf-8")
        payload = self._request(
            "PUT",
            f"{node_url}/v1/blobs/{blob_id}/metadata",
            body=body,
            headers={"Content-Type": "application/json"},
            timeout_sec=timeout_sec,
        )
        return json.loads(payload.decode("utf-8"))

    def get_sliver(
        self,
        blob_id: str,
        sliver_pair_index: int,
        sliver_type: SliverType,
        node_url: str,
        timeout_sec: Optional[float] = None,
    ) -> bytes:
        return self._request(
            "GET",
            f"{node_url}/v1/blobs/{blob_id}/slivers/{sliver_pair_index}/{sliver_type}",
            timeout_sec=timeout_sec,
        )

    def store_sliver(
        self,
        blob_id: str,
        sliver_pair_index: int,
        sliver_type: SliverType,
        sliver: bytes,
        node_url: str,
        timeout_sec: Optional[float] = None,
    ) -> Dict[str, Any]:
        payload = self._request(
            "PUT",
            f"{node_url}/v1/blobs/{blob_id}/slivers/{sliver_pair_index}/{sliver_type}",
            body=sliver,
            headers={"Content-Type": "application/octet-stream"},
            timeout_sec=timeout_sec,
        )
        return json.loads(payload.decode("utf-8"))

    def get_deletable_blob_confirmation(
        self,
        blob_id: str,
        object_id: str,
        node_url: str,
        timeout_sec: Optional[float] = None,
    ) -> Dict[str, Any]:
        payload = self._request(
            "GET",
            f"{node_url}/v1/blobs/{blob_id}/confirmation/deletable/{object_id}",
            timeout_sec=timeout_sec,
        )
        return json.loads(payload.decode("utf-8"))

    def get_permanent_blob_confirmation(
        self,
        blob_id: str,
        node_url: str,
        timeout_sec: Optional[float] = None,
    ) -> Dict[str, Any]:
        payload = self._request(
            "GET",
            f"{node_url}/v1/blobs/{blob_id}/confirmation/permanent",
            timeout_sec=timeout_sec,
        )
        return json.loads(payload.decode("utf-8"))

    def _request(
        self,
        method: str,
        url: str,
        body: Optional[bytes] = None,
        headers: Optional[Dict[str, str]] = None,
        timeout_sec: Optional[float] = None,
    ) -> bytes:
        req = Request(url, data=body, method=method, headers=merge_headers(headers))
        try:
            with urlopen(req, timeout=timeout_sec or self.timeout_sec) as resp:  # nosec B310
                return resp.read()
        except TimeoutError as exc:
            raise ConnectionTimeoutError() from exc
        except HTTPError as exc:
            body_bytes = exc.read() if hasattr(exc, "read") else b""
            message = body_bytes.decode("utf-8", errors="replace")
            if exc.code == 404:
                raise NotFoundError(exc.code, message) from exc
            if exc.code == 451:
                raise LegallyUnavailableError(exc.code, message) from exc
            raise StorageNodeAPIError(exc.code, message) from exc

    @staticmethod
    def _parse_json_or_raw(payload: bytes) -> Any:
        try:
            return json.loads(payload.decode("utf-8"))
        except Exception:
            return payload
