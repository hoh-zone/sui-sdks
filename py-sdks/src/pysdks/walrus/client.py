"""Walrus high-level client."""

from __future__ import annotations

import hashlib
from dataclasses import dataclass
from typing import Any, Dict, Optional

from .constants import MAINNET_WALRUS_PACKAGE_CONFIG, TESTNET_WALRUS_PACKAGE_CONFIG
from .error import InconsistentBlobError, WalrusClientError
from .files import WalrusBlob, WalrusFile
from .storage_node import StorageNodeClient
from .types import BlobMetadataWithId, WalrusPackageConfig
from .utils import compute_blob_metadata


@dataclass
class WalrusClient:
    sui_client: Any
    package_config: WalrusPackageConfig
    storage_node_client: StorageNodeClient

    @classmethod
    def from_network(
        cls,
        sui_client: Any,
        network: str = "testnet",
        timeout_sec: float = 30.0,
        package_config: Optional[Dict[str, Any]] = None,
    ) -> "WalrusClient":
        if package_config is None:
            if network == "testnet":
                package_config = TESTNET_WALRUS_PACKAGE_CONFIG
            elif network == "mainnet":
                package_config = MAINNET_WALRUS_PACKAGE_CONFIG
            else:
                raise WalrusClientError(f"unsupported network: {network}")

        return cls(
            sui_client=sui_client,
            package_config=WalrusPackageConfig(
                system_object_id=package_config["system_object_id"],
                staking_pool_id=package_config["staking_pool_id"],
                exchange_ids=list(package_config.get("exchange_ids", [])),
            ),
            storage_node_client=StorageNodeClient(timeout_sec=timeout_sec),
        )

    def compute_blob_metadata(self, blob: bytes) -> Dict[str, Any]:
        blob_id, root_hash, metadata, nonce = compute_blob_metadata(blob)
        return {
            "blob_id": blob_id,
            "root_hash": root_hash,
            "metadata": metadata,
            "nonce": nonce,
            "blob_digest": hashlib.sha256(blob).digest(),
        }

    def get_blob_metadata(self, blob_id: str, node_url: str) -> BlobMetadataWithId:
        return self.storage_node_client.get_blob_metadata(blob_id=blob_id, node_url=node_url)

    def get_blob_status(self, blob_id: str, node_url: str):
        return self.storage_node_client.get_blob_status(blob_id=blob_id, node_url=node_url)

    def read_blob(self, blob_id: str, node_url: str) -> bytes:
        metadata = self.get_blob_metadata(blob_id=blob_id, node_url=node_url)
        data = self.storage_node_client.get_sliver(
            blob_id=blob_id,
            sliver_pair_index=0,
            sliver_type="primary",
            node_url=node_url,
        )
        if metadata.metadata.unencoded_length != len(data):
            raise InconsistentBlobError("metadata length does not match sliver length")
        return data

    def get_blob(self, blob_id: str, node_url: str) -> WalrusBlob:
        return WalrusBlob(blob_id=blob_id, data=self.read_blob(blob_id=blob_id, node_url=node_url))

    def get_file(self, blob_id: str, node_url: str, identifier: Optional[str] = None) -> WalrusFile:
        data = self.read_blob(blob_id=blob_id, node_url=node_url)
        return WalrusFile.from_bytes(data, identifier=identifier or blob_id)
