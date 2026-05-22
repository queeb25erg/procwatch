use std::collections::HashMap;
use std::time::Duration;

use crate::history::MetricHistory;

/// Key identifying a (pid, metric_name) pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryKey {
    pub pid: u32,
    pub metric: String,
}

impl HistoryKey {
    pub fn new(pid: u32, metric: impl Into<String>) -> Self {
        Self { pid, metric: metric.into() }
    }
}

/// Central store of rolling metric histories keyed by process + metric name.
pub struct HistoryStore {
    window: Duration,
    entries: HashMap<HistoryKey, MetricHistory>,
}

impl HistoryStore {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            entries: HashMap::new(),
        }
    }

    /// Record a value for the given pid/metric, creating a history if needed.
    pub fn record(&mut self, pid: u32, metric: &str, value: f64) {
        let key = HistoryKey::new(pid, metric);
        let window = self.window;
        self.entries
            .entry(key)
            .or_insert_with(|| MetricHistory::new(window))
            .push(value);
    }

    pub fn get(&self, pid: u32, metric: &str) -> Option<&MetricHistory> {
        self.entries.get(&HistoryKey::new(pid, metric))
    }

    /// Remove all history entries for a pid (e.g. when a process exits).
    pub fn remove_pid(&mut self, pid: u32) {
        self.entries.retain(|k, _| k.pid != pid);
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
