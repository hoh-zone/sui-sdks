use httpmock::prelude::*;

#[tokio::test]
async fn verify_key_server_success() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/service")
            .query_param("service_id", "0x1");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"service_id":"0x1","pop":""}"#);
    });

    let ok = seal::key_server::verify_key_server(&server.base_url(), "0x1", 1000, None, None)
        .await
        .unwrap();
    assert!(ok);
}

#[tokio::test]
async fn fetch_keys_for_all_ids_parses_keys() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST).path("/v1/fetch_key");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"keys":[{"full_id":"abc","key":"a2V5"}]}"#);
    });

    let keys = seal::key_server::fetch_keys_for_all_ids(
        &server.base_url(),
        "sig",
        b"tx",
        b"enc_pk",
        b"enc_vk",
        1000,
        None,
        None,
    )
    .await
    .unwrap();

    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].full_id, "abc");
    assert_eq!(keys[0].key, b"key");
}
