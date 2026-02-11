use crate::error::WalrusClientError;

pub fn blob_id_from_int(v: u128) -> String {
    format!("{:064x}", v)
}

pub fn blob_id_to_int(blob_id: &str) -> Result<u128, WalrusClientError> {
    let clean = blob_id.trim_start_matches("0x");
    let suffix = if clean.len() > 32 {
        &clean[clean.len() - 32..]
    } else {
        clean
    };
    u128::from_str_radix(suffix, 16)
        .map_err(|e| WalrusClientError::StorageNode(format!("invalid blob id: {}", e)))
}
