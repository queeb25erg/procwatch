use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Drain policy for flushing buffered events on shutdown or overflow.
#[derive(Debug, Clone, PartialEq)]
pub enum DrainPolicy {
    /// Flush all pending items regardless of time.
    Immediate,
    /// Flush items up to a maximum wait duration.
    Timeout(Duration),
    /// Drop all pending items.
    Drop,
}

#[derive(Debug)]
pub struct DrainResult {
    pub flushed: usize,
    pub dropped: usize,
    pub elapsed: Duration,
}

#[derive(Debug)]
pub struct Drain {
    policy: DrainPolicy,
    queue: VecDeque<String>,
}

impl Drain {
    pub fn new(policy: DrainPolicy) -> Self {
        Self {
            policy,
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, item: String) {
        self.queue.push_back(item);
    }

    pub fn pending(&self) -> usize {
        self.queue.len()
    }

    /// Execute the drain according to the configured policy.
    /// Returns a summary of what was flushed vs dropped.
    pub fn execute<F>(&mut self, mut flush_fn: F) -> DrainResult
    where
        F: FnMut(&str) -> bool,
    {
        let start = Instant::now();
        let total = self.queue.len();

        match &self.policy {
            DrainPolicy::Drop => {
                self.queue.clear();
                DrainResult { flushed: 0, dropped: total, elapsed: start.elapsed() }
            }
            DrainPolicy::Immediate => {
                let mut flushed = 0;
                while let Some(item) = self.queue.pop_front() {
                    if flush_fn(&item) {
                        flushed += 1;
                    }
                }
                DrainResult { flushed, dropped: total - flushed, elapsed: start.elapsed() }
            }
            DrainPolicy::Timeout(limit) => {
                let limit = *limit;
                let mut flushed = 0;
                while let Some(item) = self.queue.front() {
                    if start.elapsed() >= limit {
                        break;
                    }
                    let item = item.clone();
                    self.queue.pop_front();
                    if flush_fn(&item) {
                        flushed += 1;
                    }
                }
                let dropped = self.queue.len();
                self.queue.clear();
                DrainResult { flushed, dropped, elapsed: start.elapsed() }
            }
        }
    }
}
