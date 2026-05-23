use std::time::Duration;
use crate::suppression::SuppressionStore;

/// High-level manager that decides whether an alert for a process should
/// be suppressed and records suppression state.
pub struct SuppressionManager {
    store: SuppressionStore,
}

impl SuppressionManager {
    pub fn new(window_secs: u64) -> Self {
        Self {
            store: SuppressionStore::new(Duration::from_secs(window_secs)),
        }
    }

    /// Check whether an alert for `process` + `alert_type` should fire.
    /// Returns `true` if the alert should be sent, `false` if suppressed.
    pub fn should_alert(&mut self, process: &str, alert_type: &str) -> bool {
        let key = format!("{}/{}", process, alert_type);
        if self.store.is_suppressed(&key) {
            false
        } else {
            self.store.suppress(&key);
            true
        }
    }

    /// How many times has this alert been suppressed (not fired).
    pub fn suppressed_count(&self, process: &str, alert_type: &str) -> u32 {
        let key = format!("{}/{}", process, alert_type);
        let total = self.store.suppression_count(&key);
        if total == 0 { 0 } else { total - 1 }
    }

    /// Manually lift suppression, e.g. when a process recovers.
    pub fn lift(&mut self, process: &str, alert_type: &str) {
        let key = format!("{}/{}", process, alert_type);
        self.store.lift(&key);
    }

    /// Evict expired entries to keep memory usage low.
    pub fn evict_expired(&mut self) {
        self.store.evict_expired();
    }
}
