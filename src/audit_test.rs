#[cfg(test)]
mod tests {
    use crate::audit::{AuditEvent, AuditEventKind, AuditLog};
    use crate::audit_reporter::AuditReporter;

    fn make_event(kind: AuditEventKind, process: Option<&str>, msg: &str) -> AuditEvent {
        AuditEvent::new(kind, process.map(str::to_string), msg)
    }

    #[test]
    fn test_record_and_len() {
        let mut log = AuditLog::new(10);
        assert!(log.is_empty());
        log.record(make_event(AuditEventKind::AlertFired, Some("nginx"), "cpu high"));
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn test_capacity_eviction() {
        let mut log = AuditLog::new(3);
        for i in 0..5 {
            log.record(make_event(
                AuditEventKind::AlertFired,
                None,
                &format!("event {}", i),
            ));
        }
        assert_eq!(log.len(), 3);
        let msgs: Vec<_> = log.events().iter().map(|e| e.message.clone()).collect();
        assert_eq!(msgs, vec!["event 2", "event 3", "event 4"]);
    }

    #[test]
    fn test_filter_by_kind() {
        let mut log = AuditLog::new(20);
        log.record(make_event(AuditEventKind::AlertFired, None, "a"));
        log.record(make_event(AuditEventKind::WebhookSent, None, "b"));
        log.record(make_event(AuditEventKind::AlertFired, None, "c"));
        let fired = log.filter_by_kind(&AuditEventKind::AlertFired);
        assert_eq!(fired.len(), 2);
    }

    #[test]
    fn test_filter_by_process() {
        let mut log = AuditLog::new(20);
        log.record(make_event(AuditEventKind::AlertFired, Some("nginx"), "x"));
        log.record(make_event(AuditEventKind::AlertFired, Some("redis"), "y"));
        log.record(make_event(AuditEventKind::WebhookSent, Some("nginx"), "z"));
        let nginx = log.filter_by_process("nginx");
        assert_eq!(nginx.len(), 2);
    }

    #[test]
    fn test_reporter_summary() {
        let mut log = AuditLog::new(50);
        log.record(make_event(AuditEventKind::AlertFired, None, "1"));
        log.record(make_event(AuditEventKind::AlertFired, None, "2"));
        log.record(make_event(AuditEventKind::AlertSuppressed, None, "3"));
        log.record(make_event(AuditEventKind::WebhookSent, None, "4"));
        log.record(make_event(AuditEventKind::WebhookFailed, None, "5"));
        log.record(make_event(AuditEventKind::CircuitBreakerOpened, None, "6"));
        let reporter = AuditReporter::new(&log);
        let summary = reporter.summary();
        assert_eq!(summary.total, 6);
        assert_eq!(summary.alerts_fired, 2);
        assert_eq!(summary.alerts_suppressed, 1);
        assert_eq!(summary.webhooks_sent, 1);
        assert_eq!(summary.webhooks_failed, 1);
        assert_eq!(summary.circuit_breaker_trips, 1);
    }

    #[test]
    fn test_reporter_recent_events() {
        let mut log = AuditLog::new(20);
        for i in 0..5 {
            log.record(make_event(AuditEventKind::AlertFired, None, &format!("e{}", i)));
        }
        let reporter = AuditReporter::new(&log);
        let recent = reporter.recent_events(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].message, "e4");
    }

    #[test]
    fn test_clear() {
        let mut log = AuditLog::new(10);
        log.record(make_event(AuditEventKind::AlertFired, None, "x"));
        log.clear();
        assert!(log.is_empty());
    }
}
