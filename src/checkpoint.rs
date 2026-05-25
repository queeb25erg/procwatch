use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A named checkpoint that records when a process metric was last observed
/// within acceptable bounds.
#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub name: String,
    pub pid: u32,
    pub recorded_at: Instant,
    pub metadata: HashMap<String, String>,
}

impl Checkpoint {
    pub fn new(name: impl Into<String>, pid: u32) -> Self {
        Self {
            name: name.into(),
            pid,
            recorded_at: Instant::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn age(&self) -> Duration {
        self.recorded_at.elapsed()
    }

    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.age() > ttl
    }
}

/// Stores and manages checkpoints keyed by (pid, name).
#[derive(Debug, Default)]
pub struct CheckpointStore {
    entries: HashMap<(u32, String), Checkpoint>,
}

impl CheckpointStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, checkpoint: Checkpoint) {
        let key = (checkpoint.pid, checkpoint.name.clone());
        self.entries.insert(key, checkpoint);
    }

    pub fn get(&self, pid: u32, name: &str) -> Option<&Checkpoint> {
        self.entries.get(&(pid, name.to_string()))
    }

    pub fn remove(&mut self, pid: u32, name: &str) -> Option<Checkpoint> {
        self.entries.remove(&(pid, name.to_string()))
    }

    pub fn evict_expired(&mut self, ttl: Duration) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, cp| !cp.is_expired(ttl));
        before - self.entries.len()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
