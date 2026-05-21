#[cfg(test)]
mod tests {
    use crate::alert::{evaluate_alerts, AlertType};
    use crate::config::AlertConfig;
    use crate::metrics::ProcessMetrics;

    fn make_config(cpu: f64, mem_mb: f64) -> AlertConfig {
        AlertConfig {
            cpu_threshold_percent: cpu,
            memory_threshold_mb: mem_mb,
            repeat_interval_cycles: Some(5),
        }
    }

    fn make_metrics(cpu: f64, mem_bytes: u64) -> ProcessMetrics {
        ProcessMetrics {
            pid: 1234,
            name: "test_proc".to_string(),
            cpu_percent: cpu,
            memory_bytes: mem_bytes,
        }
    }

    #[test]
    fn no_alerts_when_below_thresholds() {
        let config = make_config(80.0, 512.0);
        let metrics = make_metrics(50.0, 256 * 1024 * 1024);
        let alerts = evaluate_alerts(&metrics, &config);
        assert!(alerts.is_empty());
    }

    #[test]
    fn cpu_alert_triggered() {
        let config = make_config(80.0, 512.0);
        let metrics = make_metrics(95.0, 100 * 1024 * 1024);
        let alerts = evaluate_alerts(&metrics, &config);
        assert_eq!(alerts.len(), 1);
        assert!(matches!(alerts[0].alert_type, AlertType::CpuThresholdExceeded));
        assert_eq!(alerts[0].pid, 1234);
    }

    #[test]
    fn memory_alert_triggered() {
        let config = make_config(80.0, 256.0);
        let metrics = make_metrics(10.0, 512 * 1024 * 1024);
        let alerts = evaluate_alerts(&metrics, &config);
        assert_eq!(alerts.len(), 1);
        assert!(matches!(alerts[0].alert_type, AlertType::MemoryThresholdExceeded));
    }

    #[test]
    fn both_alerts_triggered() {
        let config = make_config(50.0, 128.0);
        let metrics = make_metrics(99.0, 512 * 1024 * 1024);
        let alerts = evaluate_alerts(&metrics, &config);
        assert_eq!(alerts.len(), 2);
    }

    #[test]
    fn alert_value_and_threshold_correct() {
        let config = make_config(70.0, 512.0);
        let metrics = make_metrics(85.5, 100 * 1024 * 1024);
        let alerts = evaluate_alerts(&metrics, &config);
        assert_eq!(alerts[0].value, 85.5);
        assert_eq!(alerts[0].threshold, 70.0);
    }
}
