pub fn normalize_sui_address(addr: &str) -> String {
    let mut raw = addr.trim().to_lowercase();
    if let Some(stripped) = raw.strip_prefix("0x") {
        raw = stripped.to_string();
    }

    let max_len = super::SUI_ADDRESS_LENGTH * 2;
    if raw.len() > max_len {
        raw = raw[raw.len() - max_len..].to_string();
    }
    if raw.len() < max_len {
        raw = format!("{}{}", "0".repeat(max_len - raw.len()), raw);
    }
    format!("0x{raw}")
}

pub fn validate_sui_address(addr: &str) -> bool {
    let raw = addr.trim().strip_prefix("0x").unwrap_or(addr.trim());
    if raw.is_empty() || raw.len() > super::SUI_ADDRESS_LENGTH * 2 {
        return false;
    }
    let padded = if raw.len() % 2 != 0 {
        format!("0{}", raw)
    } else {
        raw.to_string()
    };
    hex::decode(padded).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_sui_address_with_0x() {
        let addr = "0x1";
        let normalized = normalize_sui_address(addr);
        assert_eq!(normalized.len(), 66);
        assert!(normalized.starts_with("0x"));
    }

    #[test]
    fn test_normalize_sui_address_without_0x() {
        let addr = "1";
        let normalized = normalize_sui_address(addr);
        assert_eq!(normalized.len(), 66);
        assert!(normalized.starts_with("0x"));
    }

    #[test]
    fn test_normalize_sui_address_full() {
        let addr = format!("0x{}1", "0".repeat(63));
        let normalized = normalize_sui_address(&addr);
        assert_eq!(normalized, format!("0x{}1", "0".repeat(63)));
    }

    #[test]
    fn test_validate_sui_address_valid() {
        assert!(validate_sui_address("0x1"));
        assert!(validate_sui_address("1"));
        assert!(validate_sui_address(&format!("0x{}", "0".repeat(64))));
    }

    #[test]
    fn test_validate_sui_address_invalid() {
        assert!(!validate_sui_address(""));
        assert!(!validate_sui_address("0x"));
        assert!(!validate_sui_address("gg"));
        assert!(!validate_sui_address(&format!("0x{}", "0".repeat(65))));
    }
}
