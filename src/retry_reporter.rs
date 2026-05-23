use crate::retry::RetryOutcome;

#[derive(Debug, Clone)]
pub struct RetryEvent {
    pub key: String,
    pub outcome: RetryOutcome,
    pub message: String,
}

impl RetryEvent {
    pub fn new(key: impl Into<String>, outcome: RetryOutcome, message: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            outcome,
            message: message.into(),
        }
    }
}

pub struct RetryReporter {
    events: Vec<RetryEvent>,
}

impl RetryReporter {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn record(&mut self, event: RetryEvent) {
        eprintln!(
            "[retry] key={} outcome={:?} msg={}",
            event.key, event.outcome, event.message
        );
        self.events.push(event);
    }

    pub fn exhausted_count(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e.outcome, RetryOutcome::Exhausted { .. }))
            .count()
    }

    pub fn success_count(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e.outcome, RetryOutcome::Succeeded { .. }))
            .count()
    }

    pub fn drain(&mut self) -> Vec<RetryEvent> {
        std::mem::take(&mut self.events)
    }
}

impl Default for RetryReporter {
    fn default() -> Self {
        Self::new()
    }
}
