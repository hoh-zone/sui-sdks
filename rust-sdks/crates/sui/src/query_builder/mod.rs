use std::collections::HashMap;

use serde_json::Value;

#[derive(Default, Clone)]
pub struct QueryCache {
    entries: HashMap<String, Value>,
}

impl QueryCache {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.entries.insert(key, value);
    }
}

#[derive(Default, Clone)]
pub struct GraphqlQueryBuilder {
    cache: QueryCache,
}

impl GraphqlQueryBuilder {
    pub fn build(query: &str, variables: Option<Value>) -> Value {
        serde_json::json!({
            "query": query,
            "variables": variables.unwrap_or_else(|| serde_json::json!({})),
        })
    }

    pub fn execute_with_cache<F>(
        &mut self,
        query: &str,
        variables: Option<Value>,
        mut executor: F,
    ) -> Result<Value, String>
    where
        F: FnMut(Value) -> Result<Value, String>,
    {
        let key = cache_key(query, variables.as_ref());
        if let Some(hit) = self.cache.get(&key) {
            return Ok(hit.clone());
        }
        let options = Self::build(query, variables);
        let result = executor(options)?;
        self.cache.set(key, result.clone());
        Ok(result)
    }
}

fn cache_key(query: &str, variables: Option<&Value>) -> String {
    format!(
        "{}{}",
        query,
        variables
            .map(|v| v.to_string())
            .unwrap_or_else(|| "{}".to_string())
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_with_cache() {
        let mut builder = GraphqlQueryBuilder::default();
        let mut calls = 0usize;
        let result1 = builder
            .execute_with_cache("query { ping }", None, |_| {
                calls += 1;
                Ok(serde_json::json!({"data":{"ping":"pong"}}))
            })
            .unwrap();
        let result2 = builder
            .execute_with_cache("query { ping }", None, |_| {
                calls += 1;
                Ok(serde_json::json!({"data":{"ping":"pong2"}}))
            })
            .unwrap();
        assert_eq!(calls, 1);
        assert_eq!(result1, result2);
    }
}
