#[cfg(test)]
mod tests {
    use crate::snapshot::{ProcessSnapshot, SnapshotStore};
    use crate::snapshot_diff::{diff_snapshots, ChangeKind};
    use crate::snapshot_manager::SnapshotManager;

    fn make_snap(pid: u32, name: &str, cpu: f64, mem: u64) -> ProcessSnapshot {
        ProcessSnapshot::new(pid, name.to_string(), cpu, mem)
    }

    #[test]
    fn test_store_insert_and_get() {
        let mut store = SnapshotStore::new();
        store.insert(make_snap(1, "nginx", 5.0, 1024));
        let s = store.get(1).expect("should exist");
        assert_eq!(s.name, "nginx");
        assert_eq!(s.cpu_percent, 5.0);
    }

    #[test]
    fn test_store_remove() {
        let mut store = SnapshotStore::new();
        store.insert(make_snap(2, "redis", 1.0, 512));
        assert!(store.remove(2).is_some());
        assert!(store.get(2).is_none());
        assert!(store.is_empty());
    }

    #[test]
    fn test_diff_appeared() {
        let curr = make_snap(10, "sshd", 0.5, 256);
        let diff = diff_snapshots(None, Some(&curr)).unwrap();
        assert_eq!(diff.kind, ChangeKind::Appeared);
        assert_eq!(diff.cpu_delta, 0.5);
    }

    #[test]
    fn test_diff_disappeared() {
        let prev = make_snap(10, "sshd", 0.5, 256);
        let diff = diff_snapshots(Some(&prev), None).unwrap();
        assert_eq!(diff.kind, ChangeKind::Disappeared);
        assert!(diff.cpu_delta < 0.0);
    }

    #[test]
    fn test_diff_changed() {
        let prev = make_snap(5, "app", 10.0, 2048);
        let curr = make_snap(5, "app", 25.0, 3000);
        let diff = diff_snapshots(Some(&prev), Some(&curr)).unwrap();
        assert_eq!(diff.kind, ChangeKind::Changed);
        assert!((diff.cpu_delta - 15.0).abs() < 0.001);
        assert_eq!(diff.mem_delta_kb, 952);
    }

    #[test]
    fn test_manager_update_produces_diffs() {
        let mut mgr = SnapshotManager::new();
        let batch1 = vec![make_snap(1, "a", 5.0, 100), make_snap(2, "b", 2.0, 200)];
        let diffs = mgr.update(batch1);
        assert_eq!(diffs.len(), 2);
        assert!(diffs.iter().all(|d| d.kind == ChangeKind::Appeared));

        let batch2 = vec![make_snap(1, "a", 10.0, 100)];
        let diffs2 = mgr.update(batch2);
        // pid 1 changed, pid 2 disappeared
        assert_eq!(diffs2.len(), 2);
        let disappeared = diffs2.iter().find(|d| d.pid == 2).unwrap();
        assert_eq!(disappeared.kind, ChangeKind::Disappeared);
    }
}
