use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks alert firing times to prevent alert storms.
pub struct AlertThrottle {
    cooldowns: HashMap<String, Instant>,
    cooldown_duration: Duration,
}

impl AlertThrottle {
    pub fn new(cooldown_secs: u64) -> Self {
        Self {
            cooldowns: HashMap::new(),
            cooldown_duration: Duration::from_secs(cooldown_secs),
        }
    }

    /// Returns `true` if the alert for `key` should be fired (not throttled).
    pub fn should_fire(&mut self, key: &str) -> bool {
        let now = Instant::now();
        if let Some(&last_fired) = self.cooldowns.get(key) {
            if now.duration_since(last_fired) < self.cooldown_duration {
                return false;
            }
        }
        self.cooldowns.insert(key.to_string(), now);
        true
    }

    /// Resets the cooldown entry for `key`, e.g. when a process recovers.
    pub fn reset(&mut self, key: &str) {
        self.cooldowns.remove(key);
    }

    /// Removes stale entries older than twice the cooldown window to avoid
    /// unbounded memory growth.
    pub fn evict_stale(&mut self) {
        let cutoff = self.cooldown_duration * 2;
        let now = Instant::now();
        self.cooldowns
            .retain(|_, last_fired| now.duration_since(*last_fired) < cutoff);
    }
}
