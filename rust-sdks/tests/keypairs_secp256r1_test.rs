use sui_sdks_rust::sui::keypairs::secp256r1::Keypair;

#[test]
fn secp256r1_sign_verify_and_encode_decode() {
    let keypair = Keypair::generate();
    let message = b"hello-secp256r1";

    let sig = keypair.sign(message);
    let verified = keypair.verify(message, &sig).expect("verify result");
    assert!(verified);

    let address = keypair.to_sui_address();
    assert!(address.starts_with("0x"));

    let exported = keypair.to_sui_private_key();
    let restored = Keypair::from_sui_private_key(&exported).expect("restore keypair");
    let sig2 = restored.sign(message);
    let verified2 = restored.verify(message, &sig2).expect("verify result");
    assert!(verified2);
}
