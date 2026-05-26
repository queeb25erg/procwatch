#[cfg(test)]
mod tests {
    use crate::registry::{Registry, RegistryEntry};
    use crate::registry_manager::RegistryManager;
    use crate::metrics::ProcessMetrics;

    fn make_metrics(cpu: f32, mem: u64) -> ProcessMetrics {
        ProcessMetrics {
            pid: 0,
            cpu_percent: cpu,
            mem_rss_kb: mem,
            threads: 1,
        }
    }

    #[test]
    fn test_register_and_get() {
        let reg = Registry::new();
        reg.register(100, "proc_a".into(), make_metrics(1.0, 512));
        let entry = reg.get(100).expect("entry should exist");
        assert_eq!(entry.pid, 100);
        assert_eq!(entry.name, "proc_a");
        assert_eq!(entry.metrics.cpu_percent, 1.0);
    }

    #[test]
    fn test_update_existing_entry() {
        let reg = Registry::new();
        reg.register(200, "proc_b".into(), make_metrics(2.0, 1024));
        reg.register(200, "proc_b".into(), make_metrics(5.0, 2048));
        let entry = reg.get(200).unwrap();
        assert_eq!(entry.metrics.cpu_percent, 5.0);
        assert_eq!(entry.metrics.mem_rss_kb, 2048);
    }

    #[test]
    fn test_unregister() {
        let reg = Registry::new();
        reg.register(300, "proc_c".into(), make_metrics(0.5, 256));
        assert!(reg.unregister(300));
        assert!(reg.get(300).is_none());
        assert!(!reg.unregister(300));
    }

    #[test]
    fn test_count_and_all() {
        let reg = Registry::new();
        reg.register(1, "a".into(), make_metrics(1.0, 100));
        reg.register(2, "b".into(), make_metrics(2.0, 200));
        reg.register(3, "c".into(), make_metrics(3.0, 300));
        assert_eq!(reg.count(), 3);
        assert_eq!(reg.all().len(), 3);
    }

    #[test]
    fn test_pids() {
        let reg = Registry::new();
        reg.register(10, "x".into(), make_metrics(0.0, 0));
        reg.register(20, "y".into(), make_metrics(0.0, 0));
        let mut pids = reg.pids();
        pids.sort();
        assert_eq!(pids, vec![10, 20]);
    }

    #[test]
    fn test_manager_summary() {
        let mgr = RegistryManager::new(60);
        mgr.track(1, "alpha".into(), make_metrics(1.0, 512));
        mgr.track(2, "beta".into(), make_metrics(2.0, 1024));
        let summary = mgr.summary();
        assert_eq!(summary.total, 2);
        assert!(summary.avg_age_secs < 5);
    }

    #[test]
    fn test_manager_remove() {
        let mgr = RegistryManager::new(60);
        mgr.track(99, "temp".into(), make_metrics(0.1, 64));
        assert!(mgr.remove(99));
        assert!(!mgr.remove(99));
    }

    #[test]
    fn test_evict_stale_none_old_enough() {
        let reg = Registry::new();
        reg.register(5, "fresh".into(), make_metrics(1.0, 128));
        let evicted = reg.evict_stale(3600);
        assert_eq!(evicted, 0);
        assert_eq!(reg.count(), 1);
    }
}
