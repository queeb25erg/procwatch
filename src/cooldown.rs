use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks per-process cooldown periods to prevent alert fatigue.
/// Once an alert fires for a process, it won't fire again until the cooldown expires.
pub struct CooldownTracker {
    cooldown_duration: Duration,
    last_alerted: HashMap<String, Instant>,
}

impl CooldownTracker {
    pub fn new(cooldown_secs: u64) -> Self {
        Self {
            cooldown_duration: Duration::from_secs(cooldown_secs),
            last_alerted: HashMap::new(),
        }
    }

    /// Returns true if the process is allowed to trigger an alert (not in cooldown).
    pub fn is_allowed(&self, process_name: &str) -> bool {
        match self.last_alerted.get(process_name) {
            Some(last) => last.elapsed() >= self.cooldown_duration,
            None => true,
        }
    }

    /// Records that an alert was sent for the given process.
    pub fn record_alert(&mut self, process_name: &str) {
        self.last_alerted
            .insert(process_name.to_string(), Instant::now());
    }

    /// Returns remaining cooldown in seconds, or 0 if not in cooldown.
    pub fn remaining_secs(&self, process_name: &str) -> u64 {
        match self.last_alerted.get(process_name) {
            Some(last) => {
                let elapsed = last.elapsed();
                if elapsed >= self.cooldown_duration {
                    0
                } else {
                    (self.cooldown_duration - elapsed).as_secs()
                }
            }
            None => 0,
        }
    }

    /// Clears cooldown state for a specific process.
    pub fn reset(&mut self, process_name: &str) {
        self.last_alerted.remove(process_name);
    }

    /// Clears all cooldown state.
    pub fn reset_all(&mut self) {
        self.last_alerted.clear();
    }
}
