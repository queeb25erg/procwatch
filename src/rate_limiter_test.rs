#[cfg(test)]
mod tests {
    use super::super::rate_limiter::RateLimiter;
    use std::time::Duration;

    fn make_limiter(max_tokens: u32, refill_secs: u64) -> RateLimiter {
        RateLimiter::new(max_tokens, Duration::from_secs(refill_secs))
    }

    #[test]
    fn test_allows_up_to_max_tokens() {
        let mut limiter = make_limiter(3, 60);
        assert!(limiter.allow("proc_a"));
        assert!(limiter.allow("proc_a"));
        assert!(limiter.allow("proc_a"));
        assert!(!limiter.allow("proc_a"));
    }

    #[test]
    fn test_independent_buckets_per_key() {
        let mut limiter = make_limiter(2, 60);
        assert!(limiter.allow("proc_a"));
        assert!(limiter.allow("proc_a"));
        assert!(!limiter.allow("proc_a"));
        // proc_b should still have tokens
        assert!(limiter.allow("proc_b"));
        assert!(limiter.allow("proc_b"));
        assert!(!limiter.allow("proc_b"));
    }

    #[test]
    fn test_new_key_starts_with_full_tokens() {
        let mut limiter = make_limiter(5, 60);
        for _ in 0..5 {
            assert!(limiter.allow("new_proc"));
        }
        assert!(!limiter.allow("new_proc"));
    }

    #[test]
    fn test_bucket_count_tracks_unique_keys() {
        let mut limiter = make_limiter(3, 60);
        limiter.allow("p1");
        limiter.allow("p2");
        limiter.allow("p3");
        assert_eq!(limiter.bucket_count(), 3);
    }

    #[test]
    fn test_refill_after_interval() {
        let mut limiter = RateLimiter::new(2, Duration::from_millis(10));
        assert!(limiter.allow("proc"));
        assert!(limiter.allow("proc"));
        assert!(!limiter.allow("proc"));
        std::thread::sleep(Duration::from_millis(15));
        assert!(limiter.allow("proc"));
    }

    #[test]
    fn test_cleanup_removes_no_recent_buckets() {
        let mut limiter = make_limiter(3, 60);
        limiter.allow("proc_x");
        assert_eq!(limiter.bucket_count(), 1);
        limiter.cleanup();
        // Not stale yet (threshold is 10 * 60 = 600s)
        assert_eq!(limiter.bucket_count(), 1);
    }
}
