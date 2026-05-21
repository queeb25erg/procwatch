use crate::alert::{Alert, AlertLevel};
use crate::throttle::AlertThrottle;
use crate::webhook::WebhookClient;

pub struct Dispatcher {
    client: WebhookClient,
    throttle: AlertThrottle,
}

impl Dispatcher {
    pub fn new(webhook_url: &str, cooldown_secs: u64) -> Self {
        Self {
            client: WebhookClient::new(webhook_url),
            throttle: AlertThrottle::new(cooldown_secs),
        }
    }

    /// Attempt to dispatch an alert, respecting the throttle.
    /// Returns `Ok(true)` if the alert was sent, `Ok(false)` if throttled.
    pub async fn dispatch(&mut self, alert: &Alert) -> Result<bool, String> {
        let key = format!("{}:{:?}", alert.process_name, alert.level);

        if !self.throttle.should_fire(&key) {
            log::debug!("Alert throttled for key: {}", key);
            return Ok(false);
        }

        self.client
            .send(alert)
            .await
            .map_err(|e| format!("Webhook send failed: {}", e))?;

        log::info!(
            "Alert dispatched [{}] for process '{}': {}",
            format!("{:?}", alert.level),
            alert.process_name,
            alert.message
        );
        Ok(true)
    }

    /// Call when a monitored process returns to a healthy state so the
    /// next alert fires immediately without waiting for the cooldown.
    pub fn clear_cooldown(&mut self, process_name: &str, level: &AlertLevel) {
        let key = format!("{}:{:?}", process_name, level);
        self.throttle.reset(&key);
    }

    /// Periodically prune stale throttle entries.
    pub fn evict_stale(&mut self) {
        self.throttle.evict_stale();
    }
}
