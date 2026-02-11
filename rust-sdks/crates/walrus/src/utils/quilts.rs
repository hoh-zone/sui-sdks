use crate::error::WalrusClientError;

#[derive(Debug, Clone)]
pub struct EncodeQuiltOptions {
    pub blobs: Vec<(String, Vec<u8>)>,
}

#[derive(Debug, Clone)]
pub struct EncodedQuilt {
    pub bytes: Vec<u8>,
    pub offsets: Vec<(String, u64, u64)>,
}

pub fn encode_quilt(options: EncodeQuiltOptions) -> Result<EncodedQuilt, WalrusClientError> {
    let mut bytes = Vec::new();
    let mut offsets = Vec::new();
    for (id, blob) in options.blobs {
        let start = bytes.len() as u64;
        bytes.extend_from_slice(&blob);
        let end = bytes.len() as u64;
        offsets.push((id, start, end));
    }
    Ok(EncodedQuilt { bytes, offsets })
}
