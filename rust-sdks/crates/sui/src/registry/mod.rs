const MAX_APP_SIZE: usize = 64;

pub fn is_valid_sui_ns_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

pub fn is_valid_named_package(name: &str) -> bool {
    let parts: Vec<&str> = name.split('/').collect();
    if !(2..=3).contains(&parts.len()) {
        return false;
    }
    let org = parts[0];
    let app = parts[1];
    let version = parts.get(2).copied();
    if !is_valid_sui_ns_name(org) {
        return false;
    }
    if app.is_empty() || app.len() >= MAX_APP_SIZE {
        return false;
    }
    if !app
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return false;
    }
    if app.starts_with('-') || app.ends_with('-') || app.contains("--") {
        return false;
    }
    if let Some(v) = version {
        if v.is_empty() || !v.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }
    true
}

pub fn is_valid_named_type(type_str: &str) -> bool {
    let mut token = String::new();
    for c in type_str.chars() {
        if matches!(c, ':' | '<' | '>' | ',') {
            if token.contains('/') && !is_valid_named_package(token.trim()) {
                return false;
            }
            token.clear();
        } else {
            token.push(c);
        }
    }
    if token.contains('/') && !is_valid_named_package(token.trim()) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_package_validation() {
        assert!(is_valid_named_package("mysten/sui"));
        assert!(is_valid_named_package("mysten/sui/1"));
        assert!(!is_valid_named_package("mysten"));
        assert!(!is_valid_named_package("mysten/sui/v1"));
    }

    #[test]
    fn test_named_type_validation() {
        assert!(is_valid_named_type("mysten/sui::coin::Coin<mysten/sui::sui::SUI>"));
        assert!(!is_valid_named_type("mysten/sui/v1::coin::Coin"));
    }
}
