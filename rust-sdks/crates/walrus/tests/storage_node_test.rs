use httpmock::prelude::*;
use walrus::storage_node::client::{StorageNodeClient, StorageNodeClientOptions};
use walrus::types::BlobStatus;

#[tokio::test]
async fn get_blob_status_nonexistent() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/v1/blobs/blob-1/status");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"code":200,"success":{"data":"nonexistent"}}"#);
    });

    let client = StorageNodeClient::new(StorageNodeClientOptions { timeout_ms: 1000 });
    let status = client
        .get_blob_status(&server.base_url(), "blob-1")
        .await
        .unwrap();

    assert!(matches!(status, BlobStatus::Nonexistent));
}
