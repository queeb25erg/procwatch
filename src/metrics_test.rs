#[cfg(test)]
mod tests {
    use std::process;
    use crate::metrics;

    #[test]
    fn test_collect_current_process() {
        let pid = process::id();
        let result = metrics::collect(pid);
        assert!(result.is_ok(), "Failed to collect metrics for current process: {:?}", result.err());

        let m = result.unwrap();
        assert_eq!(m.pid, pid);
        assert!(!m.name.is_empty(), "Process name should not be empty");
        assert!(m.memory_bytes > 0, "Memory usage should be greater than zero");
        assert!(
            m.memory_percent > 0.0 && m.memory_percent <= 100.0,
            "Memory percent out of range: {}",
            m.memory_percent
        );
        assert!(
            m.cpu_percent >= 0.0 && m.cpu_percent <= 100.0,
            "CPU percent out of range: {}",
            m.cpu_percent
        );
    }

    #[test]
    fn test_collect_invalid_pid() {
        // PID 0 should not exist as a readable process
        let result = metrics::collect(0);
        assert!(result.is_err(), "Expected error for PID 0");
    }

    #[test]
    fn test_metrics_debug_format() {
        let pid = process::id();
        if let Ok(m) = metrics::collect(pid) {
            let debug_str = format!("{:?}", m);
            assert!(debug_str.contains("ProcessMetrics"));
            assert!(debug_str.contains("pid"));
            assert!(debug_str.contains("memory_bytes"));
        }
    }

    #[test]
    fn test_metrics_clone() {
        let pid = process::id();
        if let Ok(m) = metrics::collect(pid) {
            let cloned = m.clone();
            assert_eq!(cloned.pid, m.pid);
            assert_eq!(cloned.name, m.name);
            assert_eq!(cloned.memory_bytes, m.memory_bytes);
        }
    }
}
