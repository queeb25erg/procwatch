use std::time::{Duration, Instant};
use crate::webhook::WebhookClient;
use crate::alert::Alert;
use crate::throttle::Throttle;
use crate::circuit_breaker::CircuitBreaker;

#[derive(Debug, Clone, PartialEq)]
pub enum NotifyResult {
    Sent,
    Throttled,
    CircuitOpen,
    Failed(String),
}

pub struct Notifier {
    client: WebhookClient,
    throttle: Throttle,
    breaker: CircuitBreaker,
    last_attempt: Option<Instant>,
    timeout: Duration,
}

impl Notifier {
    pub fn new(client: WebhookClient, throttle: Throttle, breaker: CircuitBreaker) -> Self {
        Self {
            client,
            throttle,
            breaker,
            last_attempt: None,
            timeout: Duration::from_secs(10),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn notify(&mut self, alert: &Alert) -> NotifyResult {
        if !self.breaker.allow_request() {
            return NotifyResult::CircuitOpen;
        }

        if !self.throttle.allow(&alert.process_name) {
            return NotifyResult::Throttled;
        }

        self.last_attempt = Some(Instant::now());

        match self.client.send(alert) {
            Ok(_) => {
                self.breaker.record_success();
                NotifyResult::Sent
            }
            Err(e) => {
                self.breaker.record_failure();
                NotifyResult::Failed(e.to_string())
            }
        }
    }

    pub fn last_attempt(&self) -> Option<Instant> {
        self.last_attempt
    }

    pub fn is_healthy(&self) -> bool {
        self.breaker.is_closed()
    }
}
