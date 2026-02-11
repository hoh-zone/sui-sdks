use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type Callback<T> = Arc<dyn Fn(T) + Send + Sync>;

#[derive(Debug, Clone)]
pub struct EventSubscriberOptions {
    pub buffer_size: usize,
}

impl Default for EventSubscriberOptions {
    fn default() -> Self {
        Self { buffer_size: 1000 }
    }
}

#[derive(Clone)]
pub struct EventSubscriber<T> {
    callbacks: Arc<RwLock<Vec<Callback<T>>>>,
    #[allow(dead_code)]
    buffer_size: usize,
}

impl<T: Clone + Send + 'static> EventSubscriber<T> {
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(Vec::new())),
            buffer_size: 1000,
        }
    }

    pub fn with_options(opts: EventSubscriberOptions) -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(Vec::new())),
            buffer_size: opts.buffer_size,
        }
    }

    pub async fn on_event<F>(&self, callback: F) -> u64
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.write().await;
        let id = callbacks.len() as u64;
        callbacks.push(Arc::new(callback));
        id
    }

    pub async fn publish(&self, event: T) {
        let callbacks = self.callbacks.read().await;
        for callback in callbacks.iter() {
            callback(event.clone());
        }
    }

    pub async fn remove(&self, index: u64) -> bool {
        let mut callbacks = self.callbacks.write().await;
        if index as usize >= callbacks.len() {
            return false;
        }
        callbacks.remove(index as usize);
        true
    }

    pub async fn clear(&self) {
        let mut callbacks = self.callbacks.write().await;
        callbacks.clear();
    }

    pub async fn count(&self) -> usize {
        let callbacks = self.callbacks.read().await;
        callbacks.len()
    }
}

impl<T: Clone + Send + 'static> Default for EventSubscriber<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("callback not found: {0}")]
    NotFound(u64),
    #[error("subscription limit reached: {0}")]
    LimitReached(usize),
    #[error("channel error")]
    Channel,
}

#[derive(Debug, Clone)]
pub struct EventFilter {
    pub event_type: Option<String>,
    pub sender: Option<String>,
    pub module: Option<String>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            event_type: None,
            sender: None,
            module: None,
        }
    }

    pub fn with_event_type(mut self, event_type: String) -> Self {
        self.event_type = Some(event_type);
        self
    }

    pub fn with_sender(mut self, sender: String) -> Self {
        self.sender = Some(sender);
        self
    }

    pub fn with_module(mut self, module: String) -> Self {
        self.module = Some(module);
        self
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct EventManager<T> {
    subscribers: Arc<RwLock<HashMap<String, EventSubscriber<T>>>>,
}

impl<T: Clone + Send + 'static> EventManager<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, topic: String) -> EventSubscriber<T> {
        let mut subscribers = self.subscribers.write().await;
        subscribers
            .entry(topic.clone())
            .or_insert_with(EventSubscriber::new);
        subscribers.get(&topic).unwrap().clone()
    }

    pub async fn unsubscribe(&self, topic: &str) -> bool {
        let mut subscribers = self.subscribers.write().await;
        subscribers.remove(topic).is_some()
    }

    pub async fn publish(&self, topic: &str, event: T) {
        let subscribers = self.subscribers.read().await;
        if let Some(subscriber) = subscribers.get(topic) {
            let callbacks = subscriber.callbacks.read().await;
            for callback in callbacks.iter() {
                callback(event.clone());
            }
        }
    }

    pub async fn subscriber_count(&self, topic: &str) -> usize {
        let subscribers = self.subscribers.read().await;
        subscribers
            .get(topic)
            .map(|s| Arc::strong_count(&s.callbacks) - 1)
            .unwrap_or(0)
    }
}

impl<T: Clone + Send + 'static> Default for EventManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn test_event_subscriber_new() {
        let subscriber: EventSubscriber<i32> = EventSubscriber::new();
        assert_eq!(subscriber.count().await, 0);
    }

    #[tokio::test]
    async fn test_event_subscriber_on_event() {
        let subscriber: EventSubscriber<i32> = EventSubscriber::new();
        let counter = Arc::new(AtomicU32::new(0));

        let counter_clone = counter.clone();
        subscriber
            .on_event(move |value| {
                let _ = counter_clone.fetch_add(value as u32, Ordering::SeqCst);
            })
            .await;

        subscriber.publish(10).await;
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[tokio::test]
    async fn test_event_subscriber_multiple_callbacks() {
        let subscriber: EventSubscriber<i32> = EventSubscriber::new();
        let counter1 = Arc::new(AtomicU32::new(0));
        let counter2 = Arc::new(AtomicU32::new(0));

        let c1 = counter1.clone();
        let c2 = counter2.clone();
        subscriber.on_event(move |v| { let _ = c1.fetch_add(v as u32, Ordering::SeqCst); }).await;
        subscriber.on_event(move |v| { let _ = c2.fetch_add(v as u32, Ordering::SeqCst); }).await;

        subscriber.publish(5).await;

        assert_eq!(counter1.load(Ordering::SeqCst), 5);
        assert_eq!(counter2.load(Ordering::SeqCst), 5);
    }

    #[tokio::test]
    async fn test_event_subscriber_remove() {
        let subscriber: EventSubscriber<i32> = EventSubscriber::new();
        let counter = Arc::new(AtomicU32::new(0));

        let c = counter.clone();
        subscriber
            .on_event(move |v| { let _ = c.fetch_add(v as u32, Ordering::SeqCst); })
            .await;

        let removed = subscriber.remove(0).await;
        assert!(removed);
        assert_eq!(subscriber.count().await, 0);
    }

    #[tokio::test]
    async fn test_event_subscriber_remove_invalid() {
        let subscriber: EventSubscriber<i32> = EventSubscriber::new();
        let removed = subscriber.remove(999).await;
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_event_subscriber_clear() {
        let subscriber: EventSubscriber<i32> = EventSubscriber::new();
        subscriber.on_event(|_| {}).await;
        subscriber.on_event(|_| {}).await;

        subscriber.clear().await;
        assert_eq!(subscriber.count().await, 0);
    }

    #[tokio::test]
    async fn test_event_filter_new() {
        let filter = EventFilter::new();
        assert!(filter.event_type.is_none());
        assert!(filter.sender.is_none());
        assert!(filter.module.is_none());
    }

    #[tokio::test]
    async fn test_event_filter_builder() {
        let filter = EventFilter::new()
            .with_event_type("TransferEvent".to_string())
            .with_sender("0x123".to_string())
            .with_module("coin".to_string());

        assert_eq!(filter.event_type, Some("TransferEvent".to_string()));
        assert_eq!(filter.sender, Some("0x123".to_string()));
        assert_eq!(filter.module, Some("coin".to_string()));
    }

    #[tokio::test]
    async fn test_event_manager_subscribe() {
        let manager: EventManager<i32> = EventManager::new();
        let subscriber = manager.subscribe("test_topic".to_string()).await;
        assert_eq!(subscriber.count().await, 0);
    }

    #[tokio::test]
    async fn test_event_manager_publish() {
        let manager: EventManager<i32> = EventManager::new();
        let subscriber = manager.subscribe("test_topic".to_string()).await;

        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();
        subscriber
            .on_event(move |v| { let _ = c.fetch_add(v as u32, Ordering::SeqCst); })
            .await;

        manager.publish("test_topic", 10).await;
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[tokio::test]
    async fn test_event_manager_unsubscribe() {
        let manager: EventManager<i32> = EventManager::new();
        manager.subscribe("test_topic".to_string()).await;

        let unsubscribed = manager.unsubscribe("test_topic").await;
        assert!(unsubscribed);
    }

    #[tokio::test]
    async fn test_event_manager_unsubscribe_nonexistent() {
        let manager: EventManager<i32> = EventManager::new();
        let unsubscribed = manager.unsubscribe("nonexistent").await;
        assert!(!unsubscribed);
    }
}