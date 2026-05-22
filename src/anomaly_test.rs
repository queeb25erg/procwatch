#[cfg(test)]
mod tests {
    use crate::anomaly::{AnomalyDetector, AnomalyKind};
    use crate::anomaly_manager::AnomalyManager;
    use crate::baseline::Baseline;
    use crate::metrics::ProcessMetrics;

    fn make_baseline(cpu_mean: f64, cpu_stddev: f64, mem_mean: f64, mem_stddev: f64) -> Baseline {
        Baseline {
            pid: 1,
            process_name: "test".into(),
            cpu_mean,
            cpu_stddev,
            mem_mean,
            mem_stddev,
            sample_count: 30,
        }
    }

    fn make_metrics(pid: u32, cpu: f64, mem: f64) -> ProcessMetrics {
        ProcessMetrics {
            pid,
            name: "test".into(),
            cpu_usage: cpu,
            memory_mb: mem,
        }
    }

    #[test]
    fn test_no_anomaly_within_threshold() {
        let detector = AnomalyDetector::new(2.0, 2.0);
        let baseline = make_baseline(10.0, 2.0, 100.0, 10.0);
        let metrics = make_metrics(1, 11.0, 105.0);
        let anomalies = detector.detect(&metrics, &baseline);
        assert!(anomalies.is_empty());
    }

    #[test]
    fn test_cpu_spike_detected() {
        let detector = AnomalyDetector::new(2.0, 2.0);
        let baseline = make_baseline(10.0, 2.0, 100.0, 10.0);
        let metrics = make_metrics(1, 20.0, 100.0);
        let anomalies = detector.detect(&metrics, &baseline);
        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0].kind, AnomalyKind::CpuSpike);
    }

    #[test]
    fn test_memory_spike_detected() {
        let detector = AnomalyDetector::new(2.0, 2.0);
        let baseline = make_baseline(10.0, 2.0, 100.0, 10.0);
        let metrics = make_metrics(1, 10.0, 150.0);
        let anomalies = detector.detect(&metrics, &baseline);
        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0].kind, AnomalyKind::MemorySpike);
    }

    #[test]
    fn test_manager_no_baseline_returns_empty() {
        let manager = AnomalyManager::new(2.0, 2.0);
        let metrics = make_metrics(99, 80.0, 500.0);
        assert!(manager.check(&metrics).is_empty());
    }

    #[test]
    fn test_manager_registers_and_detects() {
        let mut manager = AnomalyManager::new(2.0, 2.0);
        let baseline = make_baseline(10.0, 2.0, 100.0, 10.0);
        manager.register_baseline(1, baseline);
        assert_eq!(manager.baseline_count(), 1);
        let metrics = make_metrics(1, 25.0, 100.0);
        let anomalies = manager.check(&metrics);
        assert!(!anomalies.is_empty());
    }

    #[test]
    fn test_manager_remove_baseline() {
        let mut manager = AnomalyManager::new(2.0, 2.0);
        let baseline = make_baseline(10.0, 2.0, 100.0, 10.0);
        manager.register_baseline(1, baseline);
        manager.remove_baseline(1);
        assert_eq!(manager.baseline_count(), 0);
    }
}
