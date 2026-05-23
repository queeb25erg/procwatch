#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::circuit_breaker::{CircuitBreaker, CircuitState};
    use crate::circuit_breaker_manager::CircuitBreakerManager;

    fn make_breaker() -> CircuitBreaker {
        CircuitBreaker::new(3, 2, Duration::from_secs(60))
    }

    #[test]
    fn test_initial_state_is_closed() {
        let cb = make_breaker();
        assert_eq!(*cb.state(), CircuitState::Closed);
        assert!(!cb.is_open());
    }

    #[test]
    fn test_opens_after_threshold_failures() {
        let mut cb = make_breaker();
        cb.record_failure();
        cb.record_failure();
        assert_eq!(*cb.state(), CircuitState::Closed);
        cb.record_failure();
        assert_eq!(*cb.state(), CircuitState::Open);
        assert!(cb.is_open());
    }

    #[test]
    fn test_open_blocks_requests() {
        let mut cb = make_breaker();
        for _ in 0..3 {
            cb.record_failure();
        }
        assert!(!cb.allow_request());
    }

    #[test]
    fn test_success_resets_failure_count() {
        let mut cb = make_breaker();
        cb.record_failure();
        cb.record_failure();
        cb.record_success();
        assert_eq!(cb.failure_count(), 0);
        assert_eq!(*cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_half_open_failure_reopens() {
        let mut cb = CircuitBreaker::new(1, 2, Duration::from_millis(1));
        cb.record_failure();
        assert!(cb.is_open());
        std::thread::sleep(Duration::from_millis(5));
        assert!(cb.allow_request());
        assert_eq!(*cb.state(), CircuitState::HalfOpen);
        cb.record_failure();
        assert_eq!(*cb.state(), CircuitState::Open);
    }

    #[test]
    fn test_half_open_success_closes_breaker() {
        let mut cb = CircuitBreaker::new(1, 1, Duration::from_millis(1));
        cb.record_failure();
        assert!(cb.is_open());
        std::thread::sleep(Duration::from_millis(5));
        // Transition to HalfOpen by allowing a probe request
        assert!(cb.allow_request());
        assert_eq!(*cb.state(), CircuitState::HalfOpen);
        // A single success should close the breaker (success_threshold = 1)
        cb.record_success();
        assert_eq!(*cb.state(), CircuitState::Closed);
        assert!(!cb.is_open());
    }

    #[test]
    fn test_manager_tracks_multiple_keys() {
        let mut mgr = CircuitBreakerManager::new(2, 1, 60);
        mgr.record_failure("webhook_a");
        mgr.record_failure("webhook_a");
        mgr.record_failure("webhook_b");
        assert_eq!(mgr.state("webhook_a"), CircuitState::Open);
        assert_eq!(mgr.state("webhook_b"), CircuitState::Closed);
        let open = mgr.open_circuits();
        assert_eq!(open.len(), 1);
        assert!(open.contains(&"webhook_a"));
    }

    #[test]
    fn test_manager_reset_removes_breaker() {
        let mut mgr = CircuitBreakerManager::new(1, 1, 60);
        mgr.record_failure("svc");
        assert_eq!(mgr.state("svc"), CircuitState::Open);
        mgr.reset("svc");
        assert_eq!(mgr.state("svc"), CircuitState::Closed);
    }
}
