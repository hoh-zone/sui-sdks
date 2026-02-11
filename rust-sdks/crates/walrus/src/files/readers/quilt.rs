#[derive(Debug, Clone)]
pub struct QuiltReader {
    data: Vec<u8>,
}

impl QuiltReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
}
