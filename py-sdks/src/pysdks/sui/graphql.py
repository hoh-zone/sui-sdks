"""Minimal GraphQL HTTP client."""

from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Any, Dict, Optional
from urllib.request import Request, urlopen


@dataclass
class GraphQLClient:
    endpoint: str
    timeout_sec: float = 30.0

    def execute(self, query: str, variables: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        payload = {"query": query, "variables": variables or {}}
        req = Request(
            self.endpoint,
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urlopen(req, timeout=self.timeout_sec) as resp:  # nosec B310
            body = resp.read()

        parsed = json.loads(body.decode("utf-8"))
        if parsed.get("errors"):
            raise RuntimeError(f"graphql error: {parsed['errors']}")
        return parsed
