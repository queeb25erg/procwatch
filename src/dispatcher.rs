use crate::alert::{evaluate_alerts, Alert};
use crate::config::Config;
use crate::metrics::ProcessMetrics;
use crate::webhook::WebhookSender;
use std::collections::HashMap;
use tracing::{error, info, warn};

pub struct Dispatcher {
    sender: WebhookSender,
    config: Config,
    /// Track consecutive alert counts per pid to avoid spam
    alert_counts: HashMap<u32, u32>,
}

impl Dispatcher {
    pub fn new(config: Config) -> Self {
        let sender = WebhookSender::new(config.webhook.url.clone());
        Self {
            sender,
            config,
            alert_counts: HashMap::new(),
        }
    }

    pub async fn process(&mut self, metrics: &[ProcessMetrics]) {
        let mut pending: Vec<Alert> = Vec::new();

        for m in metrics {
            let alerts = evaluate_alerts(m, &self.config.alerts);
            if alerts.is_empty() {
                self.alert_counts.remove(&m.pid);
                continue;
            }
            let count = self.alert_counts.entry(m.pid).or_insert(0);
            *count += 1;
            let repeat_interval = self.config.alerts.repeat_interval_cycles.unwrap_or(5);
            if *count == 1 || *count % repeat_interval == 0 {
                for a in &alerts {
                    warn!(
                        pid = a.pid,
                        process = %a.process_name,
                        value = a.value,
                        threshold = a.threshold,
                        "Alert triggered: {:?}",
                        a.alert_type
                    );
                }
                pending.extend(alerts);
            }
        }

        if !pending.is_empty() {
            info!(count = pending.len(), "Sending alerts via webhook");
            if let Err(e) = self.sender.send(&pending).await {
                error!("Failed to send webhook: {}", e);
            }
        }
    }
}
