#[cfg(test)]
mod tests {
    use crate::baseline::{Baseline, BaselineStore};
    use crate::baseline_checker::{BaselineChecker, BaselineCheckerConfig};
    use crate::baseline_manager::BaselineManager;
    use crate::metrics::ProcessMetrics;

    fn make_metrics(pid: u32, cpu: f64, mem: f64) -> ProcessMetrics {
        ProcessMetrics {
            pid,
            name: format!("proc_{}", pid),
            cpu_percent: cpu,
            mem_mb: mem,
        }
    }

    #[test]
    fn test_baseline_update_averages() {
        let mut b = Baseline::new(1);
        b.update(&make_metrics(1, 10.0, 100.0));
        b.update(&make_metrics(1, 20.0, 200.0));
        assert!((b.avg_cpu - 15.0).abs() < 0.01);
        assert!((b.avg_mem - 150.0).abs() < 0.01);
        assert_eq!(b.sample_count, 2);
    }

    #[test]
    fn test_baseline_maturity() {
        let mut b = Baseline::new(1);
        assert!(!b.is_mature(3));
        for _ in 0..3 {
            b.update(&make_metrics(1, 5.0, 50.0));
        }
        assert!(b.is_mature(3));
    }

    #[test]
    fn test_deviation_calculation() {
        let mut b = Baseline::new(1);
        for _ in 0..5 {
            b.update(&make_metrics(1, 10.0, 100.0));
        }
        let dev = b.cpu_deviation(15.0);
        assert!((dev - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_checker_no_anomaly_before_maturity() {
        let config = BaselineCheckerConfig { min_samples: 5, ..Default::default() };
        let checker = BaselineChecker::new(config);
        let mut store = BaselineStore::new();
        let m = make_metrics(1, 100.0, 500.0);
        store.update(&m);
        assert!(checker.check(&m, &store).is_none());
    }

    #[test]
    fn test_manager_detects_anomaly() {
        let config = BaselineCheckerConfig {
            min_samples: 3,
            cpu_deviation_threshold_pct: 40.0,
            mem_deviation_threshold_pct: 40.0,
        };
        let mut mgr = BaselineManager::new(config);
        for _ in 0..4 {
            mgr.process(&make_metrics(42, 10.0, 100.0));
        }
        // Spike: CPU goes 5x baseline
        let result = mgr.process(&make_metrics(42, 50.0, 100.0));
        assert!(result.is_some());
        let (anomaly, msg, severity) = result.unwrap();
        assert_eq!(anomaly.pid, 42);
        assert!(msg.contains("BASELINE"));
        assert!(severity == "warning" || severity == "critical");
    }

    #[test]
    fn test_manager_evict() {
        let mut mgr = BaselineManager::default();
        mgr.process(&make_metrics(7, 5.0, 50.0));
        assert_eq!(mgr.baseline_count(), 1);
        mgr.evict(7);
        assert_eq!(mgr.baseline_count(), 0);
    }
}
