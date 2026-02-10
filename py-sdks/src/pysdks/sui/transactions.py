"""Sui transaction builder and execution baseline."""

from __future__ import annotations

import base64
import json
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass, field
from threading import Lock
from typing import Any, Callable, Dict, List, Optional, Protocol

_SYSTEM_STATE_OBJECT_ID = "0x5"
_STAKE_REQUEST_TARGET = "0x3::sui_system::request_add_stake"
_UNSTAKE_REQUEST_TARGET = "0x3::sui_system::request_withdraw_stake"


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

    @staticmethod
    def transfer_objects(objects: List[Dict[str, Any]], address: Dict[str, Any]) -> Dict[str, Any]:
        return {"$kind": "TransferObjects", "TransferObjects": {"objects": objects, "address": address}}

    @staticmethod
    def merge_coins(destination: Dict[str, Any], sources: List[Dict[str, Any]]) -> Dict[str, Any]:
        return {"$kind": "MergeCoins", "MergeCoins": {"destination": destination, "sources": sources}}

    @staticmethod
    def publish(modules: List[bytes], dependencies: List[str]) -> Dict[str, Any]:
        encoded_modules = [base64.b64encode(m).decode() for m in modules]
        return {"$kind": "Publish", "Publish": {"modules": encoded_modules, "dependencies": dependencies}}

    @staticmethod
    def upgrade(modules: List[bytes], dependencies: List[str], package_id: str, ticket: Dict[str, Any]) -> Dict[str, Any]:
        encoded_modules = [base64.b64encode(m).decode() for m in modules]
        return {
            "$kind": "Upgrade",
            "Upgrade": {
                "modules": encoded_modules,
                "dependencies": dependencies,
                "package": package_id,
                "ticket": ticket,
            },
        }

    @staticmethod
    def make_move_vec(type_tag: Optional[str], elements: List[Dict[str, Any]]) -> Dict[str, Any]:
        return {"$kind": "MakeMoveVec", "MakeMoveVec": {"type": type_tag, "elements": elements}}


@dataclass
class TransactionData:
    sender: str = ""
    expiration: Any = None
    gas_data: Dict[str, Any] = field(default_factory=dict)
    inputs: List[Dict[str, Any]] = field(default_factory=list)
    commands: List[Dict[str, Any]] = field(default_factory=list)


class Transaction:
    def __init__(self, client: Any = None):
        self.data = TransactionData(gas_data={})
        self.client = client

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

    def set_expiration(self, expiration: Any) -> None:
        self.data.expiration = expiration

    def set_gas_price(self, price: int) -> None:
        self.data.gas_data["price"] = str(price)

    def set_gas_owner(self, owner: str) -> None:
        self.data.gas_data["owner"] = owner

    def set_gas_payment(self, payments: List[Dict[str, Any]]) -> None:
        self.data.gas_data["payment"] = payments

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

    def transfer_objects(self, objects: List[Dict[str, Any]], address: Dict[str, Any]) -> Dict[str, Any]:
        return self.add_command(TransactionCommands.transfer_objects(objects, address))

    def split_coins(self, coin: Dict[str, Any], amounts: List[Dict[str, Any]]) -> Dict[str, Any]:
        return self.add_command(TransactionCommands.split_coins(coin, amounts))

    def merge_coins(self, destination: Dict[str, Any], sources: List[Dict[str, Any]]) -> Dict[str, Any]:
        return self.add_command(TransactionCommands.merge_coins(destination, sources))

    def publish(self, modules: List[bytes], dependencies: List[str]) -> Dict[str, Any]:
        return self.add_command(TransactionCommands.publish(modules, dependencies))

    def upgrade(
        self,
        modules: List[bytes],
        dependencies: List[str],
        package_id: str,
        ticket: Dict[str, Any],
    ) -> Dict[str, Any]:
        return self.add_command(TransactionCommands.upgrade(modules, dependencies, package_id, ticket))

    def publish_upgrade(
        self,
        modules: List[bytes],
        dependencies: List[str],
        package_id: str,
        ticket: Dict[str, Any],
    ) -> Dict[str, Any]:
        return self.upgrade(modules=modules, dependencies=dependencies, package_id=package_id, ticket=ticket)

    def custom_upgrade(
        self,
        modules: List[bytes],
        dependencies: List[str],
        package_id: str,
        ticket: Dict[str, Any],
    ) -> Dict[str, Any]:
        return self.upgrade(modules=modules, dependencies=dependencies, package_id=package_id, ticket=ticket)

    def make_move_vec(self, type_tag: Optional[str], elements: List[Dict[str, Any]]) -> Dict[str, Any]:
        return self.add_command(TransactionCommands.make_move_vec(type_tag, elements))

    def transfer_sui(self, recipient: str, amount: int) -> Dict[str, Any]:
        split_result = self.split_coins(self.gas(), [self.pure(self._u64_bytes(amount))])
        return self.transfer_objects([split_result], self.pure(recipient.encode("utf-8")))

    def split_coin_equal(self, coin: Dict[str, Any], split_count: int, amount_per_split: int) -> Dict[str, Any]:
        if split_count <= 0:
            raise ValueError("split_count must be positive")
        amounts = [self.pure(self._u64_bytes(amount_per_split)) for _ in range(split_count)]
        return self.split_coins(coin, amounts)

    def split_coin_and_return(self, coin: Dict[str, Any], amount: int, recipient: str) -> Dict[str, Any]:
        split_result = self.split_coins(coin, [self.pure(self._u64_bytes(amount))])
        self.transfer_objects([split_result], self.pure(recipient.encode("utf-8")))
        return split_result

    def stake_coin(
        self,
        *,
        coins: List[str | Dict[str, Any]],
        validator_address: str,
        amount: Optional[int] = None,
        system_state_object_id: str = _SYSTEM_STATE_OBJECT_ID,
    ) -> Dict[str, Any]:
        coin_args = [self.object(coin) for coin in coins]
        coins_vec = self.make_move_vec(None, coin_args)
        amount_arg = self.pure(self._option_u64_bytes(amount))
        return self.move_call(
            _STAKE_REQUEST_TARGET,
            [
                self.object(system_state_object_id),
                coins_vec,
                amount_arg,
                self.pure(validator_address.encode("utf-8")),
            ],
        )

    def unstake_coin(
        self,
        *,
        staked_coin: str | Dict[str, Any],
        system_state_object_id: str = _SYSTEM_STATE_OBJECT_ID,
    ) -> Dict[str, Any]:
        return self.move_call(
            _UNSTAKE_REQUEST_TARGET,
            [self.object(system_state_object_id), self.object(staked_coin)],
        )

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

    def get_transaction_data(self) -> Dict[str, Any]:
        return {
            "Sender": self.data.sender,
            "Expiration": self.data.expiration,
            "GasData": self.data.gas_data,
            "Inputs": self.data.inputs,
            "Commands": self.data.commands,
        }

    def deferred_execution(self) -> Dict[str, Any]:
        return {"sender": self.data.sender, "tx_bytes": self.build_base64()}

    def execute(
        self,
        client: Any = None,
        signatures: Optional[List[str]] = None,
        options: Optional[Dict[str, Any]] = None,
    ) -> Dict[str, Any]:
        active_client = self._resolve_client(client)
        tx_bytes = self.build_base64()
        if signatures is None and options is None:
            return active_client.call("sui_executeTransactionBlock", [tx_bytes])
        return active_client.call("sui_executeTransactionBlock", [tx_bytes, signatures or [], options or {}])

    def inspect_all(self, client: Any = None, sender: Optional[str] = None) -> Dict[str, Any]:
        active_client = self._resolve_client(client)
        active_sender = sender or self.data.sender
        return active_client.call("sui_devInspectTransactionBlock", [active_sender, self.build_base64()])

    def inspect_for_cost(self, client: Any = None, sender: Optional[str] = None) -> Dict[str, int]:
        result = self.inspect_all(client=client, sender=sender)
        summary = ((result or {}).get("result") or {}).get("effects", {}).get("gasUsed", {})
        computation = int(summary.get("computationCost", 0))
        storage = int(summary.get("storageCost", 0))
        rebate = int(summary.get("storageRebate", 0))
        return {
            "computation_cost": computation,
            "storage_cost": storage,
            "storage_rebate": rebate,
            "total_cost": computation + storage - rebate,
        }

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

    def _resolve_client(self, client: Any = None) -> Any:
        active_client = client or self.client
        if active_client is None:
            raise ValueError("client is required for execution and inspection")
        return active_client

    @staticmethod
    def _u64_bytes(value: int) -> bytes:
        if value < 0:
            raise ValueError("u64 value must be non-negative")
        if value > (1 << 64) - 1:
            raise ValueError("u64 value out of range")
        return value.to_bytes(8, "little")

    @staticmethod
    def _option_u64_bytes(value: Optional[int]) -> bytes:
        if value is None:
            return b"\x00"
        return b"\x01" + Transaction._u64_bytes(value)


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
