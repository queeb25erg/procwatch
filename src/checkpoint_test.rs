#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::checkpoint::{Checkpoint, CheckpointStore};
    use crate::checkpoint_manager::CheckpointManager;

    #[test]
    fn test_checkpoint_age_and_expiry() {
        let cp = Checkpoint::new("cpu_ok", 1234);
        assert!(!cp.is_expired(Duration::from_secs(60)));
        assert!(cp.age() < Duration::from_secs(1));
    }

    #[test]
    fn test_checkpoint_with_meta() {
        let cp = Checkpoint::new("mem_ok", 42)
            .with_meta("threshold", "80%");
        assert_eq!(cp.metadata.get("threshold"), Some(&"80%".to_string()));
    }

    #[test]
    fn test_store_record_and_get() {
        let mut store = CheckpointStore::new();
        let cp = Checkpoint::new("cpu_ok", 100);
        store.record(cp);
        assert!(store.get(100, "cpu_ok").is_some());
        assert!(store.get(100, "mem_ok").is_none());
    }

    #[test]
    fn test_store_remove() {
        let mut store = CheckpointStore::new();
        store.record(Checkpoint::new("cpu_ok", 200));
        assert_eq!(store.len(), 1);
        store.remove(200, "cpu_ok");
        assert!(store.is_empty());
    }

    #[test]
    fn test_store_evict_expired() {
        let mut store = CheckpointStore::new();
        store.record(Checkpoint::new("cpu_ok", 300));
        store.record(Checkpoint::new("mem_ok", 300));
        // Nothing expired yet with a large TTL
        let evicted = store.evict_expired(Duration::from_secs(60));
        assert_eq!(evicted, 0);
        assert_eq!(store.len(), 2);
        // Evict with zero TTL — all should be gone
        let evicted = store.evict_expired(Duration::from_nanos(0));
        assert_eq!(evicted, 2);
        assert!(store.is_empty());
    }

    #[test]
    fn test_manager_record_and_has_valid() {
        let mut mgr = CheckpointManager::new();
        mgr.record(1, "cpu_ok");
        assert!(mgr.has_valid(1, "cpu_ok"));
        assert!(!mgr.has_valid(1, "mem_ok"));
    }

    #[test]
    fn test_manager_expired_checkpoint_invalid() {
        let mut mgr = CheckpointManager::new().with_ttl(Duration::from_nanos(1));
        mgr.record(2, "cpu_ok");
        std::thread::sleep(Duration::from_millis(5));
        assert!(!mgr.has_valid(2, "cpu_ok"));
    }

    #[test]
    fn test_manager_evict_and_count() {
        let mut mgr = CheckpointManager::new().with_ttl(Duration::from_nanos(1));
        mgr.record(3, "cpu_ok");
        mgr.record(3, "mem_ok");
        std::thread::sleep(Duration::from_millis(5));
        let removed = mgr.evict_expired();
        assert_eq!(removed, 2);
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_manager_clear() {
        let mut mgr = CheckpointManager::new();
        mgr.record(4, "cpu_ok");
        mgr.clear(4, "cpu_ok");
        assert!(!mgr.has_valid(4, "cpu_ok"));
    }
}
