use serde_json::json;
use sui::zklogin;

#[test]
fn generate_nonce_and_randomness() {
    let randomness = zklogin::generate_randomness();
    let nonce = zklogin::generate_nonce(&[1, 2, 3], 12345, &randomness).expect("nonce");
    assert!(!nonce.is_empty());
    assert!(nonce.len() <= zklogin::NONCE_LENGTH);
}

#[test]
fn decode_jwt_payload() {
    // header: {"alg":"none"}, payload: {"iss":"https://issuer","aud":"sui"}
    let jwt = "eyJhbGciOiJub25lIn0.eyJpc3MiOiJodHRwczovL2lzc3VlciIsImF1ZCI6InN1aSJ9.sig";
    let payload = zklogin::decode_jwt(jwt).expect("decode jwt");
    assert_eq!(payload.get("aud").and_then(|v| v.as_str()), Some("sui"));
}

#[test]
fn compute_address_and_signature_roundtrip() {
    let addr = zklogin::compute_zklogin_address(zklogin::ComputeZkLoginAddressOptions {
        iss: Some("https://issuer".to_string()),
        aud: Some("sui".to_string()),
        user_salt: "salt123".to_string(),
        jwt: None,
        legacy_address: Some(false),
    })
    .expect("compute address");
    assert!(addr.starts_with("0x"));

    let sig = zklogin::get_zklogin_signature(&zklogin::ZkLoginSignatureExtended {
        inputs: json!({"proof": "abc"}),
        max_epoch: 123,
        user_signature: "user_sig".to_string(),
    })
    .expect("serialize zklogin sig");

    let parsed = zklogin::parse_zklogin_signature(&sig).expect("parse zklogin sig");
    assert_eq!(parsed.max_epoch, 123);
    assert_eq!(parsed.user_signature, "user_sig");
}
