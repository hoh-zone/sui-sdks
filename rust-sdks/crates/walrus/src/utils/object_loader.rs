use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct ObjectLoader {
    cache: HashMap<String, serde_json::Value>,
}

impl ObjectLoader {
    pub fn insert(&mut self, object_id: String, value: serde_json::Value) {
        self.cache.insert(object_id, value);
    }

    pub fn get(&self, object_id: &str) -> Option<&serde_json::Value> {
        self.cache.get(object_id)
    }
}
