use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;
use sui::graphql::{Client, GraphqlError, QueryOptions};

#[tokio::test]
async fn query_and_execute() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"data":{"ping":"pong"}}"#);
    });

    let client = Client::new(server.url("/"), "testnet")
        .with_query("ping", "query Ping { ping }");

    let res = client
        .query(QueryOptions {
            query: "query Ping { ping }".to_string(),
            variables: Some(json!({"x": 1})),
            operation_name: Some("Ping".to_string()),
            extensions: None,
        })
        .await
        .expect("query should succeed");

    assert!(res.data.is_some());

    let res2 = client
        .execute("ping", None, None, None)
        .await
        .expect("execute should succeed");

    assert!(res2.data.is_some());
}

#[tokio::test]
async fn execute_unknown_query_returns_error() {
    let client = Client::new("http://localhost:12345/graphql", "testnet");
    let err = client
        .execute("missing", None, None, None)
        .await
        .expect_err("should fail with unknown query");

    match err {
        GraphqlError::UnknownQuery(name) => assert_eq!(name, "missing"),
        other => panic!("unexpected error: {other:?}"),
    }
}
