#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::retry::{RetryPolicy, RetryExecutor, RetryOutcome};
    use crate::retry_manager::RetryManager;
    use crate::retry_reporter::{RetryReporter, RetryEvent};

    #[test]
    fn test_delay_for_increases_exponentially() {
        let policy = RetryPolicy::new(
            5,
            Duration::from_millis(100),
            Duration::from_secs(10),
            2.0,
        );
        assert_eq!(policy.delay_for(0), Duration::from_millis(100));
        assert_eq!(policy.delay_for(1), Duration::from_millis(200));
        assert_eq!(policy.delay_for(2), Duration::from_millis(400));
    }

    #[test]
    fn test_delay_capped_at_max() {
        let policy = RetryPolicy::new(
            10,
            Duration::from_millis(500),
            Duration::from_secs(1),
            4.0,
        );
        assert!(policy.delay_for(5) <= Duration::from_secs(1));
    }

    #[test]
    fn test_executor_succeeds_on_first_try() {
        let policy = RetryPolicy::default();
        let executor = RetryExecutor::new(policy);
        let result: Result<RetryOutcome, &str> = executor.execute(|_attempt| Ok(()));
        assert_eq!(result.unwrap(), RetryOutcome::Succeeded { attempts: 1 });
    }

    #[test]
    fn test_executor_retries_and_succeeds() {
        let policy = RetryPolicy::default();
        let executor = RetryExecutor::new(policy);
        let mut calls = 0;
        let result: Result<RetryOutcome, &str> = executor.execute(|_| {
            calls += 1;
            if calls < 3 { Err("fail") } else { Ok(()) }
        });
        assert_eq!(result.unwrap(), RetryOutcome::Succeeded { attempts: 3 });
    }

    #[test]
    fn test_executor_exhausts_retries() {
        let policy = RetryPolicy::new(2, Duration::from_millis(1), Duration::from_millis(10), 1.0);
        let executor = RetryExecutor::new(policy);
        let result: Result<RetryOutcome, &str> = executor.execute(|_| Err("always fail"));
        assert!(result.is_err());
    }

    #[test]
    fn test_retry_manager_tracks_state() {
        let policy = RetryPolicy::new(3, Duration::from_millis(10), Duration::from_millis(100), 2.0);
        let mut manager = RetryManager::new(policy);
        manager.record_failure("webhook");
        manager.record_failure("webhook");
        let outcome = manager.record_failure("webhook");
        assert!(matches!(outcome, RetryOutcome::Exhausted { .. }));
    }

    #[test]
    fn test_retry_manager_reset_on_success() {
        let policy = RetryPolicy::default();
        let mut manager = RetryManager::new(policy);
        manager.record_failure("svc");
        manager.record_success("svc");
        let state = manager.get_or_create("svc");
        assert_eq!(state.attempt, 0);
    }

    #[test]
    fn test_reporter_counts() {
        let mut reporter = RetryReporter::new();
        reporter.record(RetryEvent::new("a", RetryOutcome::Succeeded { attempts: 1 }, "ok"));
        reporter.record(RetryEvent::new("b", RetryOutcome::Exhausted { attempts: 3 }, "fail"));
        assert_eq!(reporter.success_count(), 1);
        assert_eq!(reporter.exhausted_count(), 1);
    }
}
