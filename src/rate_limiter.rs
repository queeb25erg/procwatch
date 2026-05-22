use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks per-process alert rate limiting using a token bucket approach.
/// Each process gets a fixed number of tokens that refill over time.
pub struct RateLimiter {
    buckets: HashMap<String, TokenBucket>,
    max_tokens: u32,
    refill_interval: Duration,
}

struct TokenBucket {
    tokens: u32,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_tokens: u32, refill_interval: Duration) -> Self {
        RateLimiter {
            buckets: HashMap::new(),
            max_tokens,
            refill_interval,
        }
    }

    /// Returns true if the alert for the given process key is allowed.
    pub fn allow(&mut self, key: &str) -> bool {
        let now = Instant::now();
        let max_tokens = self.max_tokens;
        let refill_interval = self.refill_interval;

        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| TokenBucket {
            tokens: max_tokens,
            last_refill: now,
        });

        let elapsed = now.duration_since(bucket.last_refill);
        if elapsed >= refill_interval {
            let refills = (elapsed.as_secs_f64() / refill_interval.as_secs_f64()) as u32;
            bucket.tokens = (bucket.tokens + refills).min(max_tokens);
            bucket.last_refill = now;
        }

        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Removes stale buckets that haven't been used for more than 10x the refill interval.
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        let stale_threshold = self.refill_interval * 10;
        self.buckets.retain(|_, bucket| {
            now.duration_since(bucket.last_refill) < stale_threshold
        });
    }

    pub fn bucket_count(&self) -> usize {
        self.buckets.len()
    }
}
