"""Minimal faucet helpers."""

from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Any, Dict, Optional
from urllib.request import Request, urlopen


def get_faucet_host(network: str = "testnet") -> str:
    hosts = {
        "testnet": "https://faucet.testnet.sui.io/v2/gas",
        "devnet": "https://faucet.devnet.sui.io/v2/gas",
    }
    if network not in hosts:
        raise ValueError(f"unsupported faucet network: {network}")
    return hosts[network]


class FaucetRateLimitError(RuntimeError):
    pass


@dataclass
class FaucetClient:
    endpoint: str
    timeout_sec: float = 30.0

    @classmethod
    def from_network(cls, network: str = "testnet", timeout_sec: float = 30.0) -> "FaucetClient":
        return cls(endpoint=get_faucet_host(network), timeout_sec=timeout_sec)

    def request_sui_from_faucet_v2(self, recipient: str, fixed_amount: Optional[int] = None) -> Dict[str, Any]:
        payload: Dict[str, Any] = {"FixedAmountRequest": {"recipient": recipient}}
        if fixed_amount is not None:
            payload["FixedAmountRequest"]["amount"] = fixed_amount

        req = Request(
            self.endpoint,
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        try:
            with urlopen(req, timeout=self.timeout_sec) as resp:  # nosec B310
                body = resp.read()
            return json.loads(body.decode("utf-8"))
        except Exception as exc:  # pragma: no cover - mapped in tests via mocked exception message
            if "429" in str(exc):
                raise FaucetRateLimitError(str(exc)) from exc
            raise
