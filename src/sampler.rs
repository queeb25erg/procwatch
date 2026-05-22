use std::time::{Duration, Instant};
use crate::metrics::ProcessMetrics;

/// Collects periodic samples of process metrics at a fixed interval.
pub struct Sampler {
    interval: Duration,
    last_sample: Option<Instant>,
    buffer: Vec<ProcessMetrics>,
    max_buffer: usize,
}

impl Sampler {
    pub fn new(interval_secs: u64, max_buffer: usize) -> Self {
        Sampler {
            interval: Duration::from_secs(interval_secs),
            last_sample: None,
            buffer: Vec::with_capacity(max_buffer),
            max_buffer,
        }
    }

    /// Returns true if enough time has elapsed to take a new sample.
    pub fn should_sample(&self) -> bool {
        match self.last_sample {
            None => true,
            Some(last) => last.elapsed() >= self.interval,
        }
    }

    /// Records a new sample, evicting the oldest if the buffer is full.
    pub fn record(&mut self, metrics: ProcessMetrics) {
        if self.buffer.len() >= self.max_buffer {
            self.buffer.remove(0);
        }
        self.buffer.push(metrics);
        self.last_sample = Some(Instant::now());
    }

    /// Returns a slice of all buffered samples.
    pub fn samples(&self) -> &[ProcessMetrics] {
        &self.buffer
    }

    /// Returns the most recent sample, if any.
    pub fn latest(&self) -> Option<&ProcessMetrics> {
        self.buffer.last()
    }

    /// Clears all buffered samples.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.last_sample = None;
    }

    /// Returns the configured sampling interval.
    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// Returns the number of samples currently buffered.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
