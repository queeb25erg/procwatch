use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A single historical sample of a metric value.
#[derive(Debug, Clone)]
pub struct Sample {
    pub value: f64,
    pub recorded_at: Instant,
}

/// Keeps a rolling window of metric samples for a single process/metric key.
#[derive(Debug)]
pub struct MetricHistory {
    window: Duration,
    samples: VecDeque<Sample>,
}

impl MetricHistory {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            samples: VecDeque::new(),
        }
    }

    /// Record a new sample, evicting entries older than the window.
    pub fn push(&mut self, value: f64) {
        let now = Instant::now();
        self.samples.push_back(Sample { value, recorded_at: now });
        self.evict(now);
    }

    fn evict(&mut self, now: Instant) {
        while let Some(front) = self.samples.front() {
            if now.duration_since(front.recorded_at) > self.window {
                self.samples.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn average(&self) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        let sum: f64 = self.samples.iter().map(|s| s.value).sum();
        Some(sum / self.samples.len() as f64)
    }

    pub fn max(&self) -> Option<f64> {
        self.samples.iter().map(|s| s.value).reduce(f64::max)
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}
