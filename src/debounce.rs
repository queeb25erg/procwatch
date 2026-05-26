use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Debounce tracker that suppresses repeated events within a quiet period.
/// An event is only "fired" if it hasn't been seen within the debounce window.
pub struct Debouncer {
    window: Duration,
    last_seen: HashMap<String, Instant>,
}

impl Debouncer {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            last_seen: HashMap::new(),
        }
    }

    /// Returns `true` if the event should be forwarded (i.e. not debounced).
    /// Returns `false` if the event occurred within the debounce window.
    pub fn should_fire(&mut self, key: &str) -> bool {
        let now = Instant::now();
        if let Some(&last) = self.last_seen.get(key) {
            if now.duration_since(last) < self.window {
                return false;
            }
        }
        self.last_seen.insert(key.to_string(), now);
        true
    }

    /// Explicitly reset the debounce state for a given key.
    pub fn reset(&mut self, key: &str) {
        self.last_seen.remove(key);
    }

    /// Remove all entries whose last-seen timestamp is older than the window,
    /// keeping memory bounded during long-running operation.
    pub fn evict_expired(&mut self) {
        let now = Instant::now();
        self.last_seen
            .retain(|_, last| now.duration_since(*last) < self.window);
    }

    /// Number of keys currently tracked.
    pub fn len(&self) -> usize {
        self.last_seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.last_seen.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
