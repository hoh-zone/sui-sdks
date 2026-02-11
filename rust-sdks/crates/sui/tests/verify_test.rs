use sui::crypto::SignatureScheme;
use sui::sui::keypairs::{ed25519, secp256k1, secp256r1};
use sui::sui::verify;

#[test]
fn verify_ed25519_signature() {
    let kp = ed25519::Keypair::generate();
    let msg = b"hello-verify-ed25519";
    let sig = kp.sign(msg);
    let ok = verify::verify_signature(SignatureScheme::Ed25519, &kp.public_key_bytes(), msg, &sig)
        .expect("verify");
    assert!(ok);
}

#[test]
fn verify_secp_signatures() {
    let k1 = secp256k1::Keypair::generate();
    let r1 = secp256r1::Keypair::generate();
    let msg = b"hello-verify-secp";

    let sig_k1 = k1.sign(msg);
    let sig_r1 = r1.sign(msg);

    let ok_k1 = verify::verify_signature(
        SignatureScheme::Secp256k1,
        &k1.public_key_bytes(),
        msg,
        &sig_k1,
    )
    .expect("verify k1");
    let ok_r1 = verify::verify_signature(
        SignatureScheme::Secp256r1,
        &r1.public_key_bytes(),
        msg,
        &sig_r1,
    )
    .expect("verify r1");

    assert!(ok_k1);
    assert!(ok_r1);
}

#[test]
fn verify_transaction_intent() {
    let kp = ed25519::Keypair::generate();
    let tx = b"tx-bytes";
    let intent_msg = sui::crypto::message_with_intent([0, 0, 0], tx);
    let sig = kp.sign(&intent_msg);

    let ok = verify::verify_transaction(SignatureScheme::Ed25519, &kp.public_key_bytes(), tx, &sig)
        .expect("verify tx");
    assert!(ok);
}
