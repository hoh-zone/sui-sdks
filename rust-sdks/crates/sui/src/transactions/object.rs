#[derive(Debug, Clone)]
pub struct SuiObject {
    pub object_id: String,
    pub digest: String,
    pub version: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sui_object_creation() {
        let obj = SuiObject {
            object_id: "0x1".to_string(),
            digest: "abc123".to_string(),
            version: 1,
        };

        assert_eq!(obj.object_id, "0x1");
    }
}
