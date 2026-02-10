"""Sui sync client facade built on top of JsonRpcClient."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, Iterator, List, Optional

from .batch import map_sync
from .jsonrpc import JsonRpcClient
from .pagination import iter_paginated_items

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

    def discover_rpc_api(self) -> Dict[str, Any]:
        return self.execute("rpc.discover")

    def dry_run(self, tx_bytes_b64: str) -> Dict[str, Any]:
        return self.execute("sui_dryRunTransactionBlock", [tx_bytes_b64])

    def get_object(self, object_id: str, options: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return self.execute("sui_getObject", [object_id, options or {}])

    def get_objects(self, object_ids: List[str], options: Optional[Dict[str, Any]] = None) -> List[Dict[str, Any]]:
        return map_sync(object_ids, lambda object_id: self.get_object(object_id, options))

    def multi_get_objects(
        self, object_ids: List[str], options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return self.execute("sui_multiGetObjects", [object_ids, options or {}])

    def get_events(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
        descending_order: bool = False,
    ) -> Dict[str, Any]:
        return self.execute("suix_queryEvents", [query, cursor, limit, descending_order])

    def iter_events(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: int = 100,
        descending_order: bool = False,
        max_items: Optional[int] = None,
    ) -> Iterator[Dict[str, Any]]:
        yield from iter_paginated_items(
            lambda c: self.get_events(query=query, cursor=c, limit=limit, descending_order=descending_order),
            start_cursor=cursor,
            max_items=max_items,
        )

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

    def get_all_coins(self, owner: str, cursor: Optional[str] = None, limit: Optional[int] = None) -> Dict[str, Any]:
        return self.execute("suix_getAllCoins", [owner, cursor, limit])

    def iter_all_coins(
        self, owner: str, cursor: Optional[str] = None, limit: int = 100, max_items: Optional[int] = None
    ) -> Iterator[Dict[str, Any]]:
        yield from iter_paginated_items(
            lambda c: self.get_all_coins(owner=owner, cursor=c, limit=limit),
            start_cursor=cursor,
            max_items=max_items,
        )

    def get_balance(self, owner: str, coin_type: str = _DEFAULT_COIN_TYPE) -> Dict[str, Any]:
        return self.execute("suix_getBalance", [owner, coin_type])

    def get_all_balances(self, owner: str) -> Dict[str, Any]:
        return self.execute("suix_getAllBalances", [owner])

    def get_coin_metadata(self, coin_type: str) -> Dict[str, Any]:
        return self.execute("suix_getCoinMetadata", [coin_type])

    def get_total_supply(self, coin_type: str) -> Dict[str, Any]:
        return self.execute("suix_getTotalSupply", [coin_type])

    def get_owned_objects(
        self,
        owner: str,
        query: Optional[Dict[str, Any]] = None,
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        return self.execute("suix_getOwnedObjects", [owner, query or {}, cursor, limit])

    def get_owned_objects_legacy(
        self,
        owner: str,
        query: Optional[Dict[str, Any]] = None,
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        # Compatibility for nodes/SDKs that still expose `sui_getOwnedObjects`.
        return self.execute("sui_getOwnedObjects", [owner, query or {}, cursor, limit])

    def iter_owned_objects(
        self,
        owner: str,
        query: Optional[Dict[str, Any]] = None,
        cursor: Optional[str] = None,
        limit: int = 100,
        max_items: Optional[int] = None,
    ) -> Iterator[Dict[str, Any]]:
        yield from iter_paginated_items(
            lambda c: self.get_owned_objects(owner=owner, query=query, cursor=c, limit=limit),
            start_cursor=cursor,
            max_items=max_items,
        )

    def get_dynamic_fields(
        self, parent_object_id: str, cursor: Optional[str] = None, limit: Optional[int] = None
    ) -> Dict[str, Any]:
        return self.execute("suix_getDynamicFields", [parent_object_id, cursor, limit])

    def iter_dynamic_fields(
        self,
        parent_object_id: str,
        cursor: Optional[str] = None,
        limit: int = 100,
        max_items: Optional[int] = None,
    ) -> Iterator[Dict[str, Any]]:
        yield from iter_paginated_items(
            lambda c: self.get_dynamic_fields(parent_object_id=parent_object_id, cursor=c, limit=limit),
            start_cursor=cursor,
            max_items=max_items,
        )

    def get_dynamic_field_object(self, parent_object_id: str, name: Dict[str, Any]) -> Dict[str, Any]:
        return self.execute("suix_getDynamicFieldObject", [parent_object_id, name])

    def get_latest_sui_system_state(self) -> Dict[str, Any]:
        return self.execute("suix_getLatestSuiSystemState")

    def get_reference_gas_price(self) -> Dict[str, Any]:
        return self.execute("suix_getReferenceGasPrice")

    def get_latest_checkpoint_sequence_number(self) -> Dict[str, Any]:
        return self.execute("sui_getLatestCheckpointSequenceNumber")

    def query_transaction_blocks(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
        descending_order: bool = False,
    ) -> Dict[str, Any]:
        return self.execute("suix_queryTransactionBlocks", [query, cursor, limit, descending_order])

    def iter_transaction_blocks(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: int = 100,
        descending_order: bool = False,
        max_items: Optional[int] = None,
    ) -> Iterator[Dict[str, Any]]:
        yield from iter_paginated_items(
            lambda c: self.query_transaction_blocks(
                query=query, cursor=c, limit=limit, descending_order=descending_order
            ),
            start_cursor=cursor,
            max_items=max_items,
        )

    def get_transaction_block(self, digest: str, options: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return self.execute("sui_getTransactionBlock", [digest, options or {}])

    def get_total_transaction_blocks(self) -> Dict[str, Any]:
        return self.execute("sui_getTotalTransactionBlocks")

    def multi_get_transaction_blocks(
        self, digests: List[str], options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return self.execute("sui_multiGetTransactionBlocks", [digests, options or {}])

    def get_events_by_transaction(self, transaction_digest: str) -> Dict[str, Any]:
        return self.execute("sui_getEvents", [transaction_digest])

    def get_checkpoint(self, checkpoint_id: str) -> Dict[str, Any]:
        return self.execute("sui_getCheckpoint", [checkpoint_id])

    def get_checkpoints(
        self, cursor: Optional[str] = None, limit: Optional[int] = None, descending_order: bool = False
    ) -> Dict[str, Any]:
        return self.execute("sui_getCheckpoints", [cursor, limit, descending_order])

    def iter_checkpoints(
        self,
        cursor: Optional[str] = None,
        limit: int = 100,
        descending_order: bool = False,
        max_items: Optional[int] = None,
    ) -> Iterator[Dict[str, Any]]:
        yield from iter_paginated_items(
            lambda c: self.get_checkpoints(cursor=c, limit=limit, descending_order=descending_order),
            start_cursor=cursor,
            max_items=max_items,
        )

    def get_committee_info(self, epoch: Optional[str] = None) -> Dict[str, Any]:
        return self.execute("suix_getCommitteeInfo", [epoch])

    def get_protocol_config(self, version: Optional[str] = None) -> Dict[str, Any]:
        return self.execute("sui_getProtocolConfig", [version])

    def get_chain_identifier(self) -> Dict[str, Any]:
        return self.execute("sui_getChainIdentifier")

    def resolve_name_service_address(self, name: str) -> Dict[str, Any]:
        return self.execute("suix_resolveNameServiceAddress", [name])

    def resolve_name_service_names(
        self, address: str, cursor: Optional[str] = None, limit: Optional[int] = None
    ) -> Dict[str, Any]:
        return self.execute("suix_resolveNameServiceNames", [address, cursor, limit])

    def get_validators_apy(self) -> Dict[str, Any]:
        return self.execute("suix_getValidatorsApy")

    def get_stakes(self, owner: str) -> Dict[str, Any]:
        return self.execute("suix_getStakes", [owner])

    def get_stakes_by_ids(self, staked_sui_ids: List[str]) -> Dict[str, Any]:
        return self.execute("suix_getStakesByIds", [staked_sui_ids])

    def try_get_past_object(
        self, object_id: str, version: int, options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return self.execute("sui_tryGetPastObject", [object_id, version, options or {}])

    def try_multi_get_past_objects(
        self, past_objects: List[Dict[str, Any]], options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return self.execute("sui_tryMultiGetPastObjects", [past_objects, options or {}])

    def get_normalized_move_modules_by_package(self, package_id: str) -> Dict[str, Any]:
        return self.execute("sui_getNormalizedMoveModulesByPackage", [package_id])

    def get_normalized_move_module(self, package_id: str, module_name: str) -> Dict[str, Any]:
        return self.execute("sui_getNormalizedMoveModule", [package_id, module_name])

    def get_normalized_move_function(
        self, package_id: str, module_name: str, function_name: str
    ) -> Dict[str, Any]:
        return self.execute("sui_getNormalizedMoveFunction", [package_id, module_name, function_name])

    def get_move_function_arg_types(
        self, package_id: str, module_name: str, function_name: str
    ) -> Dict[str, Any]:
        return self.execute("sui_getMoveFunctionArgTypes", [package_id, module_name, function_name])

    def get_normalized_move_struct(self, package_id: str, module_name: str, struct_name: str) -> Dict[str, Any]:
        return self.execute("sui_getNormalizedMoveStruct", [package_id, module_name, struct_name])

    def close(self) -> None:
        # JsonRpcClient uses per-request urllib and does not keep a persistent session.
        return None
