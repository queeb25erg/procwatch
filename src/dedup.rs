//! Alert deduplication — suppresses repeated alerts for the same process/metric
//! within a configurable time window.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DedupKey {
    pub pid: u32,
    pub metric: String,
}

impl DedupKey {
    pub fn new(pid: u32, metric: impl Into<String>) -> Self {
        Self {
            pid,
            metric: metric.into(),
        }
    }
}

pub struct DedupCache {
    window: Duration,
    seen: HashMap<DedupKey, Instant>,
}

impl DedupCache {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            seen: HashMap::new(),
        }
    }

    /// Returns `true` if the alert is a duplicate and should be suppressed.
    pub fn is_duplicate(&mut self, key: &DedupKey) -> bool {
        let now = Instant::now();
        if let Some(&last_seen) = self.seen.get(key) {
            if now.duration_since(last_seen) < self.window {
                return true;
            }
        }
        self.seen.insert(key.clone(), now);
        false
    }

    /// Evict entries older than the dedup window to prevent unbounded growth.
    pub fn evict_expired(&mut self) {
        let now = Instant::now();
        self.seen
            .retain(|_, last_seen| now.duration_since(*last_seen) < self.window);
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}
