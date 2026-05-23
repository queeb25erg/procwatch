use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

/// Policy controlling how long metric samples are retained in memory.
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    /// Maximum age of a sample before it is evicted.
    pub max_age: Duration,
    /// Maximum number of samples to keep regardless of age.
    pub max_samples: usize,
}

impl RetentionPolicy {
    pub fn new(max_age: Duration, max_samples: usize) -> Self {
        Self { max_age, max_samples }
    }
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_age: Duration::from_secs(3600), // 1 hour
            max_samples: 1000,
        }
    }
}

/// A timestamped sample value stored in the retention buffer.
#[derive(Debug, Clone)]
pub struct RetainedSample {
    pub timestamp: SystemTime,
    pub value: f64,
}

impl RetainedSample {
    pub fn new(value: f64) -> Self {
        Self {
            timestamp: SystemTime::now(),
            value,
        }
    }

    pub fn with_timestamp(value: f64, timestamp: SystemTime) -> Self {
        Self { timestamp, value }
    }
}

/// Retention buffer that enforces a RetentionPolicy over a sliding window of samples.
#[derive(Debug, Default)]
pub struct RetentionBuffer {
    samples: VecDeque<RetainedSample>,
    policy: RetentionPolicy,
}

impl RetentionBuffer {
    pub fn new(policy: RetentionPolicy) -> Self {
        Self {
            samples: VecDeque::new(),
            policy,
        }
    }

    /// Push a new sample and evict stale/excess entries.
    pub fn push(&mut self, sample: RetainedSample) {
        self.samples.push_back(sample);
        self.evict();
    }

    /// Return a slice of currently retained samples.
    pub fn samples(&self) -> &VecDeque<RetainedSample> {
        &self.samples
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    fn evict(&mut self) {
        let now = SystemTime::now();
        // Evict by age (front of deque is oldest)
        while let Some(front) = self.samples.front() {
            match now.duration_since(front.timestamp) {
                Ok(age) if age > self.policy.max_age => {
                    self.samples.pop_front();
                }
                _ => break,
            }
        }
        // Evict by count
        while self.samples.len() > self.policy.max_samples {
            self.samples.pop_front();
        }
    }
}
