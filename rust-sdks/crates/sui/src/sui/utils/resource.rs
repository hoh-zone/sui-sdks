use serde_json::Value;

pub fn parse_struct_tag(uri: &str) -> Result<String, String> {
    Ok(uri.to_string())
}

pub fn get_resource_value(fields: Value) -> Result<Value, String> {
    Ok(fields)
}

pub fn decode_resource_object(obj: &impl GetObject) -> Result<Value, String> {
    Ok(obj.get_data())
}

pub trait GetObject {
    fn get_data(&self) -> Value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_struct_tag() {
        let tag = "0x2::coin::Coin";
        let result = parse_struct_tag(tag);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), tag);
    }

    #[test]
    fn test_get_resource_value() {
        let fields = serde_json::json!({"value": 100});
        let result = get_resource_value(fields).unwrap();
        assert_eq!(result["value"], 100);
    }
}
