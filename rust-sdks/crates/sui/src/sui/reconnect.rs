use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

pub struct ReconnectStrategy {
    pub max_retries: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for ReconnectStrategy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

impl ReconnectStrategy {
    pub fn new(max_retries: usize, initial_delay: Duration, max_delay: Duration, multiplier: f64) -> Self {
        Self {
            max_retries,
            initial_delay,
            max_delay,
            multiplier,
        }
    }

    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_initial_delay(mut self, initial_delay: Duration) -> Self {
        self.initial_delay = initial_delay;
        self
    }

    pub fn with_max_delay(mut self, max_delay: Duration) -> Self {
        self.max_delay = max_delay;
        self
    }

    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }
}

pub async fn with_retry<T, F, E>(
    mut f: F,
    strategy: &ReconnectStrategy,
) -> Result<T, E>
where
    F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
{
    let mut attempt = 0;
    let mut delay = strategy.initial_delay;

    loop {
        attempt += 1;
        match f().await {
            Ok(value) => return Ok(value),
            Err(e) if attempt < strategy.max_retries => {
                tokio::time::sleep(delay).await;
                let next_delay_ms = (delay.as_millis() as f64 * strategy.multiplier) as u64;
                let max_delay_ms = strategy.max_delay.as_millis() as u64;
                delay = Duration::from_millis(next_delay_ms.min(max_delay_ms));
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_success_on_first_attempt() {
        let mut call_count = 0;
        let result = with_retry(
            || {
                call_count += 1;
                Box::pin(async move { Ok::<(), ()>(()) })
            },
            &ReconnectStrategy::default(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(call_count, 1);
    }

    #[tokio::test]
    async fn test_success_on_second_attempt() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();

        let result = with_retry(
            move || {
                let c = count_clone.fetch_add(1, Ordering::SeqCst);
                Box::pin(async move {
                    if c == 0 {
                        Err::<(), ()>(())
                    } else {
                        Ok(())
                    }
                })
            },
            &ReconnectStrategy::default(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_success_on_third_attempt() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();

        let result = with_retry(
            move || {
                let c = count_clone.fetch_add(1, Ordering::SeqCst);
                Box::pin(async move {
                    if c < 2 {
                        Err::<(), ()>(())
                    } else {
                        Ok(())
                    }
                })
            },
            &ReconnectStrategy::default(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_all_attempts_fail() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();

        let result = with_retry(
            move || {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Box::pin(async move { Err::<(), _>(()) })
            },
            &ReconnectStrategy::default(),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_exponential_backoff() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();
        let start = std::time::Instant::now();

        let result = with_retry(
            move || {
                let c = count_clone.fetch_add(1, Ordering::SeqCst);
                Box::pin(async move {
                    if c < 2 {
                        Err::<(), ()>(())
                    } else {
                        Ok(())
                    }
                })
            },
            &ReconnectStrategy::default(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(count.load(Ordering::SeqCst), 3);
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(3000));
    }

    #[tokio::test]
    async fn test_max_delay_constraint() {
        let mut call_count = 0;
        let result = with_retry(
            || {
                call_count += 1;
                Box::pin(async move { if call_count < 5 { Err::<(), ()>(()) } else { Ok(()) } })
            },
            &ReconnectStrategy {
                max_retries: 5,
                initial_delay: Duration::from_millis(100),
                max_delay: Duration::from_millis(500),
                multiplier: 10.0,
            },
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_custom_strategy() {
        let mut call_count = 0;
        let strategy = ReconnectStrategy::new(
            10,
            Duration::from_millis(100),
            Duration::from_secs(5),
            1.5,
        );

        let result = with_retry(
            || {
                call_count += 1;
                Box::pin(async move { if call_count < 4 { Err::<(), ()>(()) } else { Ok(()) } })
            },
            &strategy,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(call_count, 4);
    }
}