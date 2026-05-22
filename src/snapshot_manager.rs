use crate::snapshot::{ProcessSnapshot, SnapshotStore};
use crate::snapshot_diff::{diff_snapshots, SnapshotDiff};
use std::collections::HashSet;

/// Manages rolling snapshots and produces diffs on each cycle.
pub struct SnapshotManager {
    previous: SnapshotStore,
    current: SnapshotStore,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self {
            previous: SnapshotStore::new(),
            current: SnapshotStore::new(),
        }
    }

    /// Replace current snapshots with a fresh batch and compute diffs.
    pub fn update(&mut self, snapshots: Vec<ProcessSnapshot>) -> Vec<SnapshotDiff> {
        // Rotate: current becomes previous
        std::mem::swap(&mut self.previous, &mut self.current);
        self.current = SnapshotStore::new();

        for snap in snapshots {
            self.current.insert(snap);
        }

        self.compute_diffs()
    }

    fn compute_diffs(&self) -> Vec<SnapshotDiff> {
        let mut pids: HashSet<u32> = HashSet::new();
        for s in self.previous.all() { pids.insert(s.pid); }
        for s in self.current.all()  { pids.insert(s.pid); }

        pids.into_iter()
            .filter_map(|pid| {
                diff_snapshots(self.previous.get(pid), self.current.get(pid))
            })
            .collect()
    }

    pub fn current_snapshot(&self, pid: u32) -> Option<&ProcessSnapshot> {
        self.current.get(pid)
    }

    pub fn current_count(&self) -> usize {
        self.current.len()
    }
}

impl Default for SnapshotManager {
    fn default() -> Self { Self::new() }
}
