#[cfg(test)]
mod tests {
    use super::super::dedup::{DedupCache, DedupKey};
    use std::thread;
    use std::time::Duration;

    fn key(pid: u32, metric: &str) -> DedupKey {
        DedupKey::new(pid, metric)
    }

    #[test]
    fn first_occurrence_is_not_duplicate() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        let k = key(1234, "cpu");
        assert!(!cache.is_duplicate(&k));
    }

    #[test]
    fn second_occurrence_within_window_is_duplicate() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        let k = key(1234, "cpu");
        cache.is_duplicate(&k);
        assert!(cache.is_duplicate(&k));
    }

    #[test]
    fn different_metric_is_not_duplicate() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        let k1 = key(1234, "cpu");
        let k2 = key(1234, "mem");
        cache.is_duplicate(&k1);
        assert!(!cache.is_duplicate(&k2));
    }

    #[test]
    fn different_pid_is_not_duplicate() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        let k1 = key(1234, "cpu");
        let k2 = key(5678, "cpu");
        cache.is_duplicate(&k1);
        assert!(!cache.is_duplicate(&k2));
    }

    #[test]
    fn occurrence_after_window_is_not_duplicate() {
        let mut cache = DedupCache::new(Duration::from_millis(50));
        let k = key(1234, "cpu");
        cache.is_duplicate(&k);
        thread::sleep(Duration::from_millis(80));
        assert!(!cache.is_duplicate(&k));
    }

    #[test]
    fn evict_expired_removes_old_entries() {
        let mut cache = DedupCache::new(Duration::from_millis(50));
        cache.is_duplicate(&key(1, "cpu"));
        cache.is_duplicate(&key(2, "mem"));
        assert_eq!(cache.len(), 2);
        thread::sleep(Duration::from_millis(80));
        cache.evict_expired();
        assert!(cache.is_empty());
    }

    #[test]
    fn evict_keeps_fresh_entries() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        cache.is_duplicate(&key(1, "cpu"));
        cache.evict_expired();
        assert_eq!(cache.len(), 1);
    }
}
