pub fn encode_u64(value: u64) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

pub fn encode_u128(value: u128) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

pub fn encode_bool(value: bool) -> Vec<u8> {
    vec![u8::from(value)]
}

pub fn encode_u8(value: u8) -> Vec<u8> {
    vec![value]
}

pub fn encode_option_u64(value: Option<u64>) -> Vec<u8> {
    match value {
        Some(v) => {
            let mut out = Vec::with_capacity(1 + 8);
            out.push(1);
            out.extend_from_slice(&v.to_le_bytes());
            out
        }
        None => vec![0],
    }
}

pub fn encode_vec_u128(values: &[u128]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + values.len() * 16);
    out.extend_from_slice(&(values.len() as u32).to_le_bytes());
    for value in values {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}
