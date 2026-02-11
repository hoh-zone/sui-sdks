use serde_json::Value;

pub fn get_object_digest(obj: &Value) -> String {
    obj.get("digest")
        .and_then(|d| d.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn get_object_version(obj: &Value) -> u64 {
    obj.get("version").and_then(|v| v.as_u64()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_object_digest() {
        let obj = serde_json::json!({"digest": "test_digest"});
        let digest = get_object_digest(&obj);
        assert_eq!(digest, "test_digest");
    }

    #[test]
    fn test_get_object_version() {
        let obj = serde_json::json!({"version": 42});
        let version = get_object_version(&obj);
        assert_eq!(version, 42);
    }
}
