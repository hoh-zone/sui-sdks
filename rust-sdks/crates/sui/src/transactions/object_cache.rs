use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectValue {
    pub object_id: String,
    pub digest: String,
    pub version: u64,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ObjectCache<T: Clone> {
    cache: HashMap<String, T>,
}

impl<T: Clone> ObjectCache<T> {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, id: &str) -> Option<&T> {
        self.cache.get(id)
    }

    pub fn set(&mut self, id: String, value: T) {
        self.cache.insert(id, value);
    }

    pub fn contains(&self, id: &str) -> bool {
        self.cache.contains_key(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<T> {
        self.cache.remove(id)
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl<T: Clone> Default for ObjectCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_cache_set_get() {
        let mut cache = ObjectCache::new();
        let value = ObjectValue {
            object_id: "0x1".to_string(),
            digest: "digest".to_string(),
            version: 1,
            data: serde_json::Value::Null,
        };

        cache.set("0x1".to_string(), value.clone());
        let retrieved = cache.get("0x1");

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().object_id, "0x1");
    }

    #[test]
    fn test_object_cache_contains() {
        let mut cache = ObjectCache::new();
        cache.set(
            "0x1".to_string(),
            ObjectValue {
                object_id: "0x1".to_string(),
                digest: "digest".to_string(),
                version: 1,
                data: serde_json::Value::Null,
            },
        );

        assert!(cache.contains("0x1"));
        assert!(!cache.contains("0x2"));
    }

    #[test]
    fn test_object_cache_remove() {
        let mut cache = ObjectCache::new();
        cache.set(
            "0x1".to_string(),
            ObjectValue {
                object_id: "0x1".to_string(),
                digest: "digest".to_string(),
                version: 1,
                data: serde_json::Value::Null,
            },
        );

        let removed = cache.remove("0x1");
        assert!(removed.is_some());
        assert!(!cache.contains("0x1"));
    }

    #[test]
    fn test_object_cache_clear() {
        let mut cache = ObjectCache::new();
        cache.set(
            "0x1".to_string(),
            ObjectValue {
                object_id: "0x1".to_string(),
                digest: "digest".to_string(),
                version: 1,
                data: serde_json::Value::Null,
            },
        );
        cache.set(
            "0x2".to_string(),
            ObjectValue {
                object_id: "0x2".to_string(),
                digest: "digest".to_string(),
                version: 1,
                data: serde_json::Value::Null,
            },
        );

        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_object_cache_len() {
        let mut cache = ObjectCache::new();
        assert_eq!(cache.len(), 0);

        cache.set(
            "0x1".to_string(),
            ObjectValue {
                object_id: "0x1".to_string(),
                digest: "digest".to_string(),
                version: 1,
                data: serde_json::Value::Null,
            },
        );

        assert_eq!(cache.len(), 1);
    }
}
