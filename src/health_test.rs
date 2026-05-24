#[cfg(test)]
mod tests {
    use super::super::health::*;
    use std::time::Duration;

    #[test]
    fn test_new_checker_reports_healthy() {
        let checker = HealthChecker::new();
        let report = checker.report();
        assert!(report.is_healthy());
        assert_eq!(report.checks_passed, 0);
        assert_eq!(report.checks_failed, 0);
    }

    #[test]
    fn test_all_passes_is_healthy() {
        let mut checker = HealthChecker::new();
        checker.record_pass();
        checker.record_pass();
        checker.record_pass();
        let report = checker.report();
        assert!(report.is_healthy());
        assert_eq!(report.checks_passed, 3);
        assert_eq!(report.checks_failed, 0);
    }

    #[test]
    fn test_minority_failures_is_degraded() {
        let mut checker = HealthChecker::new();
        checker.record_pass();
        checker.record_pass();
        checker.record_pass();
        checker.record_fail();
        let report = checker.report();
        assert!(!report.is_healthy());
        assert!(matches!(report.status, HealthStatus::Degraded(_)));
    }

    #[test]
    fn test_majority_failures_is_unhealthy() {
        let mut checker = HealthChecker::new();
        checker.record_pass();
        checker.record_fail();
        checker.record_fail();
        let report = checker.report();
        assert!(!report.is_healthy());
        assert!(matches!(report.status, HealthStatus::Unhealthy(_)));
    }

    #[test]
    fn test_summary_contains_status() {
        let report = HealthReport::new(
            HealthStatus::Healthy,
            Duration::from_secs(10),
            5,
            0,
        );
        let summary = report.summary();
        assert!(summary.contains("healthy"));
        assert!(summary.contains("passed=5"));
        assert!(summary.contains("failed=0"));
    }

    #[test]
    fn test_summary_degraded_message() {
        let report = HealthReport::new(
            HealthStatus::Degraded("disk slow".to_string()),
            Duration::from_secs(60),
            8,
            2,
        );
        let summary = report.summary();
        assert!(summary.contains("degraded"));
        assert!(summary.contains("disk slow"));
    }
}
