use std::thread;
use std::time::Duration;

/// Retry policy for failed transfers
#[derive(Clone)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub base_delay_ms: u64,
}

impl RetryPolicy {
    pub fn new() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 200,
        }
    }

    pub fn execute<F, T>(&self, mut f: F) -> anyhow::Result<T>
    where
        F: FnMut() -> anyhow::Result<T>,
    {
        let mut attempt = 0;

        loop {
            attempt += 1;

            match f() {
                Ok(val) => return Ok(val),
                Err(err) if attempt < self.max_attempts => {
                    let delay = self.base_delay_ms * attempt as u64;
                    println!(
                        "[PTFS-RETRY] attempt {} failed, retrying in {}ms",
                        attempt, delay
                    );
                    thread::sleep(Duration::from_millis(delay));
                }
                Err(err) => return Err(err),
            }
        }
    }
}
