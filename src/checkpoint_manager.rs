use std::time::Duration;
use crate::checkpoint::{Checkpoint, CheckpointStore};

/// Default TTL for checkpoints before they are considered stale.
const DEFAULT_TTL_SECS: u64 = 300;

pub struct CheckpointManager {
    store: CheckpointStore,
    ttl: Duration,
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self {
            store: CheckpointStore::new(),
            ttl: Duration::from_secs(DEFAULT_TTL_SECS),
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    /// Record a checkpoint for a process.
    pub fn record(&mut self, pid: u32, name: impl Into<String>) {
        let cp = Checkpoint::new(name, pid);
        self.store.record(cp);
    }

    /// Record a checkpoint with additional metadata.
    pub fn record_with_meta(
        &mut self,
        pid: u32,
        name: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        let cp = Checkpoint::new(name, pid).with_meta(key, value);
        self.store.record(cp);
    }

    /// Check whether a valid (non-expired) checkpoint exists.
    pub fn has_valid(&self, pid: u32, name: &str) -> bool {
        self.store
            .get(pid, name)
            .map(|cp| !cp.is_expired(self.ttl))
            .unwrap_or(false)
    }

    /// Remove a specific checkpoint.
    pub fn clear(&mut self, pid: u32, name: &str) {
        self.store.remove(pid, name);
    }

    /// Evict all expired checkpoints and return how many were removed.
    pub fn evict_expired(&mut self) -> usize {
        self.store.evict_expired(self.ttl)
    }

    pub fn count(&self) -> usize {
        self.store.len()
    }
}

impl Default for CheckpointManager {
    fn default() -> Self {
        Self::new()
    }
}
