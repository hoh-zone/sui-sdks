use crate::files::blob::WalrusBlob;

#[derive(Debug, Clone)]
pub struct BlobReader {
    blob: WalrusBlob,
}

impl BlobReader {
    pub fn new(blob: WalrusBlob) -> Self {
        Self { blob }
    }

    pub fn read_all(&self) -> &[u8] {
        &self.blob.contents
    }
}
