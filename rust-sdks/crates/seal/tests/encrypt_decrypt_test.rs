use seal::{DemType, EncryptOptions, KeyServerConfig, SealClient, SealClientOptions, SessionKey, DecryptOptions};

#[tokio::test]
async fn encrypt_decrypt_roundtrip() {
    let mut client = SealClient::new(SealClientOptions {
        server_configs: vec![KeyServerConfig {
            object_id: "0x1".to_string(),
            weight: 1,
            api_key_name: None,
            api_key: None,
            aggregator_url: None,
            url: None,
        }],
        verify_key_servers: Some(false),
        timeout_ms: Some(1000),
    })
    .unwrap();

    let plaintext = b"hello seal".to_vec();
    let (encrypted, _key) = client
        .encrypt(EncryptOptions {
            kem_type: None,
            dem_type: Some(DemType::AesGcm256),
            threshold: 1,
            package_id: "0x2".to_string(),
            id: "object-1".to_string(),
            data: plaintext.clone(),
            aad: Some(b"aad".to_vec()),
        })
        .await
        .unwrap();

    let session = SessionKey::create("0xabc".to_string(), "0x2".to_string(), None, 10).unwrap();

    let decrypted = client
        .decrypt(DecryptOptions {
            data: encrypted,
            session_key: session,
            tx_bytes: b"tx".to_vec(),
            check_share_consistency: Some(false),
            check_le_encoding: Some(false),
        })
        .await
        .unwrap();

    assert_eq!(decrypted, plaintext);
}
