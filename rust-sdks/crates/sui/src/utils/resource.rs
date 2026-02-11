use serde_json::Value;

pub fn parse_struct_tag(uri: &str) -> Result<String, String> {
    let mut parts = uri.splitn(3, "::");
    let address = parts.next().ok_or_else(|| "missing address".to_string())?;
    let module = parts.next().ok_or_else(|| "missing module".to_string())?;
    let rest = parts
        .next()
        .ok_or_else(|| "missing struct name".to_string())?;
    if module.is_empty() || rest.is_empty() {
        return Err("invalid struct tag".to_string());
    }

    let normalized_address = if address.contains('/') {
        address.to_string()
    } else {
        super::normalize_sui_address(address)
    };
    Ok(format!("{normalized_address}::{module}::{rest}"))
}

pub fn get_resource_value(fields: Value) -> Result<Value, String> {
    if let Some(v) = fields.get("fields") {
        return Ok(v.clone());
    }
    if let Some(v) = fields.get("value") {
        return Ok(v.clone());
    }
    if let Some(v) = fields.get("content").and_then(|c| c.get("fields")) {
        return Ok(v.clone());
    }
    if let Some(v) = fields
        .get("data")
        .and_then(|d| d.get("content"))
        .and_then(|c| c.get("fields"))
    {
        return Ok(v.clone());
    }
    Ok(fields)
}

pub fn decode_resource_object(obj: &impl GetObject) -> Result<Value, String> {
    get_resource_value(obj.get_data())
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
        assert_eq!(
            result.unwrap(),
            "0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin"
        );
    }

    #[test]
    fn test_get_resource_value() {
        let fields = serde_json::json!({"value": 100});
        let result = get_resource_value(fields).unwrap();
        assert_eq!(result, 100);
    }

    #[test]
    fn test_get_resource_value_nested_fields() {
        let fields = serde_json::json!({"content": {"fields": {"value": 100}}});
        let result = get_resource_value(fields).unwrap();
        assert_eq!(result["value"], 100);
    }
}
