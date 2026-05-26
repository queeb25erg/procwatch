use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A sliding time window that holds timestamped values and evicts stale entries.
#[derive(Debug)]
pub struct SlidingWindow<T: Clone> {
    entries: VecDeque<(Instant, T)>,
    window_size: Duration,
}

impl<T: Clone> SlidingWindow<T> {
    pub fn new(window_size: Duration) -> Self {
        Self {
            entries: VecDeque::new(),
            window_size,
        }
    }

    /// Insert a new value with the current timestamp.
    pub fn push(&mut self, value: T) {
        self.evict();
        self.entries.push_back((Instant::now(), value));
    }

    /// Insert a value with an explicit timestamp (useful for testing).
    pub fn push_at(&mut self, at: Instant, value: T) {
        self.evict_at(at);
        self.entries.push_back((at, value));
    }

    /// Remove entries older than the window size relative to now.
    pub fn evict(&mut self) {
        self.evict_at(Instant::now());
    }

    fn evict_at(&mut self, now: Instant) {
        while let Some((ts, _)) = self.entries.front() {
            if now.duration_since(*ts) > self.window_size {
                self.entries.pop_front();
            } else {
                break;
            }
        }
    }

    /// Return a snapshot of all current (non-evicted) values.
    pub fn values(&mut self) -> Vec<T> {
        self.evict();
        self.entries.iter().map(|(_, v)| v.clone()).collect()
    }

    /// Number of entries currently in the window.
    pub fn len(&mut self) -> usize {
        self.evict();
        self.entries.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    pub fn window_size(&self) -> Duration {
        self.window_size
    }
}
