use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// A point-in-time snapshot of all monitored process metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub mem_rss_kb: u64,
    pub timestamp: u64,
}

impl ProcessSnapshot {
    pub fn new(pid: u32, name: String, cpu_percent: f64, mem_rss_kb: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self { pid, name, cpu_percent, mem_rss_kb, timestamp }
    }
}

/// Holds the latest snapshot per PID.
#[derive(Debug, Default)]
pub struct SnapshotStore {
    snapshots: HashMap<u32, ProcessSnapshot>,
}

impl SnapshotStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, snapshot: ProcessSnapshot) {
        self.snapshots.insert(snapshot.pid, snapshot);
    }

    pub fn get(&self, pid: u32) -> Option<&ProcessSnapshot> {
        self.snapshots.get(&pid)
    }

    pub fn all(&self) -> Vec<&ProcessSnapshot> {
        self.snapshots.values().collect()
    }

    pub fn remove(&mut self, pid: u32) -> Option<ProcessSnapshot> {
        self.snapshots.remove(&pid)
    }

    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
}
