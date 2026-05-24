use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct QuotaEntry {
    pub limit: f64,
    pub used: f64,
    pub window: Duration,
    pub window_start: Instant,
}

impl QuotaEntry {
    pub fn new(limit: f64, window: Duration) -> Self {
        Self {
            limit,
            used: 0.0,
            window,
            window_start: Instant::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.window_start.elapsed() >= self.window
    }

    pub fn reset(&mut self) {
        self.used = 0.0;
        self.window_start = Instant::now();
    }

    pub fn remaining(&self) -> f64 {
        (self.limit - self.used).max(0.0)
    }

    pub fn exceeded(&self) -> bool {
        self.used >= self.limit
    }
}

#[derive(Debug, Default)]
pub struct QuotaTracker {
    entries: HashMap<String, QuotaEntry>,
}

impl QuotaTracker {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: &str, limit: f64, window: Duration) {
        self.entries
            .entry(key.to_string())
            .or_insert_with(|| QuotaEntry::new(limit, window));
    }

    pub fn consume(&mut self, key: &str, amount: f64) -> bool {
        if let Some(entry) = self.entries.get_mut(key) {
            if entry.is_expired() {
                entry.reset();
            }
            if entry.exceeded() {
                return false;
            }
            entry.used += amount;
            true
        } else {
            false
        }
    }

    pub fn remaining(&mut self, key: &str) -> Option<f64> {
        if let Some(entry) = self.entries.get_mut(key) {
            if entry.is_expired() {
                entry.reset();
            }
            Some(entry.remaining())
        } else {
            None
        }
    }

    pub fn reset(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.reset();
        }
    }

    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }
}
