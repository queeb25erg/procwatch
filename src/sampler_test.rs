#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use crate::sampler::Sampler;
    use crate::metrics::ProcessMetrics;

    fn make_metrics(pid: u32, cpu: f32, mem: u64) -> ProcessMetrics {
        ProcessMetrics {
            pid,
            name: format!("proc_{}", pid),
            cpu_percent: cpu,
            mem_rss_kb: mem,
        }
    }

    #[test]
    fn test_should_sample_initially() {
        let sampler = Sampler::new(5, 10);
        assert!(sampler.should_sample(), "should sample immediately on first call");
    }

    #[test]
    fn test_should_not_sample_immediately_after_record() {
        let mut sampler = Sampler::new(60, 10);
        sampler.record(make_metrics(1, 10.0, 1024));
        assert!(!sampler.should_sample(), "should not sample again right away");
    }

    #[test]
    fn test_should_sample_after_interval() {
        let mut sampler = Sampler::new(0, 10);
        sampler.record(make_metrics(1, 5.0, 512));
        thread::sleep(Duration::from_millis(10));
        assert!(sampler.should_sample(), "should sample after interval elapses");
    }

    #[test]
    fn test_buffer_eviction() {
        let mut sampler = Sampler::new(0, 3);
        for i in 0..5u32 {
            sampler.record(make_metrics(i, i as f32, i as u64 * 100));
        }
        assert_eq!(sampler.len(), 3);
        assert_eq!(sampler.samples()[0].pid, 2);
        assert_eq!(sampler.samples()[2].pid, 4);
    }

    #[test]
    fn test_latest_returns_most_recent() {
        let mut sampler = Sampler::new(0, 10);
        sampler.record(make_metrics(10, 1.0, 100));
        sampler.record(make_metrics(20, 2.0, 200));
        let latest = sampler.latest().expect("should have latest");
        assert_eq!(latest.pid, 20);
    }

    #[test]
    fn test_clear_resets_state() {
        let mut sampler = Sampler::new(60, 10);
        sampler.record(make_metrics(1, 1.0, 64));
        sampler.clear();
        assert!(sampler.is_empty());
        assert!(sampler.should_sample(), "after clear, should sample again");
    }

    #[test]
    fn test_interval_accessor() {
        let sampler = Sampler::new(30, 5);
        assert_eq!(sampler.interval(), Duration::from_secs(30));
    }
}
