#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G1Element(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G2Element(pub Vec<u8>);

impl G1Element {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl G2Element {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}
