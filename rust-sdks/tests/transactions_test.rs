use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;
use sui_sdks_rust::sui::jsonrpc::Client as JsonRpcClient;
use sui_sdks_rust::sui::keypairs::ed25519::Keypair;
use sui_sdks_rust::sui::transactions::Transaction;

#[test]
fn transaction_build_and_sign() {
    let mut tx = Transaction::new();
    tx.set_sender("0x1");
    tx.set_gas_budget(1_000_000);
    tx.set_gas_price(1000);

    let amount = tx.pure_bytes(&100u64.to_le_bytes());
    let split = tx.split_coins(Transaction::gas(), vec![amount]);
    let recipient = tx.pure_bytes(b"0x2");
    tx.transfer_objects(vec![split], recipient);

    let bytes = tx.build().expect("build");
    assert!(!bytes.is_empty());

    let b64 = tx.build_base64().expect("b64");
    assert!(!b64.is_empty());

    let kp = Keypair::generate();
    let signed = tx.sign_with_ed25519(&kp).expect("sign");
    assert_eq!(signed.signatures.len(), 1);
    assert!(!signed.tx_bytes_base64.is_empty());
}

#[tokio::test]
async fn signed_transaction_execute_calls_jsonrpc() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"sui_executeTransactionBlock\"");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"result":{"digest":"abc"}}"#);
    });

    let mut tx = Transaction::new();
    tx.set_sender("0x1");
    tx.set_gas_budget(1);
    let amount = tx.pure_bytes(&1u64.to_le_bytes());
    tx.split_coins(Transaction::gas(), vec![amount]);

    let kp = Keypair::generate();
    let signed = tx.sign_with_ed25519(&kp).expect("sign");

    let rpc = JsonRpcClient::new(server.url("/"), "testnet");
    let out = signed
        .execute(
            &rpc,
            Some(json!({"showEffects": true})),
            Some("WaitForLocalExecution"),
        )
        .await
        .expect("execute");

    assert_eq!(out["digest"], "abc");
}
