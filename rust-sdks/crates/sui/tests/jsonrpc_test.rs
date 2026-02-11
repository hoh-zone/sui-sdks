use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;
use sui::jsonrpc::{default_jsonrpc_fullnode_url, Client, JsonRpcError};

#[tokio::test]
async fn jsonrpc_call_and_get_balance() {
    let server = MockServer::start();

    let _discover = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"rpc.discover\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":{"info":{"version":"1.0.0"}}}"#);
    });

    let _balance = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"suix_getBalance\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":{"coinType":"0x2::sui::SUI","totalBalance":"123"}}"#);
    });

    let _all_balances = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"suix_getAllBalances\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":[{"coinType":"0x2::sui::SUI","totalBalance":"123"}]}"#);
    });

    let _coins = server.mock(|when, then| {
        when.method(POST).path("/").body_contains("\"method\":\"suix_getCoins\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":{"data":[],"nextCursor":null,"hasNextPage":false}}"#);
    });

    let _all_coins = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"suix_getAllCoins\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":{"data":[],"nextCursor":null,"hasNextPage":false}}"#);
    });

    let _multi_get_objects = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"sui_multiGetObjects\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":[]}"#);
    });

    let _total_supply = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"suix_getTotalSupply\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":{"value":"1000"}}"#);
    });

    let _coin_metadata = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"suix_getCoinMetadata\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":{"id":"0x2::sui::SUI","symbol":"SUI"}}"#);
    });

    let client = Client::new(server.url("/"), "testnet");
    let version = client.get_rpc_api_version().await.expect("version");
    assert_eq!(version, "1.0.0");

    let balance = client
        .get_balance(
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            None,
        )
        .await
        .expect("balance");

    assert_eq!(balance["totalBalance"], "123");

    let all_balances = client
        .get_all_balances("0x0000000000000000000000000000000000000000000000000000000000000001")
        .await
        .expect("all balances");
    assert!(all_balances.is_array());

    let coins = client
        .get_coins(
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            None,
            None,
            Some(50),
        )
        .await
        .expect("coins");
    assert!(coins.get("data").is_some());

    let all_coins = client
        .get_all_coins(
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            None,
            Some(50),
        )
        .await
        .expect("all coins");
    assert!(all_coins.get("data").is_some());

    let objects = client
        .multi_get_objects(
            vec!["0x0000000000000000000000000000000000000000000000000000000000000001".to_string()],
            None,
        )
        .await
        .expect("multi get objects");
    assert!(objects.is_array());

    let total_supply = client
        .get_total_supply("0x2::sui::SUI")
        .await
        .expect("total supply");
    assert_eq!(total_supply["value"], "1000");

    let coin_metadata = client
        .get_coin_metadata("0x2::sui::SUI")
        .await
        .expect("coin metadata");
    assert!(coin_metadata.is_object());
}

#[tokio::test]
async fn jsonrpc_batch_call_orders_by_request_id() {
    let server = MockServer::start();

    let _batch = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"[{"jsonrpc":"2.0","id":2,"result":{"b":2}},{"jsonrpc":"2.0","id":1,"result":{"a":1}}]"#,
            );
    });

    let client = Client::new(server.url("/"), "testnet");
    let results = client
        .batch_call(vec![("a_method", vec![]), ("b_method", vec![])])
        .await
        .expect("batch_call");

    assert_eq!(results.len(), 2);
    assert_eq!(results[0]["a"], 1);
    assert_eq!(results[1]["b"], 2);
}

#[tokio::test]
async fn jsonrpc_batch_call_empty() {
    let client = Client::new("http://localhost:12345", "testnet");
    let results = client.batch_call(vec![]).await.expect("empty batch");
    assert!(results.is_empty());
}

#[tokio::test]
async fn jsonrpc_extended_read_methods() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"jsonrpc":"2.0","id":1,"result":{"ok":true}}));
    });

    let client = Client::new(server.url("/"), "testnet");
    let owner = "0x0000000000000000000000000000000000000000000000000000000000000001";
    let object_id = owner;

    let r = client
        .get_owned_objects(owner, None, None, Some(10))
        .await
        .expect("get_owned_objects");
    assert_eq!(r["ok"], true);

    let r = client
        .query_events(json!({"All": []}), None, Some(10), false)
        .await
        .expect("query_events");
    assert_eq!(r["ok"], true);

    let r = client
        .query_transaction_blocks(json!({"All": []}), None, Some(10), true)
        .await
        .expect("query_transaction_blocks");
    assert_eq!(r["ok"], true);

    let r = client
        .get_transaction_block("abcd", None)
        .await
        .expect("get_transaction_block");
    assert_eq!(r["ok"], true);

    let r = client
        .multi_get_transaction_blocks(vec!["a".to_string(), "b".to_string()], None)
        .await
        .expect("multi_get_transaction_blocks");
    assert_eq!(r["ok"], true);

    let r = client.get_events("abcd").await.expect("get_events");
    assert_eq!(r["ok"], true);

    let r = client
        .get_latest_checkpoint_sequence_number()
        .await
        .expect("get_latest_checkpoint_sequence_number");
    assert_eq!(r["ok"], true);

    let r = client.get_checkpoint("1").await.expect("get_checkpoint");
    assert_eq!(r["ok"], true);

    let r = client
        .get_checkpoints(None, Some(20), false)
        .await
        .expect("get_checkpoints");
    assert_eq!(r["ok"], true);

    let r = client
        .get_protocol_config(None)
        .await
        .expect("get_protocol_config");
    assert_eq!(r["ok"], true);

    let r = client
        .get_latest_sui_system_state()
        .await
        .expect("get_latest_sui_system_state");
    assert_eq!(r["ok"], true);

    let r = client
        .get_chain_identifier()
        .await
        .expect("get_chain_identifier");
    assert_eq!(r["ok"], true);

    let r = client
        .get_committee_info(None)
        .await
        .expect("get_committee_info");
    assert_eq!(r["ok"], true);

    let r = client
        .resolve_name_service_address("alice.sui")
        .await
        .expect("resolve_name_service_address");
    assert_eq!(r["ok"], true);

    let r = client
        .resolve_name_service_names(owner, None, Some(10))
        .await
        .expect("resolve_name_service_names");
    assert_eq!(r["ok"], true);

    let r = client
        .get_validators_apy()
        .await
        .expect("get_validators_apy");
    assert_eq!(r["ok"], true);

    let r = client.get_stakes(owner).await.expect("get_stakes");
    assert_eq!(r["ok"], true);

    let r = client
        .get_stakes_by_ids(vec![object_id.to_string()])
        .await
        .expect("get_stakes_by_ids");
    assert_eq!(r["ok"], true);

    let r = client
        .dry_run_transaction_block("AA==")
        .await
        .expect("dry_run_transaction_block");
    assert_eq!(r["ok"], true);

    let r = client
        .get_dynamic_fields(object_id, None, Some(5))
        .await
        .expect("get_dynamic_fields");
    assert_eq!(r["ok"], true);

    let r = client
        .get_dynamic_field_object(object_id, json!({"type":"0x1::string::String","value":"x"}))
        .await
        .expect("get_dynamic_field_object");
    assert_eq!(r["ok"], true);

    let r = client
        .get_total_transaction_blocks()
        .await
        .expect("get_total_transaction_blocks");
    assert_eq!(r["ok"], true);

    let r = client
        .try_get_past_object(object_id, 1, None)
        .await
        .expect("try_get_past_object");
    assert_eq!(r["ok"], true);

    let r = client
        .try_multi_get_past_objects(vec![json!({"objectId": object_id, "version": "1"})], None)
        .await
        .expect("try_multi_get_past_objects");
    assert_eq!(r["ok"], true);

    let r = client
        .get_normalized_move_modules_by_package(object_id)
        .await
        .expect("get_normalized_move_modules_by_package");
    assert_eq!(r["ok"], true);

    let r = client
        .get_normalized_move_module(object_id, "m")
        .await
        .expect("get_normalized_move_module");
    assert_eq!(r["ok"], true);

    let r = client
        .get_normalized_move_function(object_id, "m", "f")
        .await
        .expect("get_normalized_move_function");
    assert_eq!(r["ok"], true);

    let r = client
        .get_move_function_arg_types(object_id, "m", "f")
        .await
        .expect("get_move_function_arg_types");
    assert_eq!(r["ok"], true);

    let r = client
        .get_normalized_move_struct(object_id, "m", "S")
        .await
        .expect("get_normalized_move_struct");
    assert_eq!(r["ok"], true);
}

#[tokio::test]
async fn invalid_owner_returns_error() {
    let client = Client::new("http://localhost:12345", "testnet");
    let err = client
        .get_balance("not-an-address", None)
        .await
        .expect_err("invalid owner should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .get_all_balances("not-an-address")
        .await
        .expect_err("invalid owner should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .get_coins("not-an-address", None, None, None)
        .await
        .expect_err("invalid owner should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .get_all_coins("not-an-address", None, None)
        .await
        .expect_err("invalid owner should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .multi_get_objects(vec!["not-an-object-id".to_string()], None)
        .await
        .expect_err("invalid object id should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .resolve_name_service_names("not-an-address", None, None)
        .await
        .expect_err("invalid address should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .get_stakes("not-an-address")
        .await
        .expect_err("invalid owner should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .get_stakes_by_ids(vec!["not-an-object-id".to_string()])
        .await
        .expect_err("invalid staked sui id should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .try_get_past_object("not-an-object-id", 1, None)
        .await
        .expect_err("invalid object id should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));

    let err = client
        .get_normalized_move_modules_by_package("not-an-package-id")
        .await
        .expect_err("invalid package id should fail");
    assert!(matches!(err, JsonRpcError::InvalidAddress));
}

#[test]
fn default_network_urls() {
    assert_eq!(
        default_jsonrpc_fullnode_url("mainnet").expect("mainnet"),
        "https://fullnode.mainnet.sui.io:443"
    );
    assert_eq!(
        default_jsonrpc_fullnode_url("testnet").expect("testnet"),
        "https://fullnode.testnet.sui.io:443"
    );
    assert!(default_jsonrpc_fullnode_url("foo").is_err());
}
