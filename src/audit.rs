use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum AuditEventKind {
    AlertFired,
    AlertSuppressed,
    WebhookSent,
    WebhookFailed,
    ConfigReloaded,
    ProcessAdded,
    ProcessRemoved,
    ThresholdBreached,
    CircuitBreakerOpened,
    CircuitBreakerClosed,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub kind: AuditEventKind,
    pub process: Option<String>,
    pub message: String,
}

impl AuditEvent {
    pub fn new(kind: AuditEventKind, process: Option<String>, message: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            timestamp,
            kind,
            process,
            message: message.into(),
        }
    }
}

pub struct AuditLog {
    events: VecDeque<AuditEvent>,
    capacity: usize,
}

impl AuditLog {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn record(&mut self, event: AuditEvent) {
        if self.events.len() >= self.capacity {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn events(&self) -> &VecDeque<AuditEvent> {
        &self.events
    }

    pub fn filter_by_kind(&self, kind: &AuditEventKind) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| &e.kind == kind).collect()
    }

    pub fn filter_by_process(&self, process: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.process.as_deref() == Some(process))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
