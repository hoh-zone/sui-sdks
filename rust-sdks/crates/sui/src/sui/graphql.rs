use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOptions {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Value>,
    #[serde(rename = "operationName", skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ResponseError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Value>>,
}

#[derive(Debug, thiserror::Error)]
pub enum GraphqlError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("invalid header name: {0}")]
    HeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error("invalid header value: {0}")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("graphql request failed: {status}")]
    HttpStatus { status: reqwest::StatusCode },
    #[error("unknown query: {0}")]
    UnknownQuery(String),
}

pub struct Client {
    url: String,
    network: String,
    headers: HeaderMap,
    client: reqwest::Client,
    queries: std::collections::HashMap<String, String>,
}

impl Client {
    pub fn new(url: impl Into<String>, network: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            network: network.into(),
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
            queries: std::collections::HashMap::new(),
        }
    }

    pub fn with_query(mut self, name: impl Into<String>, query: impl Into<String>) -> Self {
        self.queries.insert(name.into(), query.into());
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Result<Self, GraphqlError> {
        let header_name = HeaderName::from_str(key)?;
        let header_value = HeaderValue::from_str(value)?;
        self.headers.insert(header_name, header_value);
        Ok(self)
    }

    pub fn network(&self) -> &str {
        &self.network
    }

    pub async fn query(&self, opts: QueryOptions) -> Result<QueryResult, GraphqlError> {
        let mut req = self.client.post(&self.url).json(&opts);
        if !self.headers.is_empty() {
            req = req.headers(self.headers.clone());
        }

        let response = req.send().await?;
        if !response.status().is_success() {
            return Err(GraphqlError::HttpStatus {
                status: response.status(),
            });
        }
        Ok(response.json::<QueryResult>().await?)
    }

    pub async fn execute(
        &self,
        query_name: &str,
        variables: Option<Value>,
        operation_name: Option<String>,
        extensions: Option<Value>,
    ) -> Result<QueryResult, GraphqlError> {
        let query = self
            .queries
            .get(query_name)
            .cloned()
            .ok_or_else(|| GraphqlError::UnknownQuery(query_name.to_string()))?;

        self.query(QueryOptions {
            query,
            variables,
            operation_name,
            extensions,
        })
        .await
    }

    pub fn to_async_graphql_request(opts: QueryOptions) -> async_graphql::Request {
        let mut request = async_graphql::Request::new(opts.query);
        if let Some(variables) = opts.variables {
            request = request.variables(async_graphql::Variables::from_json(variables));
        }
        if let Some(operation_name) = opts.operation_name {
            request = request.operation_name(operation_name);
        }
        request
    }
}
