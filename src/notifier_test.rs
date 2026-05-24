#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::notifier::{Notifier, NotifyResult};
    use crate::notifier_manager::NotifierManager;
    use crate::alert::{Alert, AlertKind};
    use crate::webhook::WebhookClient;
    use crate::throttle::Throttle;
    use crate::circuit_breaker::CircuitBreaker;

    fn make_alert(process: &str) -> Alert {
        Alert {
            process_name: process.to_string(),
            kind: AlertKind::HighCpu,
            value: 95.0,
            threshold: 90.0,
            message: format!("{} exceeded CPU threshold", process),
        }
    }

    fn make_notifier(url: &str) -> Notifier {
        let client = WebhookClient::new(url.to_string());
        let throttle = Throttle::new(Duration::from_secs(60));
        let breaker = CircuitBreaker::new(3, Duration::from_secs(30));
        Notifier::new(client, throttle, breaker)
    }

    #[test]
    fn test_notifier_throttles_duplicate_alerts() {
        let mut notifier = make_notifier("http://localhost:9999/hook");
        let alert = make_alert("nginx");

        // First call may fail (no server) but records throttle
        let _ = notifier.notify(&alert);
        // Second call within throttle window should be throttled
        let result = notifier.notify(&alert);
        assert_eq!(result, NotifyResult::Throttled);
    }

    #[test]
    fn test_notifier_opens_circuit_after_failures() {
        let mut notifier = make_notifier("http://localhost:9999/hook");
        let alert = make_alert("myapp");

        // Exhaust failures to open circuit
        for _ in 0..5 {
            let _ = notifier.notify(&alert);
        }

        assert!(!notifier.is_healthy());
    }

    #[test]
    fn test_notifier_records_last_attempt() {
        let mut notifier = make_notifier("http://localhost:9999/hook");
        assert!(notifier.last_attempt().is_none());
        let alert = make_alert("svc");
        let _ = notifier.notify(&alert);
        assert!(notifier.last_attempt().is_some());
    }

    #[test]
    fn test_manager_healthy_count() {
        let mut manager = NotifierManager::new();
        manager.register("a", make_notifier("http://localhost:9991/hook"));
        manager.register("b", make_notifier("http://localhost:9992/hook"));
        assert_eq!(manager.healthy_count(), 2);
        assert_eq!(manager.total_count(), 2);
    }

    #[test]
    fn test_manager_notify_all_returns_results_for_all() {
        let mut manager = NotifierManager::new();
        manager.register("primary", make_notifier("http://localhost:9993/hook"));
        manager.register("secondary", make_notifier("http://localhost:9994/hook"));
        let alert = make_alert("daemon");
        let results = manager.notify_all(&alert);
        assert_eq!(results.len(), 2);
        assert!(results.contains_key("primary"));
        assert!(results.contains_key("secondary"));
    }

    #[test]
    fn test_manager_notify_one_unknown_returns_none() {
        let mut manager = NotifierManager::new();
        let alert = make_alert("proc");
        let result = manager.notify_one("missing", &alert);
        assert!(result.is_none());
    }
}
