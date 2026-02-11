use std::future::Future;

pub async fn retry<F, Fut, T, E>(mut f: F, times: usize) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempts = 0usize;
    loop {
        attempts += 1;
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) if attempts >= times => return Err(e),
            Err(_) => continue,
        }
    }
}
