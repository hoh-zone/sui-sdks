use base64::Engine as _;
use sui::crypto::SignatureScheme;
use sui::keypairs::ed25519;
use sui::multisig::{
    parse_multisig, serialize_multisig, MultiSigEntry, MultiSigPublicKey, MultiSigSerialized,
    WeightedPublicKey,
};

#[test]
fn multisig_serialize_parse_and_verify() {
    let k1 = ed25519::Keypair::generate();
    let k2 = ed25519::Keypair::generate();

    let pk1 = base64::engine::general_purpose::STANDARD.encode(k1.public_key_bytes());
    let pk2 = base64::engine::general_purpose::STANDARD.encode(k2.public_key_bytes());

    let ms = MultiSigPublicKey {
        public_keys: vec![
            WeightedPublicKey {
                scheme: SignatureScheme::Ed25519,
                public_key: pk1.clone(),
                weight: 1,
            },
            WeightedPublicKey {
                scheme: SignatureScheme::Ed25519,
                public_key: pk2.clone(),
                weight: 1,
            },
        ],
        threshold: 2,
    };

    let msg = b"hello-multisig";
    let sig1 = base64::engine::general_purpose::STANDARD.encode(k1.sign(msg));
    let sig2 = base64::engine::general_purpose::STANDARD.encode(k2.sign(msg));

    let serialized = serialize_multisig(&MultiSigSerialized {
        signatures: vec![
            MultiSigEntry {
                public_key: pk1,
                signature: sig1,
            },
            MultiSigEntry {
                public_key: pk2,
                signature: sig2,
            },
        ],
        bitmap: vec![0, 1],
        threshold: 2,
    })
    .expect("serialize");

    let parsed = parse_multisig(&serialized).expect("parse");
    assert_eq!(parsed.signatures.len(), 2);

    let ok = ms.verify(msg, &serialized).expect("verify multisig");
    assert!(ok);
}
