use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct SubscriptionOptions {
    pub buffer_size: usize,
    pub timeout: Duration,
}

impl Default for SubscriptionOptions {
    fn default() -> Self {
        Self {
            buffer_size: 1000,
            timeout: Duration::from_secs(300),
        }
    }
}

#[derive(Debug)]
pub struct Subscription<T> {
    receiver: mpsc::UnboundedReceiver<T>,
    closed: Arc<std::sync::atomic::AtomicBool>,
}

impl<T> Subscription<T> {
    pub fn new(receiver: mpsc::UnboundedReceiver<T>) -> Self {
        Self {
            receiver,
            closed: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub async fn next(&mut self) -> Option<T> {
        self.receiver.recv().await
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
    }
}

impl<T> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Debug)]
pub struct SubscriptionManager {
    #[allow(dead_code)]
    request_id: Arc<AtomicU64>,
    subscriptions: Arc<tokio::sync::RwLock<std::collections::HashMap<u64, bool>>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            request_id: Arc::new(AtomicU64::new(1)),
            subscriptions: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    #[allow(dead_code)]
async fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    pub async fn create<T>(
        &self,
        _opts: SubscriptionOptions,
    ) -> (Subscription<T>, mpsc::UnboundedSender<T>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let subscription = Subscription::new(receiver);
        (subscription, sender)
    }

    pub async fn close(&self, id: u64) {
        let mut subs = self.subscriptions.write().await;
        subs.remove(&id);
    }

    pub async fn close_all(&self) {
        let mut subs = self.subscriptions.write().await;
        subs.clear();
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimeoutError(Duration);

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "timeout after {:?}", self.0)
    }
}

impl std::error::Error for TimeoutError {}

#[derive(Debug, thiserror::Error)]
pub enum SubscriptionError {
    #[error("channel closed")]
    Closed,
    #[error("{0}")]
    Timeout(TimeoutError),
    #[error("send error")]
    Send,
    #[error("invalid subscription id: {0}")]
    InvalidId(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscription_next() {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut sub = Subscription::new(rx);

        tx.send(42).expect("send failed");
        let value = sub.next().await;
        assert_eq!(value, Some(42));
    }

    #[tokio::test]
    async fn test_subscription_close() {
        let (_tx, rx) = mpsc::unbounded_channel::<()>();
        let sub = Subscription::new(rx);

        assert!(!sub.is_closed());
        sub.close();
        assert!(sub.is_closed());
    }

    #[tokio::test]
    async fn test_subscription_drop_closes() {
        let (tx, rx) = mpsc::unbounded_channel::<i32>();

        {
            let _sub = Subscription::new(rx);
            drop(_sub);
        }

        assert!(tx.is_closed());
    }

    #[tokio::test]
    async fn test_subscription_manager_create() {
        let manager = SubscriptionManager::new();
        let opts = SubscriptionOptions::default();

        let (mut sub, tx): (Subscription<i32>, mpsc::UnboundedSender<i32>) =
            manager.create(opts).await;

        tx.send(123).expect("send failed");
        let value = sub.next().await;
        assert_eq!(value, Some(123));
    }

    #[tokio::test]
    async fn test_subscription_manager_close_all() {
        let manager = SubscriptionManager::new();
        let opts = SubscriptionOptions::default();

        let (_sub, _tx): (Subscription<i32>, mpsc::UnboundedSender<i32>) =
            manager.create(opts).await;

        manager.close_all().await;
    }

    #[tokio::test]
    async fn test_subscription_options_default() {
        let opts = SubscriptionOptions::default();
        assert_eq!(opts.buffer_size, 1000);
        assert_eq!(opts.timeout, Duration::from_secs(300));
    }
}