#[cfg(test)]
mod tests {
    use crate::aggregator::Aggregator;
    use crate::metrics::ProcessMetrics;

    fn make_metrics(pid: u32, name: &str, cpu: f64, mem: u64) -> ProcessMetrics {
        ProcessMetrics {
            pid,
            name: name.to_string(),
            cpu_percent: cpu,
            mem_bytes: mem,
        }
    }

    #[test]
    fn test_aggregate_single_sample() {
        let mut agg = Aggregator::new();
        agg.add_sample(make_metrics(1, "nginx", 10.0, 1024));
        let result = agg.aggregate(1).expect("should aggregate");
        assert_eq!(result.pid, 1);
        assert_eq!(result.name, "nginx");
        assert_eq!(result.sample_count, 1);
        assert!((result.avg_cpu_percent - 10.0).abs() < 1e-9);
        assert!((result.max_cpu_percent - 10.0).abs() < 1e-9);
        assert_eq!(result.avg_mem_bytes, 1024);
        assert_eq!(result.max_mem_bytes, 1024);
    }

    #[test]
    fn test_aggregate_multiple_samples() {
        let mut agg = Aggregator::new();
        agg.add_sample(make_metrics(2, "redis", 5.0, 512));
        agg.add_sample(make_metrics(2, "redis", 15.0, 1024));
        agg.add_sample(make_metrics(2, "redis", 10.0, 768));
        let result = agg.aggregate(2).expect("should aggregate");
        assert_eq!(result.sample_count, 3);
        assert!((result.avg_cpu_percent - 10.0).abs() < 1e-9);
        assert!((result.max_cpu_percent - 15.0).abs() < 1e-9);
        assert_eq!(result.avg_mem_bytes, 768);
        assert_eq!(result.max_mem_bytes, 1024);
    }

    #[test]
    fn test_aggregate_missing_pid() {
        let agg = Aggregator::new();
        assert!(agg.aggregate(99).is_none());
    }

    #[test]
    fn test_aggregate_all() {
        let mut agg = Aggregator::new();
        agg.add_sample(make_metrics(1, "nginx", 10.0, 1024));
        agg.add_sample(make_metrics(2, "redis", 20.0, 2048));
        let all = agg.aggregate_all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_clear_pid() {
        let mut agg = Aggregator::new();
        agg.add_sample(make_metrics(1, "nginx", 10.0, 1024));
        agg.clear(1);
        assert!(agg.aggregate(1).is_none());
    }

    #[test]
    fn test_clear_all() {
        let mut agg = Aggregator::new();
        agg.add_sample(make_metrics(1, "nginx", 10.0, 1024));
        agg.add_sample(make_metrics(2, "redis", 20.0, 2048));
        agg.clear_all();
        assert!(agg.tracked_pids().is_empty());
    }
}
