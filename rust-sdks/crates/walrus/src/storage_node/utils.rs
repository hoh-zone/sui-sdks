use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn merge_headers(base: &[(&str, &str)], extra: Option<&HeaderMap>) -> HeaderMap {
    let mut map = HeaderMap::new();
    for (k, v) in base {
        if let (Ok(name), Ok(value)) = (
            HeaderName::from_bytes(k.as_bytes()),
            HeaderValue::from_str(v),
        ) {
            map.insert(name, value);
        }
    }
    if let Some(extra) = extra {
        for (k, v) in extra {
            map.insert(k.clone(), v.clone());
        }
    }
    map
}
