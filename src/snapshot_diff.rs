use crate::snapshot::ProcessSnapshot;
use serde::{Deserialize, Serialize};

/// Describes how a metric changed between two snapshots.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeKind {
    Appeared,
    Disappeared,
    Changed,
    Unchanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDiff {
    pub pid: u32,
    pub name: String,
    pub kind: ChangeKind,
    pub cpu_delta: f64,
    pub mem_delta_kb: i64,
}

/// Compute the diff between a previous and current snapshot for a single process.
pub fn diff_snapshots(
    prev: Option<&ProcessSnapshot>,
    curr: Option<&ProcessSnapshot>,
) -> Option<SnapshotDiff> {
    match (prev, curr) {
        (None, Some(c)) => Some(SnapshotDiff {
            pid: c.pid,
            name: c.name.clone(),
            kind: ChangeKind::Appeared,
            cpu_delta: c.cpu_percent,
            mem_delta_kb: c.mem_rss_kb as i64,
        }),
        (Some(p), None) => Some(SnapshotDiff {
            pid: p.pid,
            name: p.name.clone(),
            kind: ChangeKind::Disappeared,
            cpu_delta: -(p.cpu_percent),
            mem_delta_kb: -(p.mem_rss_kb as i64),
        }),
        (Some(p), Some(c)) => {
            let cpu_delta = c.cpu_percent - p.cpu_percent;
            let mem_delta_kb = c.mem_rss_kb as i64 - p.mem_rss_kb as i64;
            let kind = if cpu_delta.abs() > 0.01 || mem_delta_kb != 0 {
                ChangeKind::Changed
            } else {
                ChangeKind::Unchanged
            };
            Some(SnapshotDiff { pid: c.pid, name: c.name.clone(), kind, cpu_delta, mem_delta_kb })
        }
        (None, None) => None,
    }
}
