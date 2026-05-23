use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks suppressed alerts to avoid re-alerting during a suppression window.
pub struct SuppressionStore {
    entries: HashMap<String, SuppressionEntry>,
    window: Duration,
}

struct SuppressionEntry {
    suppressed_at: Instant,
    count: u32,
}

impl SuppressionStore {
    pub fn new(window: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            window,
        }
    }

    /// Returns true if the key is currently suppressed.
    pub fn is_suppressed(&self, key: &str) -> bool {
        if let Some(entry) = self.entries.get(key) {
            entry.suppressed_at.elapsed() < self.window
        } else {
            false
        }
    }

    /// Suppress the given key, resetting the window.
    pub fn suppress(&mut self, key: &str) {
        let entry = self.entries.entry(key.to_string()).or_insert(SuppressionEntry {
            suppressed_at: Instant::now(),
            count: 0,
        });
        entry.suppressed_at = Instant::now();
        entry.count += 1;
    }

    /// Returns how many times this key has been suppressed.
    pub fn suppression_count(&self, key: &str) -> u32 {
        self.entries.get(key).map(|e| e.count).unwrap_or(0)
    }

    /// Remove expired suppressions to free memory.
    pub fn evict_expired(&mut self) {
        self.entries.retain(|_, entry| entry.suppressed_at.elapsed() < self.window);
    }

    /// Lift suppression for a key immediately.
    pub fn lift(&mut self, key: &str) {
        self.entries.remove(key);
    }
}
