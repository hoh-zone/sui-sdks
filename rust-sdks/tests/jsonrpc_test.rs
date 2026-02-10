use httpmock::Method::POST;
use httpmock::MockServer;
use sui_sdks_rust::sui::jsonrpc::{default_jsonrpc_fullnode_url, Client, JsonRpcError};

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
}

#[tokio::test]
async fn invalid_owner_returns_error() {
    let client = Client::new("http://localhost:12345", "testnet");
    let err = client
        .get_balance("not-an-address", None)
        .await
        .expect_err("invalid owner should fail");
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
