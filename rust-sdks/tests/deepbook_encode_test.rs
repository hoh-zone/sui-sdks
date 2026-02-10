use sui_sdks_rust::deepbook_v3::encode;

#[test]
fn encoding_helpers() {
    assert_eq!(encode::encode_u64(1), vec![1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(encode::encode_u128(1)[0], 1);
    assert_eq!(encode::encode_bool(true), vec![1]);
    let v = encode::encode_vec_u128(&[1, 2]);
    assert!(!v.is_empty());
}
