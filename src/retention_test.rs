#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};
    use crate::retention::{RetainedSample, RetentionBuffer, RetentionPolicy};

    fn make_policy(max_age_secs: u64, max_samples: usize) -> RetentionPolicy {
        RetentionPolicy::new(Duration::from_secs(max_age_secs), max_samples)
    }

    #[test]
    fn test_push_and_retrieve() {
        let mut buf = RetentionBuffer::new(make_policy(60, 100));
        buf.push(RetainedSample::new(1.0));
        buf.push(RetainedSample::new(2.0));
        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn test_max_samples_eviction() {
        let mut buf = RetentionBuffer::new(make_policy(3600, 3));
        for i in 0..5 {
            buf.push(RetainedSample::new(i as f64));
        }
        assert_eq!(buf.len(), 3);
        // Oldest samples should be evicted; newest retained
        let values: Vec<f64> = buf.samples().iter().map(|s| s.value).collect();
        assert_eq!(values, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_age_based_eviction() {
        let mut buf = RetentionBuffer::new(make_policy(1, 1000));
        // Insert a sample with an old timestamp (2 seconds ago)
        let old_ts = SystemTime::now() - Duration::from_secs(2);
        buf.push(RetainedSample::with_timestamp(99.0, old_ts));
        // Insert a fresh sample to trigger eviction
        buf.push(RetainedSample::new(42.0));
        assert_eq!(buf.len(), 1);
        assert_eq!(buf.samples().front().unwrap().value, 42.0);
    }

    #[test]
    fn test_empty_buffer() {
        let buf = RetentionBuffer::new(RetentionPolicy::default());
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_default_policy() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.max_age, Duration::from_secs(3600));
        assert_eq!(policy.max_samples, 1000);
    }

    #[test]
    fn test_no_eviction_within_limits() {
        let mut buf = RetentionBuffer::new(make_policy(3600, 10));
        for i in 0..5 {
            buf.push(RetainedSample::new(i as f64));
        }
        assert_eq!(buf.len(), 5);
    }
}
