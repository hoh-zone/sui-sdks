#[derive(Debug, Clone)]
pub struct WalrusBlob {
    pub id: String,
    pub contents: Vec<u8>,
}

impl WalrusBlob {
    pub fn new(id: String, contents: Vec<u8>) -> Self {
        Self { id, contents }
    }
}
