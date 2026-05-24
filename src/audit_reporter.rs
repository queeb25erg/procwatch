use crate::audit::{AuditEvent, AuditEventKind, AuditLog};

pub struct AuditReporter<'a> {
    log: &'a AuditLog,
}

impl<'a> AuditReporter<'a> {
    pub fn new(log: &'a AuditLog) -> Self {
        Self { log }
    }

    pub fn summary(&self) -> AuditSummary {
        let total = self.log.len();
        let alerts_fired = self.log.filter_by_kind(&AuditEventKind::AlertFired).len();
        let alerts_suppressed = self
            .log
            .filter_by_kind(&AuditEventKind::AlertSuppressed)
            .len();
        let webhooks_sent = self.log.filter_by_kind(&AuditEventKind::WebhookSent).len();
        let webhooks_failed = self
            .log
            .filter_by_kind(&AuditEventKind::WebhookFailed)
            .len();
        let circuit_breaker_trips = self
            .log
            .filter_by_kind(&AuditEventKind::CircuitBreakerOpened)
            .len();

        AuditSummary {
            total,
            alerts_fired,
            alerts_suppressed,
            webhooks_sent,
            webhooks_failed,
            circuit_breaker_trips,
        }
    }

    pub fn recent_events(&self, n: usize) -> Vec<&AuditEvent> {
        self.log.events().iter().rev().take(n).collect()
    }

    pub fn format_event(event: &AuditEvent) -> String {
        let proc_label = event
            .process
            .as_deref()
            .map(|p| format!(" [{}]", p))
            .unwrap_or_default();
        format!(
            "[{}]{} {:?}: {}",
            event.timestamp, proc_label, event.kind, event.message
        )
    }
}

#[derive(Debug, Clone)]
pub struct AuditSummary {
    pub total: usize,
    pub alerts_fired: usize,
    pub alerts_suppressed: usize,
    pub webhooks_sent: usize,
    pub webhooks_failed: usize,
    pub circuit_breaker_trips: usize,
}
