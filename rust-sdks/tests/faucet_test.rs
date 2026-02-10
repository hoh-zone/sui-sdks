use httpmock::Method::POST;
use httpmock::MockServer;
use sui_sdks_rust::sui::faucet::{get_faucet_host, FaucetClient, FaucetError};

#[tokio::test]
async fn faucet_request_success() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/v2/gas");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"status":"Success","coins_sent":[{"amount":1000,"id":"0x1","transferTxDigest":"abc"}]}"#);
    });

    let client = FaucetClient::new(server.url("/v2/gas"));
    let out = client
        .request_sui_from_faucet_v2("0x123", None)
        .await
        .expect("faucet success");

    assert_eq!(out.coins_sent.len(), 1);
    assert_eq!(out.coins_sent[0].amount, 1000);
}

#[tokio::test]
async fn faucet_rate_limit_error() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/v2/gas");
        then.status(429);
    });

    let client = FaucetClient::new(server.url("/v2/gas"));
    let err = client
        .request_sui_from_faucet_v2("0x123", None)
        .await
        .expect_err("should rate limit");
    assert!(matches!(err, FaucetError::RateLimited));
}

#[test]
fn faucet_host_mapping() {
    assert_eq!(
        get_faucet_host("testnet").expect("testnet host"),
        "https://faucet.testnet.sui.io/v2/gas"
    );
    assert!(get_faucet_host("unknown").is_err());
}
