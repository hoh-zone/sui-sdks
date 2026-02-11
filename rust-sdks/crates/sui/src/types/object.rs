use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
    pub object_type: Option<String>,
    pub owner: Option<Owner>,
    pub previous_transaction: Option<String>,
    pub bcs: Option<ObjectBcs>,
    pub storage_rebate: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectBcs {
    pub version: u64,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Owner {
    AddressOwner(String),
    ObjectOwner(String),
    Shared { initial_shared_version: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRead {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
    pub object_type: Option<String>,
    pub owner: Option<Owner>,
    pub content: Option<ObjectContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectContent {
    pub data_type: String,
    pub data: serde_json::Value,
    pub has_public_transfer: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectKind {
    Owned,
    Shared,
    Immutable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectInfo {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
}

impl ObjectInfo {
    pub fn new(object_id: String, version: u64, digest: String) -> Self {
        Self {
            object_id,
            version,
            digest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedObjectRef {
    pub object_id: String,
    pub initial_shared_version: u64,
    pub mutable: bool,
}

impl SharedObjectRef {
    pub fn new(object_id: String, initial_shared_version: u64, mutable: bool) -> Self {
        Self {
            object_id,
            initial_shared_version,
            mutable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_info() {
        let info = ObjectInfo::new("0x123".to_string(), 1, "digest".to_string());
        assert_eq!(info.object_id, "0x123");
        assert_eq!(info.version, 1);
        assert_eq!(info.digest, "digest");
    }

    #[test]
    fn test_shared_object_ref() {
        let ref_obj = SharedObjectRef::new("0x456".to_string(), 1, true);
        assert_eq!(ref_obj.object_id, "0x456");
        assert_eq!(ref_obj.initial_shared_version, 1);
        assert!(ref_obj.mutable);
    }

    #[test]
    fn test_object_kind() {
        let kind = ObjectKind::Owned;
        assert!(matches!(kind, ObjectKind::Owned));
    }

    #[test]
    fn test_owner_variants() {
        let _addr_owner = Owner::AddressOwner("0xabc".to_string());
        let _obj_owner = Owner::ObjectOwner("0xdef".to_string());
        let _shared_owner = Owner::Shared {
            initial_shared_version: 1,
        };
    }

    #[test]
    fn test_object_read() {
        let obj_read = ObjectRead {
            object_id: "0x123".to_string(),
            version: 1,
            digest: "digest".to_string(),
            object_type: Some("type".to_string()),
            owner: None,
            content: None,
        };
        assert_eq!(obj_read.version, 1);
    }
}
