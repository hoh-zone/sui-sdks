use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum WebsocketError {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("rpc error {code}: {message}")]
    Rpc { code: i64, message: String },
}

#[derive(Debug, Clone)]
pub struct WebsocketClientOptions {
    pub call_timeout_ms: u64,
}

impl Default for WebsocketClientOptions {
    fn default() -> Self {
        Self {
            call_timeout_ms: 30_000,
        }
    }
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait WebsocketTransport: Send + Sync {
    fn request(&self, id: u64, method: &str, params: Vec<Value>) -> BoxFuture<'_, Result<Value, WebsocketError>>;
    fn subscribe(
        &self,
        id: u64,
        method: &str,
        params: Vec<Value>,
    ) -> BoxFuture<'_, Result<u64, WebsocketError>>;
    fn unsubscribe(
        &self,
        id: u64,
        method: &str,
        subscription_id: u64,
    ) -> BoxFuture<'_, Result<bool, WebsocketError>>;
}

#[derive(Default)]
pub struct InMemoryWebsocketTransport;

impl WebsocketTransport for InMemoryWebsocketTransport {
    fn request(
        &self,
        _id: u64,
        _method: &str,
        _params: Vec<Value>,
    ) -> BoxFuture<'_, Result<Value, WebsocketError>> {
        Box::pin(async move { Err(WebsocketError::Transport(
            "no websocket backend configured".to_string(),
        )) })
    }

    fn subscribe(
        &self,
        _id: u64,
        _method: &str,
        _params: Vec<Value>,
    ) -> BoxFuture<'_, Result<u64, WebsocketError>> {
        Box::pin(async move { Err(WebsocketError::Transport(
            "no websocket backend configured".to_string(),
        )) })
    }

    fn unsubscribe(
        &self,
        _id: u64,
        _method: &str,
        _subscription_id: u64,
    ) -> BoxFuture<'_, Result<bool, WebsocketError>> {
        Box::pin(async move { Ok(true) })
    }
}

pub struct WebsocketClient<T: WebsocketTransport> {
    pub endpoint: String,
    pub options: WebsocketClientOptions,
    transport: T,
    request_id: AtomicU64,
    subscriptions: Arc<RwLock<HashMap<u64, String>>>,
}

impl<T: WebsocketTransport> WebsocketClient<T> {
    pub fn new(endpoint: impl Into<String>, transport: T, options: WebsocketClientOptions) -> Self {
        Self {
            endpoint: endpoint.into(),
            options,
            transport,
            request_id: AtomicU64::new(1),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    pub async fn make_request(&self, method: &str, params: Vec<Value>) -> Result<Value, WebsocketError> {
        let id = self.next_request_id();
        self.transport.request(id, method, params).await
    }

    pub async fn subscribe(
        &self,
        method: &str,
        unsubscribe_method: &str,
        params: Vec<Value>,
    ) -> Result<u64, WebsocketError> {
        let id = self.next_request_id();
        let subscription_id = self.transport.subscribe(id, method, params).await?;
        let mut guard = self.subscriptions.write().await;
        guard.insert(subscription_id, unsubscribe_method.to_string());
        Ok(subscription_id)
    }

    pub async fn unsubscribe(&self, subscription_id: u64) -> Result<bool, WebsocketError> {
        let unsubscribe_method = {
            let mut guard = self.subscriptions.write().await;
            guard.remove(&subscription_id)
        };
        if let Some(method) = unsubscribe_method {
            let id = self.next_request_id();
            self.transport.unsubscribe(id, &method, subscription_id).await
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTransport;

    impl WebsocketTransport for TestTransport {
        fn request(
            &self,
            _id: u64,
            method: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<'_, Result<Value, WebsocketError>> {
            let method = method.to_string();
            Box::pin(async move { Ok(serde_json::json!({"method": method})) })
        }

        fn subscribe(
            &self,
            _id: u64,
            _method: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<'_, Result<u64, WebsocketError>> {
            Box::pin(async move { Ok(42) })
        }

        fn unsubscribe(
            &self,
            _id: u64,
            _method: &str,
            _subscription_id: u64,
        ) -> BoxFuture<'_, Result<bool, WebsocketError>> {
            Box::pin(async move { Ok(true) })
        }
    }

    #[tokio::test]
    async fn test_request_and_subscription() {
        let client = WebsocketClient::new(
            "wss://example.com",
            TestTransport,
            WebsocketClientOptions::default(),
        );
        let resp = client.make_request("sui_ping", vec![]).await.unwrap();
        assert_eq!(resp["method"], "sui_ping");
        let sub_id = client
            .subscribe("sui_subscribeEvent", "sui_unsubscribeEvent", vec![])
            .await
            .unwrap();
        assert_eq!(sub_id, 42);
        assert!(client.unsubscribe(sub_id).await.unwrap());
    }
}
