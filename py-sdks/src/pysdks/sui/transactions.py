"""Sui transaction builder and execution baseline."""

from __future__ import annotations

import base64
import json
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass, field
from threading import Lock
from typing import Any, Callable, Dict, List, Optional, Protocol


@dataclass
class ObjectRef:
    object_id: str
    digest: str
    version: int


class Inputs:
    @staticmethod
    def pure(value: bytes) -> Dict[str, Any]:
        return {"$kind": "Pure", "Pure": {"bytes": base64.b64encode(value).decode()}}

    @staticmethod
    def object_ref(ref: ObjectRef) -> Dict[str, Any]:
        return {
            "$kind": "Object",
            "Object": {
                "$kind": "ImmOrOwnedObject",
                "ImmOrOwnedObject": {
                    "objectId": ref.object_id,
                    "digest": ref.digest,
                    "version": ref.version,
                },
            },
        }

    @staticmethod
    def shared_object_ref(object_id: str, mutable: bool, initial_shared_version: int) -> Dict[str, Any]:
        return {
            "$kind": "Object",
            "Object": {
                "$kind": "SharedObject",
                "SharedObject": {
                    "objectId": object_id,
                    "mutable": mutable,
                    "initialSharedVersion": initial_shared_version,
                },
            },
        }


class TransactionCommands:
    @staticmethod
    def move_call(target: str, args: List[Dict[str, Any]], type_args: Optional[List[str]] = None) -> Dict[str, Any]:
        pkg, mod, fn = (target.split("::") + ["", "", ""])[:3]
        return {
            "$kind": "MoveCall",
            "MoveCall": {
                "package": pkg,
                "module": mod,
                "function": fn,
                "arguments": args,
                "typeArguments": type_args or [],
            },
        }

    @staticmethod
    def split_coins(coin: Dict[str, Any], amounts: List[Dict[str, Any]]) -> Dict[str, Any]:
        return {"$kind": "SplitCoins", "SplitCoins": {"coin": coin, "amounts": amounts}}


@dataclass
class TransactionData:
    sender: str = ""
    expiration: Any = None
    gas_data: Dict[str, Any] = field(default_factory=dict)
    inputs: List[Dict[str, Any]] = field(default_factory=list)
    commands: List[Dict[str, Any]] = field(default_factory=list)


class Transaction:
    def __init__(self):
        self.data = TransactionData(gas_data={})

    @property
    def commands(self):
        return self.data.commands

    @property
    def inputs(self):
        return self.data.inputs

    def set_sender(self, sender: str) -> None:
        self.data.sender = sender

    def set_sender_if_not_set(self, sender: str) -> None:
        if not self.data.sender:
            self.set_sender(sender)

    def set_gas_budget(self, budget: int) -> None:
        self.data.gas_data["budget"] = str(budget)

    def set_gas_budget_if_not_set(self, budget: int) -> None:
        if not self.data.gas_data.get("budget"):
            self.set_gas_budget(budget)

    def gas(self) -> Dict[str, Any]:
        return {"$kind": "GasCoin", "GasCoin": True}

    def add_input(self, arg: Dict[str, Any]) -> Dict[str, Any]:
        self.data.inputs.append(arg)
        return {"$kind": "Input", "Input": len(self.data.inputs) - 1}

    def object(self, value: str | Dict[str, Any]) -> Dict[str, Any]:
        if isinstance(value, str):
            return self.add_input({"$kind": "UnresolvedObject", "UnresolvedObject": {"objectId": value}})
        if value.get("$kind") == "Input":
            return value
        return self.add_input(value)

    def pure(self, value: bytes) -> Dict[str, Any]:
        return self.add_input(Inputs.pure(value))

    def add_command(self, cmd: Dict[str, Any]) -> Dict[str, Any]:
        self.data.commands.append(cmd)
        return {"$kind": "Result", "Result": len(self.data.commands) - 1}

    def move_call(self, target: str, args: List[Dict[str, Any]], type_args: Optional[List[str]] = None) -> Dict[str, Any]:
        return self.add_command(TransactionCommands.move_call(target, args, type_args))

    def build(self) -> bytes:
        payload = {
            "Sender": self.data.sender,
            "Expiration": self.data.expiration,
            "GasData": self.data.gas_data,
            "Inputs": self.data.inputs,
            "Commands": self.data.commands,
        }
        return json.dumps(payload).encode()

    def build_base64(self) -> str:
        return base64.b64encode(self.build()).decode()

    def serialize(self) -> str:
        return self.build().decode()

    @classmethod
    def from_serialized(cls, serialized: str) -> "Transaction":
        raw = base64.b64decode(serialized) if not serialized.startswith("{") else serialized.encode()
        payload = json.loads(raw.decode())
        tx = cls()
        tx.data.sender = payload.get("Sender", "")
        tx.data.expiration = payload.get("Expiration")
        tx.data.gas_data = payload.get("GasData", {})
        tx.data.inputs = payload.get("Inputs", [])
        tx.data.commands = payload.get("Commands", [])
        return tx


@dataclass
class ResolveContext:
    transaction: Transaction
    unresolved_inputs: List[Dict[str, Any]] = field(default_factory=list)


class ResolvePlugin(Protocol):
    def __call__(self, context: ResolveContext) -> None:
        ...


class Resolver:
    def __init__(self):
        self._plugins: List[ResolvePlugin] = []

    def add_plugin(self, plugin: ResolvePlugin) -> None:
        self._plugins.append(plugin)

    def resolve(self, tx: Transaction) -> ResolveContext:
        context = ResolveContext(transaction=tx)
        for inp in tx.data.inputs:
            if inp.get("$kind") == "UnresolvedObject":
                context.unresolved_inputs.append(inp)
        for plugin in self._plugins:
            plugin(context)
        return context


class CachingExecutor:
    def __init__(self, client: Any):
        self.client = client
        self._cache: Dict[str, Dict[str, Any]] = {}
        self._lock = Lock()

    def execute_transaction(self, tx: Transaction) -> Dict[str, Any]:
        key = tx.build_base64()
        with self._lock:
            if key in self._cache:
                return self._cache[key]
        result = self.client.call("sui_executeTransactionBlock", [key])
        with self._lock:
            self._cache[key] = result
        return result


class SerialExecutor:
    def __init__(self, executor: CachingExecutor):
        self.executor = executor

    def execute(self, txs: List[Transaction]) -> List[Dict[str, Any]]:
        out: List[Dict[str, Any]] = []
        for tx in txs:
            out.append(self.executor.execute_transaction(tx))
        return out


class ParallelExecutor:
    def __init__(self, executor: CachingExecutor, max_workers: int = 4):
        self.executor = executor
        self.max_workers = max_workers

    def execute(self, txs: List[Transaction]) -> List[Dict[str, Any]]:
        with ThreadPoolExecutor(max_workers=self.max_workers) as pool:
            futures = [pool.submit(self.executor.execute_transaction, tx) for tx in txs]
            return [f.result() for f in futures]
