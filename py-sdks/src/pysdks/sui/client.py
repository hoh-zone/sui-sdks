"""Sui sync client facade built on top of JsonRpcClient."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, List, Optional

from .jsonrpc import JsonRpcClient

_DEFAULT_COIN_TYPE = "0x2::sui::SUI"


@dataclass
class SuiClient:
    endpoint: str
    timeout_sec: float = 30.0
    _rpc_client: Optional[JsonRpcClient] = None

    @classmethod
    def from_network(cls, network: str = "testnet", timeout_sec: float = 30.0) -> "SuiClient":
        rpc = JsonRpcClient.from_network(network=network, timeout_sec=timeout_sec)
        return cls(endpoint=rpc.endpoint, timeout_sec=timeout_sec, _rpc_client=rpc)

    @property
    def rpc(self) -> JsonRpcClient:
        if self._rpc_client is None:
            self._rpc_client = JsonRpcClient(endpoint=self.endpoint, timeout_sec=self.timeout_sec)
        return self._rpc_client

    def execute(self, method: str, params: Optional[List[Any]] = None) -> Dict[str, Any]:
        return self.rpc.call(method, params or [])

    def dry_run(self, tx_bytes_b64: str) -> Dict[str, Any]:
        return self.execute("sui_dryRunTransactionBlock", [tx_bytes_b64])

    def get_object(self, object_id: str, options: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return self.execute("sui_getObject", [object_id, options or {}])

    def get_objects(self, object_ids: List[str], options: Optional[Dict[str, Any]] = None) -> List[Dict[str, Any]]:
        return [self.get_object(object_id, options) for object_id in object_ids]

    def get_events(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
        descending_order: bool = False,
    ) -> Dict[str, Any]:
        return self.execute("suix_queryEvents", [query, cursor, limit, descending_order])

    def get_package(self, package_id: str) -> Dict[str, Any]:
        return self.get_object(
            package_id,
            {
                "showType": True,
                "showOwner": True,
                "showPreviousTransaction": True,
                "showDisplay": False,
                "showContent": True,
                "showBcs": True,
                "showStorageRebate": True,
            },
        )

    def get_gas(
        self,
        owner: str,
        coin_type: str = _DEFAULT_COIN_TYPE,
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        return self.execute("suix_getCoins", [owner, coin_type, cursor, limit])

    def close(self) -> None:
        # JsonRpcClient uses per-request urllib and does not keep a persistent session.
        return None
