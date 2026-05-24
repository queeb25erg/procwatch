#[cfg(test)]
mod tests {
    use crate::quota::{QuotaEntry, QuotaTracker};
    use crate::quota_manager::QuotaManager;
    use crate::quota_reporter::QuotaReporter;
    use std::time::Duration;

    #[test]
    fn test_quota_entry_consume_and_remaining() {
        let mut entry = QuotaEntry::new(100.0, Duration::from_secs(60));
        assert_eq!(entry.remaining(), 100.0);
        assert!(!entry.exceeded());
    }

    #[test]
    fn test_quota_entry_exceeds_limit() {
        let mut entry = QuotaEntry::new(10.0, Duration::from_secs(60));
        entry.used = 10.0;
        assert!(entry.exceeded());
        assert_eq!(entry.remaining(), 0.0);
    }

    #[test]
    fn test_quota_entry_reset() {
        let mut entry = QuotaEntry::new(50.0, Duration::from_secs(60));
        entry.used = 30.0;
        entry.reset();
        assert_eq!(entry.used, 0.0);
        assert_eq!(entry.remaining(), 50.0);
    }

    #[test]
    fn test_tracker_consume_allows_within_limit() {
        let mut tracker = QuotaTracker::new();
        tracker.register("proc:1", 100.0, Duration::from_secs(60));
        assert!(tracker.consume("proc:1", 50.0));
        assert!(tracker.consume("proc:1", 49.0));
    }

    #[test]
    fn test_tracker_consume_blocks_over_limit() {
        let mut tracker = QuotaTracker::new();
        tracker.register("proc:2", 10.0, Duration::from_secs(60));
        assert!(tracker.consume("proc:2", 10.0));
        assert!(!tracker.consume("proc:2", 1.0));
    }

    #[test]
    fn test_tracker_unknown_key_returns_false() {
        let mut tracker = QuotaTracker::new();
        assert!(!tracker.consume("unknown", 5.0));
        assert!(tracker.remaining("unknown").is_none());
    }

    #[test]
    fn test_manager_register_and_check() {
        let manager = QuotaManager::new(Duration::from_secs(60));
        manager.register_process(42, 80.0, 512.0);
        assert!(manager.check_cpu(42, 40.0));
        assert!(manager.check_memory(42, 256.0));
        assert!(manager.remaining_cpu(42).unwrap() > 0.0);
    }

    #[test]
    fn test_manager_deregister_removes_entries() {
        let manager = QuotaManager::new(Duration::from_secs(60));
        manager.register_process(99, 50.0, 128.0);
        manager.deregister_process(99);
        assert!(manager.remaining_cpu(99).is_none());
        assert!(manager.remaining_memory(99).is_none());
    }

    #[test]
    fn test_reporter_exhausted_processes() {
        let manager = QuotaManager::new(Duration::from_secs(60));
        manager.register_process(1, 10.0, 100.0);
        manager.register_process(2, 10.0, 100.0);
        // exhaust cpu for pid 1
        manager.check_cpu(1, 10.0);
        let reporter = QuotaReporter::new(manager);
        let exhausted = reporter.exhausted_processes(&[1, 2]);
        assert_eq!(exhausted.len(), 1);
        assert_eq!(exhausted[0].pid, 1);
        assert!(exhausted[0].cpu_exhausted);
    }

    #[test]
    fn test_report_summary_format() {
        let manager = QuotaManager::new(Duration::from_secs(60));
        manager.register_process(7, 100.0, 200.0);
        let reporter = QuotaReporter::new(manager);
        let report = reporter.report(7);
        let summary = report.summary();
        assert!(summary.contains("pid=7"));
        assert!(summary.contains("exhausted=false"));
    }
}
