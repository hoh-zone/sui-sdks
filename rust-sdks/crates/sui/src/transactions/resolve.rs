use super::normalize_sui_address;

pub fn resolve_address(addr: &str) -> String {
    normalize_sui_address(addr)
}

pub fn resolve_object_ref(object_id: &str, digest: &str, version: u64) -> super::inputs::ObjectRef {
    super::inputs::ObjectRef {
        object_id: normalize_sui_address(object_id),
        digest: digest.to_string(),
        version,
    }
}

pub fn resolve_type_parameter(type_arg: &str) -> String {
    type_arg.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_address() {
        let addr = "0x1";
        let resolved = resolve_address(addr);
        assert!(resolved.starts_with("0x"));
        assert_eq!(resolved.len(), 66);
    }

    #[test]
    fn test_resolve_object_ref() {
        let obj_ref = resolve_object_ref("0x1", "digest", 1);
        assert_eq!(obj_ref.version, 1);
        assert_eq!(obj_ref.digest, "digest");
    }

    #[test]
    fn test_resolve_type_parameter() {
        let type_arg = "0x2::coin::Coin";
        let resolved = resolve_type_parameter(type_arg);
        assert_eq!(resolved, type_arg);
    }
}
