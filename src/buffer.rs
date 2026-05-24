use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BufferedEvent {
    pub payload: String,
    pub queued_at: Instant,
}

impl BufferedEvent {
    pub fn new(payload: String) -> Self {
        Self {
            payload,
            queued_at: Instant::now(),
        }
    }

    pub fn age(&self) -> Duration {
        self.queued_at.elapsed()
    }
}

#[derive(Debug)]
pub struct EventBuffer {
    queue: VecDeque<BufferedEvent>,
    capacity: usize,
    max_age: Duration,
}

impl EventBuffer {
    pub fn new(capacity: usize, max_age: Duration) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity,
            max_age,
        }
    }

    pub fn push(&mut self, payload: String) -> bool {
        if self.queue.len() >= self.capacity {
            return false;
        }
        self.queue.push_back(BufferedEvent::new(payload));
        true
    }

    pub fn drain_ready(&mut self) -> Vec<BufferedEvent> {
        self.evict_expired();
        self.queue.drain(..).collect()
    }

    pub fn evict_expired(&mut self) {
        self.queue.retain(|e| e.age() <= self.max_age);
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() >= self.capacity
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
