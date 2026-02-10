"""Sui async client facade built on top of JsonRpcClient."""

from __future__ import annotations

import asyncio
from dataclasses import dataclass
from typing import Any, AsyncIterator, Dict, List, Optional

from .batch import map_async
from .jsonrpc import JsonRpcClient
from .pagination import aiter_paginated_items

_DEFAULT_COIN_TYPE = "0x2::sui::SUI"


@dataclass
class AsyncSuiClient:
    endpoint: str
    timeout_sec: float = 30.0
    _rpc_client: Optional[JsonRpcClient] = None

    @classmethod
    def from_network(cls, network: str = "testnet", timeout_sec: float = 30.0) -> "AsyncSuiClient":
        rpc = JsonRpcClient.from_network(network=network, timeout_sec=timeout_sec)
        return cls(endpoint=rpc.endpoint, timeout_sec=timeout_sec, _rpc_client=rpc)

    @property
    def rpc(self) -> JsonRpcClient:
        if self._rpc_client is None:
            self._rpc_client = JsonRpcClient(endpoint=self.endpoint, timeout_sec=self.timeout_sec)
        return self._rpc_client

    async def execute(self, method: str, params: Optional[List[Any]] = None) -> Dict[str, Any]:
        return await asyncio.to_thread(self.rpc.call, method, params or [])

    async def discover_rpc_api(self) -> Dict[str, Any]:
        return await self.execute("rpc.discover")

    async def dry_run(self, tx_bytes_b64: str) -> Dict[str, Any]:
        return await self.execute("sui_dryRunTransactionBlock", [tx_bytes_b64])

    async def get_object(self, object_id: str, options: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return await self.execute("sui_getObject", [object_id, options or {}])

    async def get_objects(
        self, object_ids: List[str], options: Optional[Dict[str, Any]] = None
    ) -> List[Dict[str, Any]]:
        return await map_async(object_ids, lambda object_id: self.get_object(object_id, options))

    async def multi_get_objects(
        self, object_ids: List[str], options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return await self.execute("sui_multiGetObjects", [object_ids, options or {}])

    async def get_events(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
        descending_order: bool = False,
    ) -> Dict[str, Any]:
        return await self.execute("suix_queryEvents", [query, cursor, limit, descending_order])

    async def iter_events(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: int = 100,
        descending_order: bool = False,
        max_items: Optional[int] = None,
    ) -> AsyncIterator[Dict[str, Any]]:
        async for item in aiter_paginated_items(
            lambda c: self.get_events(query=query, cursor=c, limit=limit, descending_order=descending_order),
            start_cursor=cursor,
            max_items=max_items,
        ):
            yield item

    async def get_package(self, package_id: str) -> Dict[str, Any]:
        return await self.get_object(
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

    async def get_gas(
        self,
        owner: str,
        coin_type: str = _DEFAULT_COIN_TYPE,
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        return await self.execute("suix_getCoins", [owner, coin_type, cursor, limit])

    async def get_all_coins(
        self, owner: str, cursor: Optional[str] = None, limit: Optional[int] = None
    ) -> Dict[str, Any]:
        return await self.execute("suix_getAllCoins", [owner, cursor, limit])

    async def iter_all_coins(
        self, owner: str, cursor: Optional[str] = None, limit: int = 100, max_items: Optional[int] = None
    ) -> AsyncIterator[Dict[str, Any]]:
        async for item in aiter_paginated_items(
            lambda c: self.get_all_coins(owner=owner, cursor=c, limit=limit),
            start_cursor=cursor,
            max_items=max_items,
        ):
            yield item

    async def get_balance(self, owner: str, coin_type: str = _DEFAULT_COIN_TYPE) -> Dict[str, Any]:
        return await self.execute("suix_getBalance", [owner, coin_type])

    async def get_all_balances(self, owner: str) -> Dict[str, Any]:
        return await self.execute("suix_getAllBalances", [owner])

    async def get_coin_metadata(self, coin_type: str) -> Dict[str, Any]:
        return await self.execute("suix_getCoinMetadata", [coin_type])

    async def get_total_supply(self, coin_type: str) -> Dict[str, Any]:
        return await self.execute("suix_getTotalSupply", [coin_type])

    async def get_owned_objects(
        self,
        owner: str,
        query: Optional[Dict[str, Any]] = None,
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        return await self.execute("suix_getOwnedObjects", [owner, query or {}, cursor, limit])

    async def get_owned_objects_legacy(
        self,
        owner: str,
        query: Optional[Dict[str, Any]] = None,
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        # Compatibility for nodes/SDKs that still expose `sui_getOwnedObjects`.
        return await self.execute("sui_getOwnedObjects", [owner, query or {}, cursor, limit])

    async def iter_owned_objects(
        self,
        owner: str,
        query: Optional[Dict[str, Any]] = None,
        cursor: Optional[str] = None,
        limit: int = 100,
        max_items: Optional[int] = None,
    ) -> AsyncIterator[Dict[str, Any]]:
        async for item in aiter_paginated_items(
            lambda c: self.get_owned_objects(owner=owner, query=query, cursor=c, limit=limit),
            start_cursor=cursor,
            max_items=max_items,
        ):
            yield item

    async def get_dynamic_fields(
        self, parent_object_id: str, cursor: Optional[str] = None, limit: Optional[int] = None
    ) -> Dict[str, Any]:
        return await self.execute("suix_getDynamicFields", [parent_object_id, cursor, limit])

    async def iter_dynamic_fields(
        self,
        parent_object_id: str,
        cursor: Optional[str] = None,
        limit: int = 100,
        max_items: Optional[int] = None,
    ) -> AsyncIterator[Dict[str, Any]]:
        async for item in aiter_paginated_items(
            lambda c: self.get_dynamic_fields(parent_object_id=parent_object_id, cursor=c, limit=limit),
            start_cursor=cursor,
            max_items=max_items,
        ):
            yield item

    async def get_dynamic_field_object(self, parent_object_id: str, name: Dict[str, Any]) -> Dict[str, Any]:
        return await self.execute("suix_getDynamicFieldObject", [parent_object_id, name])

    async def get_latest_sui_system_state(self) -> Dict[str, Any]:
        return await self.execute("suix_getLatestSuiSystemState")

    async def get_reference_gas_price(self) -> Dict[str, Any]:
        return await self.execute("suix_getReferenceGasPrice")

    async def get_latest_checkpoint_sequence_number(self) -> Dict[str, Any]:
        return await self.execute("sui_getLatestCheckpointSequenceNumber")

    async def query_transaction_blocks(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: Optional[int] = None,
        descending_order: bool = False,
    ) -> Dict[str, Any]:
        return await self.execute("suix_queryTransactionBlocks", [query, cursor, limit, descending_order])

    async def iter_transaction_blocks(
        self,
        query: Dict[str, Any],
        cursor: Optional[str] = None,
        limit: int = 100,
        descending_order: bool = False,
        max_items: Optional[int] = None,
    ) -> AsyncIterator[Dict[str, Any]]:
        async for item in aiter_paginated_items(
            lambda c: self.query_transaction_blocks(
                query=query, cursor=c, limit=limit, descending_order=descending_order
            ),
            start_cursor=cursor,
            max_items=max_items,
        ):
            yield item

    async def get_transaction_block(self, digest: str, options: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return await self.execute("sui_getTransactionBlock", [digest, options or {}])

    async def get_total_transaction_blocks(self) -> Dict[str, Any]:
        return await self.execute("sui_getTotalTransactionBlocks")

    async def multi_get_transaction_blocks(
        self, digests: List[str], options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return await self.execute("sui_multiGetTransactionBlocks", [digests, options or {}])

    async def get_events_by_transaction(self, transaction_digest: str) -> Dict[str, Any]:
        return await self.execute("sui_getEvents", [transaction_digest])

    async def get_checkpoint(self, checkpoint_id: str) -> Dict[str, Any]:
        return await self.execute("sui_getCheckpoint", [checkpoint_id])

    async def get_checkpoints(
        self, cursor: Optional[str] = None, limit: Optional[int] = None, descending_order: bool = False
    ) -> Dict[str, Any]:
        return await self.execute("sui_getCheckpoints", [cursor, limit, descending_order])

    async def iter_checkpoints(
        self,
        cursor: Optional[str] = None,
        limit: int = 100,
        descending_order: bool = False,
        max_items: Optional[int] = None,
    ) -> AsyncIterator[Dict[str, Any]]:
        async for item in aiter_paginated_items(
            lambda c: self.get_checkpoints(cursor=c, limit=limit, descending_order=descending_order),
            start_cursor=cursor,
            max_items=max_items,
        ):
            yield item

    async def get_committee_info(self, epoch: Optional[str] = None) -> Dict[str, Any]:
        return await self.execute("suix_getCommitteeInfo", [epoch])

    async def get_protocol_config(self, version: Optional[str] = None) -> Dict[str, Any]:
        return await self.execute("sui_getProtocolConfig", [version])

    async def get_chain_identifier(self) -> Dict[str, Any]:
        return await self.execute("sui_getChainIdentifier")

    async def resolve_name_service_address(self, name: str) -> Dict[str, Any]:
        return await self.execute("suix_resolveNameServiceAddress", [name])

    async def resolve_name_service_names(
        self, address: str, cursor: Optional[str] = None, limit: Optional[int] = None
    ) -> Dict[str, Any]:
        return await self.execute("suix_resolveNameServiceNames", [address, cursor, limit])

    async def get_validators_apy(self) -> Dict[str, Any]:
        return await self.execute("suix_getValidatorsApy")

    async def get_stakes(self, owner: str) -> Dict[str, Any]:
        return await self.execute("suix_getStakes", [owner])

    async def get_stakes_by_ids(self, staked_sui_ids: List[str]) -> Dict[str, Any]:
        return await self.execute("suix_getStakesByIds", [staked_sui_ids])

    async def try_get_past_object(
        self, object_id: str, version: int, options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return await self.execute("sui_tryGetPastObject", [object_id, version, options or {}])

    async def try_multi_get_past_objects(
        self, past_objects: List[Dict[str, Any]], options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        return await self.execute("sui_tryMultiGetPastObjects", [past_objects, options or {}])

    async def get_normalized_move_modules_by_package(self, package_id: str) -> Dict[str, Any]:
        return await self.execute("sui_getNormalizedMoveModulesByPackage", [package_id])

    async def get_normalized_move_module(self, package_id: str, module_name: str) -> Dict[str, Any]:
        return await self.execute("sui_getNormalizedMoveModule", [package_id, module_name])

    async def get_normalized_move_function(
        self, package_id: str, module_name: str, function_name: str
    ) -> Dict[str, Any]:
        return await self.execute("sui_getNormalizedMoveFunction", [package_id, module_name, function_name])

    async def get_move_function_arg_types(
        self, package_id: str, module_name: str, function_name: str
    ) -> Dict[str, Any]:
        return await self.execute("sui_getMoveFunctionArgTypes", [package_id, module_name, function_name])

    async def get_normalized_move_struct(
        self, package_id: str, module_name: str, struct_name: str
    ) -> Dict[str, Any]:
        return await self.execute("sui_getNormalizedMoveStruct", [package_id, module_name, struct_name])

    async def close(self) -> None:
        # JsonRpcClient uses per-request urllib and does not keep a persistent session.
        return None
