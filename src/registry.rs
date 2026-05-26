use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::metrics::ProcessMetrics;

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub pid: u32,
    pub name: String,
    pub metrics: ProcessMetrics,
    pub registered_at: std::time::Instant,
    pub last_seen: std::time::Instant,
}

impl RegistryEntry {
    pub fn new(pid: u32, name: String, metrics: ProcessMetrics) -> Self {
        let now = std::time::Instant::now();
        Self {
            pid,
            name,
            metrics,
            registered_at: now,
            last_seen: now,
        }
    }

    pub fn update(&mut self, metrics: ProcessMetrics) {
        self.metrics = metrics;
        self.last_seen = std::time::Instant::now();
    }

    pub fn age_secs(&self) -> u64 {
        self.registered_at.elapsed().as_secs()
    }

    pub fn stale_secs(&self) -> u64 {
        self.last_seen.elapsed().as_secs()
    }
}

#[derive(Clone, Default)]
pub struct Registry {
    entries: Arc<RwLock<HashMap<u32, RegistryEntry>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, pid: u32, name: String, metrics: ProcessMetrics) {
        let mut map = self.entries.write().unwrap();
        map.entry(pid)
            .and_modify(|e| e.update(metrics.clone()))
            .or_insert_with(|| RegistryEntry::new(pid, name, metrics));
    }

    pub fn unregister(&self, pid: u32) -> bool {
        self.entries.write().unwrap().remove(&pid).is_some()
    }

    pub fn get(&self, pid: u32) -> Option<RegistryEntry> {
        self.entries.read().unwrap().get(&pid).cloned()
    }

    pub fn all(&self) -> Vec<RegistryEntry> {
        self.entries.read().unwrap().values().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.entries.read().unwrap().len()
    }

    pub fn evict_stale(&self, max_stale_secs: u64) -> usize {
        let mut map = self.entries.write().unwrap();
        let before = map.len();
        map.retain(|_, e| e.stale_secs() < max_stale_secs);
        before - map.len()
    }

    pub fn pids(&self) -> Vec<u32> {
        self.entries.read().unwrap().keys().cloned().collect()
    }
}
