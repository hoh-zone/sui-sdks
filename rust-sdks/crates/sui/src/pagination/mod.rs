use serde_json::Value;

pub fn collect_paginated_items<F>(
    mut fetch_page: F,
    start_cursor: Option<String>,
    max_items: Option<usize>,
) -> Result<Vec<Value>, String>
where
    F: FnMut(Option<String>) -> Result<Value, String>,
{
    let mut out = Vec::new();
    let mut cursor = start_cursor;
    loop {
        let page = fetch_page(cursor.clone())?;
        let data = page
            .get("data")
            .and_then(Value::as_array)
            .ok_or_else(|| "missing pagination data".to_string())?;
        for item in data {
            out.push(item.clone());
            if let Some(max) = max_items {
                if out.len() >= max {
                    return Ok(out);
                }
            }
        }
        let has_next = page
            .get("hasNextPage")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if !has_next {
            return Ok(out);
        }
        let next_cursor = page
            .get("nextCursor")
            .and_then(Value::as_str)
            .map(str::to_string);
        if next_cursor.is_none() || next_cursor == cursor {
            return Ok(out);
        }
        cursor = next_cursor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_paginated_items() {
        let mut calls = 0;
        let result = collect_paginated_items(
            |_| {
                calls += 1;
                if calls == 1 {
                    Ok(serde_json::json!({
                        "data": [{"id": 1}],
                        "hasNextPage": true,
                        "nextCursor": "c1"
                    }))
                } else {
                    Ok(serde_json::json!({
                        "data": [{"id": 2}],
                        "hasNextPage": false,
                        "nextCursor": null
                    }))
                }
            },
            None,
            None,
        )
        .unwrap();
        assert_eq!(result.len(), 2);
    }
}
