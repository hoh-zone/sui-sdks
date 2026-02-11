use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PureValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(String),
    Address(String),
    String(String),
    Vec(Vec<u8>),
}

impl PureValue {
    pub fn serialize(&self) -> Result<Vec<u8>, bcs::Error> {
        use bcs;

        match self {
            PureValue::Bool(v) => bcs::to_bytes(v),
            PureValue::U8(v) => bcs::to_bytes(v),
            PureValue::U16(v) => bcs::to_bytes(v),
            PureValue::U32(v) => bcs::to_bytes(v),
            PureValue::U64(v) => bcs::to_bytes(v),
            PureValue::U128(v) => bcs::to_bytes(v),
            PureValue::U256(v) => bcs::to_bytes(v),
            PureValue::Address(addr) => bcs::to_bytes(addr.as_bytes()),
            PureValue::String(s) => bcs::to_bytes(s),
            PureValue::Vec(v) => bcs::to_bytes(v),
        }
    }

    pub fn bool(v: bool) -> Self {
        PureValue::Bool(v)
    }

    pub fn u8(v: u8) -> Self {
        PureValue::U8(v)
    }

    pub fn u16(v: u16) -> Self {
        PureValue::U16(v)
    }

    pub fn u32(v: u32) -> Self {
        PureValue::U32(v)
    }

    pub fn u64(v: u64) -> Self {
        PureValue::U64(v)
    }

    pub fn u128(v: u128) -> Self {
        PureValue::U128(v)
    }

    pub fn vec(v: Vec<u8>) -> Self {
        PureValue::Vec(v)
    }

    pub fn address(s: String) -> Self {
        PureValue::Address(s)
    }

    pub fn string(s: String) -> Self {
        PureValue::String(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_bool() {
        let val = PureValue::bool(true);
        let serialized = val.serialize().unwrap();
        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_serialize_u8() {
        let val = PureValue::u8(42);
        let serialized = val.serialize().unwrap();
        assert_eq!(serialized.len(), 1);
    }

    #[test]
    fn test_serialize_u16() {
        let val = PureValue::u16(1000);
        let serialized = val.serialize().unwrap();
        assert_eq!(serialized.len(), 2);
    }

    #[test]
    fn test_serialize_u32() {
        let val = PureValue::u32(1000000);
        let serialized = val.serialize().unwrap();
        assert_eq!(serialized.len(), 4);
    }

    #[test]
    fn test_serialize_u64() {
        let val = PureValue::u64(1000000000);
        let serialized = val.serialize().unwrap();
        assert_eq!(serialized.len(), 8);
    }

    #[test]
    fn test_serialize_string() {
        let val = PureValue::string("hello".to_string());
        let serialized = val.serialize().unwrap();
        assert!(serialized.len() > 0);
    }

    #[test]
    fn test_serialize_address() {
        let val = PureValue::address("0x1".to_string());
        let serialized = val.serialize().unwrap();
        assert!(!serialized.is_empty());
    }
}
