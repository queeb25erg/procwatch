use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A processing pipeline stage result
#[derive(Debug, Clone, PartialEq)]
pub enum StageResult {
    Pass,
    Drop(String),
    Transform(String),
}

/// Represents a single item flowing through the pipeline
#[derive(Debug, Clone)]
pub struct PipelineItem {
    pub pid: u32,
    pub name: String,
    pub value: f64,
    pub label: Option<String>,
    pub timestamp: Instant,
}

impl PipelineItem {
    pub fn new(pid: u32, name: impl Into<String>, value: f64) -> Self {
        Self {
            pid,
            name: name.into(),
            value,
            label: None,
            timestamp: Instant::now(),
        }
    }

    pub fn age(&self) -> Duration {
        self.timestamp.elapsed()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Pipeline processes items through a series of stages
pub struct Pipeline {
    pub name: String,
    queue: VecDeque<PipelineItem>,
    capacity: usize,
    processed: u64,
    dropped: u64,
}

impl Pipeline {
    pub fn new(name: impl Into<String>, capacity: usize) -> Self {
        Self {
            name: name.into(),
            queue: VecDeque::with_capacity(capacity),
            capacity,
            processed: 0,
            dropped: 0,
        }
    }

    pub fn push(&mut self, item: PipelineItem) -> bool {
        if self.queue.len() >= self.capacity {
            self.dropped += 1;
            return false;
        }
        self.queue.push_back(item);
        true
    }

    pub fn pop(&mut self) -> Option<PipelineItem> {
        let item = self.queue.pop_front();
        if item.is_some() {
            self.processed += 1;
        }
        item
    }

    pub fn drain_expired(&mut self, max_age: Duration) -> Vec<PipelineItem> {
        let mut expired = Vec::new();
        self.queue.retain(|item| {
            if item.age() > max_age {
                expired.push(item.clone());
                false
            } else {
                true
            }
        });
        self.dropped += expired.len() as u64;
        expired
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn stats(&self) -> (u64, u64) {
        (self.processed, self.dropped)
    }
}
