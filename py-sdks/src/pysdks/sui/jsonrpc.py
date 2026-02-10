"""Minimal JSON-RPC client for Sui fullnode."""

from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Any, Dict, List, Optional
from urllib.request import Request, urlopen


_DEFAULT_ENDPOINTS = {
    "mainnet": "https://fullnode.mainnet.sui.io:443",
    "testnet": "https://fullnode.testnet.sui.io:443",
    "devnet": "https://fullnode.devnet.sui.io:443",
}


@dataclass
class JsonRpcClient:
    endpoint: str
    timeout_sec: float = 30.0

    @classmethod
    def from_network(cls, network: str = "testnet", timeout_sec: float = 30.0) -> "JsonRpcClient":
        endpoint = _DEFAULT_ENDPOINTS.get(network)
        if endpoint is None:
            raise ValueError(f"unsupported network: {network}")
        return cls(endpoint=endpoint, timeout_sec=timeout_sec)

    def call(self, method: str, params: Optional[List[Any]] = None) -> Dict[str, Any]:
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params or [],
        }
        req = Request(
            self.endpoint,
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urlopen(req, timeout=self.timeout_sec) as resp:  # nosec B310
            body = resp.read()

        parsed = json.loads(body.decode("utf-8"))
        if "error" in parsed:
            raise RuntimeError(f"jsonrpc error: {parsed['error']}")
        return parsed
