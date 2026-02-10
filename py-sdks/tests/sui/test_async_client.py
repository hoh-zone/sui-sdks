import unittest

from pysdks.sui.async_client import AsyncSuiClient


class _MockJsonRpcClient:
    def __init__(self):
        self.calls = []

    def call(self, method, params=None):
        self.calls.append((method, params or []))
        return {"jsonrpc": "2.0", "id": 1, "result": {"method": method, "params": params or []}}


class TestAsyncSuiClient(unittest.IsolatedAsyncioTestCase):
    async def test_discover_rpc_api(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.discover_rpc_api()

        self.assertEqual("rpc.discover", mock.calls[0][0])
        self.assertEqual([], mock.calls[0][1])

    async def test_execute_and_dry_run(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.execute("sui_getLatestCheckpointSequenceNumber")
        await c.dry_run("AA==")

        self.assertEqual("sui_getLatestCheckpointSequenceNumber", mock.calls[0][0])
        self.assertEqual("sui_dryRunTransactionBlock", mock.calls[1][0])
        self.assertEqual(["AA=="], mock.calls[1][1])

    async def test_get_object_and_get_objects(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.get_object("0x1", {"showContent": True})
        out = await c.get_objects(["0x2", "0x3"], {"showType": True})
        await c.multi_get_objects(["0x2", "0x3"], {"showType": True})

        self.assertEqual("sui_getObject", mock.calls[0][0])
        self.assertEqual(["0x1", {"showContent": True}], mock.calls[0][1])
        self.assertEqual(2, len(out))
        self.assertEqual("0x2", mock.calls[1][1][0])
        self.assertEqual("0x3", mock.calls[2][1][0])
        self.assertEqual("sui_multiGetObjects", mock.calls[3][0])
        self.assertEqual([["0x2", "0x3"], {"showType": True}], mock.calls[3][1])

    async def test_get_events(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)
        query = {"MoveEventType": "0x2::coin::Coin"}

        await c.get_events(query=query, cursor="abc", limit=20, descending_order=True)

        self.assertEqual("suix_queryEvents", mock.calls[0][0])
        self.assertEqual([query, "abc", 20, True], mock.calls[0][1])

    async def test_get_package(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.get_package("0x2")

        self.assertEqual("sui_getObject", mock.calls[0][0])
        self.assertEqual("0x2", mock.calls[0][1][0])
        self.assertTrue(mock.calls[0][1][1]["showBcs"])
        self.assertTrue(mock.calls[0][1][1]["showContent"])

    async def test_get_gas_and_close(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.get_gas(owner="0xabc")
        await c.close()

        self.assertEqual("suix_getCoins", mock.calls[0][0])
        self.assertEqual("0xabc", mock.calls[0][1][0])
        self.assertEqual("0x2::sui::SUI", mock.calls[0][1][1])

    async def test_coin_helpers(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.get_all_coins(owner="0xabc", cursor="next", limit=10)
        await c.get_balance(owner="0xabc")
        await c.get_all_balances(owner="0xabc")
        await c.get_coin_metadata("0x2::sui::SUI")
        await c.get_total_supply("0x2::sui::SUI")

        self.assertEqual("suix_getAllCoins", mock.calls[0][0])
        self.assertEqual(["0xabc", "next", 10], mock.calls[0][1])
        self.assertEqual("suix_getBalance", mock.calls[1][0])
        self.assertEqual(["0xabc", "0x2::sui::SUI"], mock.calls[1][1])
        self.assertEqual("suix_getAllBalances", mock.calls[2][0])
        self.assertEqual(["0xabc"], mock.calls[2][1])
        self.assertEqual("suix_getCoinMetadata", mock.calls[3][0])
        self.assertEqual(["0x2::sui::SUI"], mock.calls[3][1])
        self.assertEqual("suix_getTotalSupply", mock.calls[4][0])
        self.assertEqual(["0x2::sui::SUI"], mock.calls[4][1])

    async def test_object_and_system_helpers(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.get_owned_objects("0xabc", {"filter": {"MatchAll": []}}, "cur", 5)
        await c.get_owned_objects_legacy("0xabc", {"filter": {"MatchAll": []}}, "cur", 5)
        await c.get_dynamic_fields("0xparent", "cur2", 7)
        await c.get_dynamic_field_object("0xparent", {"type": "address", "value": "0xabc"})
        await c.get_latest_sui_system_state()
        await c.get_reference_gas_price()
        await c.get_latest_checkpoint_sequence_number()

        self.assertEqual("suix_getOwnedObjects", mock.calls[0][0])
        self.assertEqual(["0xabc", {"filter": {"MatchAll": []}}, "cur", 5], mock.calls[0][1])
        self.assertEqual("sui_getOwnedObjects", mock.calls[1][0])
        self.assertEqual(["0xabc", {"filter": {"MatchAll": []}}, "cur", 5], mock.calls[1][1])
        self.assertEqual("suix_getDynamicFields", mock.calls[2][0])
        self.assertEqual(["0xparent", "cur2", 7], mock.calls[2][1])
        self.assertEqual("suix_getDynamicFieldObject", mock.calls[3][0])
        self.assertEqual(["0xparent", {"type": "address", "value": "0xabc"}], mock.calls[3][1])
        self.assertEqual("suix_getLatestSuiSystemState", mock.calls[4][0])
        self.assertEqual([], mock.calls[4][1])
        self.assertEqual("suix_getReferenceGasPrice", mock.calls[5][0])
        self.assertEqual([], mock.calls[5][1])
        self.assertEqual("sui_getLatestCheckpointSequenceNumber", mock.calls[6][0])
        self.assertEqual([], mock.calls[6][1])

    async def test_transaction_helpers(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)
        query = {"FromAddress": "0xabc"}

        await c.query_transaction_blocks(query, cursor="cursor", limit=20, descending_order=True)
        await c.get_transaction_block("digest", {"showEvents": True})

        self.assertEqual("suix_queryTransactionBlocks", mock.calls[0][0])
        self.assertEqual([query, "cursor", 20, True], mock.calls[0][1])
        self.assertEqual("sui_getTransactionBlock", mock.calls[1][0])
        self.assertEqual(["digest", {"showEvents": True}], mock.calls[1][1])

    async def test_past_objects_and_move_helpers(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)
        past_objects = [{"objectId": "0x1", "version": 12}, {"objectId": "0x2", "version": 99}]

        await c.try_get_past_object("0x1", 12, {"showContent": True})
        await c.try_multi_get_past_objects(past_objects, {"showType": True})
        await c.get_normalized_move_modules_by_package("0x2")
        await c.get_normalized_move_module("0x2", "coin")
        await c.get_normalized_move_function("0x2", "coin", "balance")
        await c.get_move_function_arg_types("0x2", "coin", "balance")
        await c.get_normalized_move_struct("0x2", "coin", "Coin")

        self.assertEqual("sui_tryGetPastObject", mock.calls[0][0])
        self.assertEqual(["0x1", 12, {"showContent": True}], mock.calls[0][1])
        self.assertEqual("sui_tryMultiGetPastObjects", mock.calls[1][0])
        self.assertEqual([past_objects, {"showType": True}], mock.calls[1][1])
        self.assertEqual("sui_getNormalizedMoveModulesByPackage", mock.calls[2][0])
        self.assertEqual(["0x2"], mock.calls[2][1])
        self.assertEqual("sui_getNormalizedMoveModule", mock.calls[3][0])
        self.assertEqual(["0x2", "coin"], mock.calls[3][1])
        self.assertEqual("sui_getNormalizedMoveFunction", mock.calls[4][0])
        self.assertEqual(["0x2", "coin", "balance"], mock.calls[4][1])
        self.assertEqual("sui_getMoveFunctionArgTypes", mock.calls[5][0])
        self.assertEqual(["0x2", "coin", "balance"], mock.calls[5][1])
        self.assertEqual("sui_getNormalizedMoveStruct", mock.calls[6][0])
        self.assertEqual(["0x2", "coin", "Coin"], mock.calls[6][1])

    async def test_network_and_checkpoint_helpers(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.get_events_by_transaction("0xdigest")
        await c.get_checkpoint("100")
        await c.get_checkpoints(cursor="99", limit=10, descending_order=True)
        await c.get_committee_info(epoch="3")
        await c.get_protocol_config(version="6")
        await c.get_chain_identifier()
        await c.multi_get_transaction_blocks(["d1", "d2"], {"showEffects": True})
        await c.get_total_transaction_blocks()

        self.assertEqual("sui_getEvents", mock.calls[0][0])
        self.assertEqual(["0xdigest"], mock.calls[0][1])
        self.assertEqual("sui_getCheckpoint", mock.calls[1][0])
        self.assertEqual(["100"], mock.calls[1][1])
        self.assertEqual("sui_getCheckpoints", mock.calls[2][0])
        self.assertEqual(["99", 10, True], mock.calls[2][1])
        self.assertEqual("suix_getCommitteeInfo", mock.calls[3][0])
        self.assertEqual(["3"], mock.calls[3][1])
        self.assertEqual("sui_getProtocolConfig", mock.calls[4][0])
        self.assertEqual(["6"], mock.calls[4][1])
        self.assertEqual("sui_getChainIdentifier", mock.calls[5][0])
        self.assertEqual([], mock.calls[5][1])
        self.assertEqual("sui_multiGetTransactionBlocks", mock.calls[6][0])
        self.assertEqual([["d1", "d2"], {"showEffects": True}], mock.calls[6][1])
        self.assertEqual("sui_getTotalTransactionBlocks", mock.calls[7][0])
        self.assertEqual([], mock.calls[7][1])

    async def test_name_service_and_validators_helpers(self):
        mock = _MockJsonRpcClient()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        await c.resolve_name_service_address("alice.sui")
        await c.resolve_name_service_names("0xabc", cursor="c1", limit=5)
        await c.get_validators_apy()
        await c.get_stakes("0xabc")
        await c.get_stakes_by_ids(["0xstake1", "0xstake2"])

        self.assertEqual("suix_resolveNameServiceAddress", mock.calls[0][0])
        self.assertEqual(["alice.sui"], mock.calls[0][1])
        self.assertEqual("suix_resolveNameServiceNames", mock.calls[1][0])
        self.assertEqual(["0xabc", "c1", 5], mock.calls[1][1])
        self.assertEqual("suix_getValidatorsApy", mock.calls[2][0])
        self.assertEqual([], mock.calls[2][1])
        self.assertEqual("suix_getStakes", mock.calls[3][0])
        self.assertEqual(["0xabc"], mock.calls[3][1])
        self.assertEqual("suix_getStakesByIds", mock.calls[4][0])
        self.assertEqual([["0xstake1", "0xstake2"]], mock.calls[4][1])

    async def test_iter_helpers(self):
        class _PagerMock:
            def __init__(self):
                self.calls = []

            def call(self, method, params=None):
                p = params or []
                self.calls.append((method, p))
                if method == "sui_getCheckpoints":
                    cursor = p[0]
                    if cursor is None:
                        return {"data": [{"sequenceNumber": "1"}], "hasNextPage": True, "nextCursor": "1"}
                    return {"data": [{"sequenceNumber": "2"}], "hasNextPage": False, "nextCursor": None}
                if method == "suix_queryEvents":
                    cursor = p[1]
                    if cursor is None:
                        return {"data": [{"id": {"txDigest": "a"}}], "hasNextPage": True, "nextCursor": "ev1"}
                    return {"data": [{"id": {"txDigest": "b"}}], "hasNextPage": False, "nextCursor": None}
                return {"data": [], "hasNextPage": False, "nextCursor": None}

        mock = _PagerMock()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        checkpoints = []
        async for item in c.iter_checkpoints(limit=1):
            checkpoints.append(item)
        events = []
        async for item in c.iter_events(query={"All": []}, limit=1):
            events.append(item)
        checkpoints_limited = []
        async for item in c.iter_checkpoints(limit=1, max_items=1):
            checkpoints_limited.append(item)
        events_limited = []
        async for item in c.iter_events(query={"All": []}, limit=1, max_items=1):
            events_limited.append(item)

        self.assertEqual(["1", "2"], [x["sequenceNumber"] for x in checkpoints])
        self.assertEqual(["a", "b"], [x["id"]["txDigest"] for x in events])
        self.assertEqual(["1"], [x["sequenceNumber"] for x in checkpoints_limited])
        self.assertEqual(["a"], [x["id"]["txDigest"] for x in events_limited])
        self.assertEqual("sui_getCheckpoints", mock.calls[0][0])
        self.assertEqual("suix_queryEvents", mock.calls[2][0])

    async def test_iter_owned_objects_and_transactions(self):
        class _PagerMock:
            def __init__(self):
                self.calls = []

            def call(self, method, params=None):
                p = params or []
                self.calls.append((method, p))
                if method == "suix_getOwnedObjects":
                    cursor = p[2]
                    if cursor is None:
                        return {"data": [{"data": {"objectId": "0x1"}}], "hasNextPage": True, "nextCursor": "o1"}
                    return {"data": [{"data": {"objectId": "0x2"}}], "hasNextPage": False, "nextCursor": None}
                if method == "suix_getAllCoins":
                    cursor = p[1]
                    if cursor is None:
                        return {"data": [{"coinObjectId": "c1"}], "hasNextPage": True, "nextCursor": "c1"}
                    return {"data": [{"coinObjectId": "c2"}], "hasNextPage": False, "nextCursor": None}
                if method == "suix_getDynamicFields":
                    cursor = p[1]
                    if cursor is None:
                        return {"data": [{"name": {"value": "f1"}}], "hasNextPage": True, "nextCursor": "f1"}
                    return {"data": [{"name": {"value": "f2"}}], "hasNextPage": False, "nextCursor": None}
                if method == "suix_queryTransactionBlocks":
                    cursor = p[1]
                    if cursor is None:
                        return {"data": [{"digest": "d1"}], "hasNextPage": True, "nextCursor": "t1"}
                    return {"data": [{"digest": "d2"}], "hasNextPage": False, "nextCursor": None}
                return {"data": [], "hasNextPage": False, "nextCursor": None}

        mock = _PagerMock()
        c = AsyncSuiClient(endpoint="https://example.invalid", _rpc_client=mock)

        owned = []
        async for item in c.iter_owned_objects(owner="0xabc", limit=1):
            owned.append(item)
        txs = []
        async for item in c.iter_transaction_blocks(query={"FromAddress": "0xabc"}, limit=1):
            txs.append(item)
        coins = []
        async for item in c.iter_all_coins(owner="0xabc", limit=1):
            coins.append(item)
        fields = []
        async for item in c.iter_dynamic_fields(parent_object_id="0xparent", limit=1):
            fields.append(item)
        owned_limited = []
        async for item in c.iter_owned_objects(owner="0xabc", limit=1, max_items=1):
            owned_limited.append(item)
        txs_limited = []
        async for item in c.iter_transaction_blocks(query={"FromAddress": "0xabc"}, limit=1, max_items=1):
            txs_limited.append(item)

        self.assertEqual(["0x1", "0x2"], [x["data"]["objectId"] for x in owned])
        self.assertEqual(["d1", "d2"], [x["digest"] for x in txs])
        self.assertEqual(["c1", "c2"], [x["coinObjectId"] for x in coins])
        self.assertEqual(["f1", "f2"], [x["name"]["value"] for x in fields])
        self.assertEqual(["0x1"], [x["data"]["objectId"] for x in owned_limited])
        self.assertEqual(["d1"], [x["digest"] for x in txs_limited])
        methods = [m for m, _ in mock.calls]
        self.assertIn("suix_getOwnedObjects", methods)
        self.assertIn("suix_queryTransactionBlocks", methods)
        self.assertIn("suix_getAllCoins", methods)
        self.assertIn("suix_getDynamicFields", methods)


if __name__ == "__main__":
    unittest.main()
