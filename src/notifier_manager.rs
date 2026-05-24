use std::collections::HashMap;
use crate::notifier::{Notifier, NotifyResult};
use crate::alert::Alert;
use crate::webhook::WebhookClient;
use crate::throttle::Throttle;
use crate::circuit_breaker::CircuitBreaker;

pub struct NotifierManager {
    notifiers: HashMap<String, Notifier>,
}

impl NotifierManager {
    pub fn new() -> Self {
        Self {
            notifiers: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: impl Into<String>, notifier: Notifier) {
        self.notifiers.insert(name.into(), notifier);
    }

    pub fn register_default(
        &mut self,
        name: impl Into<String>,
        url: &str,
        throttle_secs: u64,
        failure_threshold: u32,
    ) {
        let client = WebhookClient::new(url.to_string());
        let throttle = Throttle::new(std::time::Duration::from_secs(throttle_secs));
        let breaker = CircuitBreaker::new(failure_threshold, std::time::Duration::from_secs(60));
        let notifier = Notifier::new(client, throttle, breaker);
        self.notifiers.insert(name.into(), notifier);
    }

    pub fn notify_all(&mut self, alert: &Alert) -> HashMap<String, NotifyResult> {
        self.notifiers
            .iter_mut()
            .map(|(name, notifier)| (name.clone(), notifier.notify(alert)))
            .collect()
    }

    pub fn notify_one(&mut self, name: &str, alert: &Alert) -> Option<NotifyResult> {
        self.notifiers.get_mut(name).map(|n| n.notify(alert))
    }

    pub fn healthy_count(&self) -> usize {
        self.notifiers.values().filter(|n| n.is_healthy()).count()
    }

    pub fn total_count(&self) -> usize {
        self.notifiers.len()
    }
}

impl Default for NotifierManager {
    fn default() -> Self {
        Self::new()
    }
}
