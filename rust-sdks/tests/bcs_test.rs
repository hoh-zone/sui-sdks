use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Sample {
    a: u8,
    b: u64,
    c: String,
}

#[test]
fn bcs_roundtrip_and_encodings() {
    let sample = Sample {
        a: 7,
        b: 42,
        c: "sui".to_string(),
    };

    let bytes = sui_sdks_rust::bcs::serialize(&sample).expect("serialize");
    let decoded: Sample = sui_sdks_rust::bcs::deserialize(&bytes).expect("deserialize");
    assert_eq!(sample, decoded);

    let b64 = sui_sdks_rust::bcs::to_base64(&bytes);
    let b64_bytes = sui_sdks_rust::bcs::from_base64(&b64).expect("from base64");
    assert_eq!(bytes, b64_bytes);

    let hex = sui_sdks_rust::bcs::to_hex(&bytes);
    let hex_bytes = sui_sdks_rust::bcs::from_hex(&hex).expect("from hex");
    assert_eq!(bytes, hex_bytes);
}
