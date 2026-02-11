use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicField {
    pub name: DynamicFieldType,
    pub value: DynamicFieldType,
}

impl DynamicField {
    pub fn new(name: DynamicFieldType, value: DynamicFieldType) -> Self {
        Self { name, value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicFieldType {
    pub type_: Option<String>,
    #[serde(rename = "value")]
    pub value: Value,
}

impl DynamicFieldType {
    pub fn new(type_: Option<String>, value: Value) -> Self {
        Self { type_, value }
    }

    pub fn as_string(&self) -> Option<String> {
        self.value.as_str().map(String::from)
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.value.as_u64()
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.value.as_bool()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DynamicFieldName(pub String);

impl DynamicFieldName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for DynamicFieldName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for DynamicFieldName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicFieldValue {
    pub type_: Option<String>,
    pub value: Value,
}

impl DynamicFieldValue {
    pub fn new(type_: Option<String>, value: Value) -> Self {
        Self { type_, value }
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.value.as_u64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_field_type() {
        let field_type = DynamicFieldType::new(Some("u64".to_string()), serde_json::json!(123));
        assert_eq!(field_type.type_.as_deref(), Some("u64"));
        assert_eq!(field_type.as_u64(), Some(123));
    }

    #[test]
    fn test_dynamic_field_type_string() {
        let field_type =
            DynamicFieldType::new(Some("string".to_string()), serde_json::json!("hello"));
        assert_eq!(field_type.as_string(), Some("hello".to_string()));
    }

    #[test]
    fn test_dynamic_field_type_bool() {
        let field_type = DynamicFieldType::new(Some("bool".to_string()), serde_json::json!(true));
        assert_eq!(field_type.as_bool(), Some(true));
    }

    #[test]
    fn test_dynamic_field() {
        let name = DynamicFieldType::new(Some("string".to_string()), serde_json::json!("key"));
        let value = DynamicFieldType::new(Some("u64".to_string()), serde_json::json!(42));
        let field = DynamicField::new(name, value);
        assert_eq!(field.name.as_string().unwrap(), "key");
        assert_eq!(field.value.as_u64().unwrap(), 42);
    }

    #[test]
    fn test_dynamic_field_name() {
        let name = DynamicFieldName::new("test_name".to_string());
        assert_eq!(name.as_str(), "test_name");
    }

    #[test]
    fn test_dynamic_field_name_from() {
        let name: DynamicFieldName = "from_string".into();
        assert_eq!(name.as_str(), "from_string");
    }

    #[test]
    fn test_dynamic_field_value() {
        let value = DynamicFieldValue::new(
            Some("u128".to_string()),
            serde_json::json!(18446744073709551615u128),
        );
        assert!(value.value.is_u64() || value.value.is_i64());
    }

    #[test]
    fn test_dynamic_field_value_u64() {
        let value = DynamicFieldValue::new(Some("u64".to_string()), serde_json::json!(1000));
        assert_eq!(value.as_u64(), Some(1000));
    }
}
