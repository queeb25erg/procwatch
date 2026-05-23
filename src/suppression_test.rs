#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::suppression::SuppressionStore;
    use crate::suppression_manager::SuppressionManager;

    #[test]
    fn test_not_suppressed_initially() {
        let store = SuppressionStore::new(Duration::from_secs(60));
        assert!(!store.is_suppressed("nginx/cpu"));
    }

    #[test]
    fn test_suppress_marks_key() {
        let mut store = SuppressionStore::new(Duration::from_secs(60));
        store.suppress("nginx/cpu");
        assert!(store.is_suppressed("nginx/cpu"));
    }

    #[test]
    fn test_lift_clears_suppression() {
        let mut store = SuppressionStore::new(Duration::from_secs(60));
        store.suppress("nginx/cpu");
        store.lift("nginx/cpu");
        assert!(!store.is_suppressed("nginx/cpu"));
    }

    #[test]
    fn test_suppression_count_increments() {
        let mut store = SuppressionStore::new(Duration::from_secs(60));
        store.suppress("nginx/cpu");
        store.suppress("nginx/cpu");
        store.suppress("nginx/cpu");
        assert_eq!(store.suppression_count("nginx/cpu"), 3);
    }

    #[test]
    fn test_expired_suppression_evicted() {
        let mut store = SuppressionStore::new(Duration::from_nanos(1));
        store.suppress("nginx/cpu");
        std::thread::sleep(Duration::from_millis(5));
        store.evict_expired();
        assert!(!store.is_suppressed("nginx/cpu"));
    }

    #[test]
    fn test_manager_first_alert_fires() {
        let mut mgr = SuppressionManager::new(60);
        assert!(mgr.should_alert("nginx", "cpu"));
    }

    #[test]
    fn test_manager_second_alert_suppressed() {
        let mut mgr = SuppressionManager::new(60);
        mgr.should_alert("nginx", "cpu");
        assert!(!mgr.should_alert("nginx", "cpu"));
    }

    #[test]
    fn test_manager_lift_allows_re_alert() {
        let mut mgr = SuppressionManager::new(60);
        mgr.should_alert("nginx", "cpu");
        mgr.lift("nginx", "cpu");
        assert!(mgr.should_alert("nginx", "cpu"));
    }

    #[test]
    fn test_manager_independent_keys() {
        let mut mgr = SuppressionManager::new(60);
        mgr.should_alert("nginx", "cpu");
        assert!(mgr.should_alert("nginx", "mem"));
        assert!(mgr.should_alert("redis", "cpu"));
    }
}
