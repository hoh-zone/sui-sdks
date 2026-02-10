use sui_sdks_rust::crypto::{hash_with_intent, message_with_intent, SignatureScheme};

#[test]
fn intent_message_and_hash_are_deterministic() {
    let intent = [0_u8, 0_u8, 0_u8];
    let message = b"hello";
    let payload = message_with_intent(intent, message);
    assert_eq!(&payload[0..3], &intent);
    assert_eq!(&payload[3..], message);

    let digest1 = hash_with_intent(intent, message);
    let digest2 = hash_with_intent(intent, message);
    assert_eq!(digest1, digest2);
}

#[test]
fn signature_scheme_flags_match_sui_convention() {
    assert_eq!(SignatureScheme::Ed25519.flag(), 0x00);
    assert_eq!(SignatureScheme::Secp256k1.flag(), 0x01);
    assert_eq!(SignatureScheme::Secp256r1.flag(), 0x02);
}
