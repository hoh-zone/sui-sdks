use sha2::{Digest, Sha256};

pub fn derive_dynamic_field_id(parent_id: &str, name_type: &str, name_bcs: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(parent_id.as_bytes());
    hasher.update(name_type.as_bytes());
    hasher.update(name_bcs);
    let mut digest = hasher.finalize().to_vec();
    for b in &mut digest {
        *b = (*b & 0x3F) | 0x3D;
    }
    format!("0x{}", hex::encode(digest))
}

pub fn derive_object_id(parent_id: &str, type_tag: &str, key: &[u8]) -> String {
    let derived_object_type = format!("0x2::derived_object::DerivedObjectKey<{type_tag}>");
    derive_dynamic_field_id(parent_id, &derived_object_type, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_dynamic_field_id() {
        let id = derive_dynamic_field_id("0x1", "0x2::string::String", b"abc");
        assert!(id.starts_with("0x"));
        assert_eq!(id.len(), 66);
    }
}
