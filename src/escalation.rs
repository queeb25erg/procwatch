use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks how many times an alert has fired for a given process/metric key
/// and determines whether the alert should be escalated based on thresholds.
#[derive(Debug)]
pub struct EscalationTracker {
    counts: HashMap<String, EscalationEntry>,
    warn_threshold: u32,
    critical_threshold: u32,
    window: Duration,
}

#[derive(Debug, Clone)]
struct EscalationEntry {
    count: u32,
    window_start: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EscalationLevel {
    Normal,
    Warning,
    Critical,
}

impl EscalationTracker {
    pub fn new(warn_threshold: u32, critical_threshold: u32, window: Duration) -> Self {
        Self {
            counts: HashMap::new(),
            warn_threshold,
            critical_threshold,
            window,
        }
    }

    /// Record a new alert firing for the given key and return the escalation level.
    pub fn record(&mut self, key: &str) -> EscalationLevel {
        let now = Instant::now();
        let entry = self.counts.entry(key.to_string()).or_insert(EscalationEntry {
            count: 0,
            window_start: now,
        });

        if now.duration_since(entry.window_start) > self.window {
            entry.count = 1;
            entry.window_start = now;
        } else {
            entry.count += 1;
        }

        let count = entry.count;
        if count >= self.critical_threshold {
            EscalationLevel::Critical
        } else if count >= self.warn_threshold {
            EscalationLevel::Warning
        } else {
            EscalationLevel::Normal
        }
    }

    /// Reset the escalation state for a given key (e.g., when alert resolves).
    pub fn reset(&mut self, key: &str) {
        self.counts.remove(key);
    }

    /// Return the current fire count for a key within the active window.
    pub fn count(&self, key: &str) -> u32 {
        self.counts.get(key).map(|e| e.count).unwrap_or(0)
    }
}
