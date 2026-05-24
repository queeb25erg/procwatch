use std::collections::HashMap;
use std::time::Duration;
use crate::buffer::EventBuffer;

pub struct BufferManager {
    buffers: HashMap<String, EventBuffer>,
    default_capacity: usize,
    default_max_age: Duration,
}

impl BufferManager {
    pub fn new(default_capacity: usize, default_max_age: Duration) -> Self {
        Self {
            buffers: HashMap::new(),
            default_capacity,
            default_max_age,
        }
    }

    pub fn get_or_create(&mut self, key: &str) -> &mut EventBuffer {
        let cap = self.default_capacity;
        let age = self.default_max_age;
        self.buffers
            .entry(key.to_string())
            .or_insert_with(|| EventBuffer::new(cap, age))
    }

    pub fn push(&mut self, key: &str, payload: String) -> bool {
        self.get_or_create(key).push(payload)
    }

    pub fn drain(&mut self, key: &str) -> Vec<String> {
        if let Some(buf) = self.buffers.get_mut(key) {
            buf.drain_ready().into_iter().map(|e| e.payload).collect()
        } else {
            vec![]
        }
    }

    pub fn drain_all(&mut self) -> HashMap<String, Vec<String>> {
        self.buffers
            .iter_mut()
            .map(|(k, buf)| {
                let events = buf.drain_ready().into_iter().map(|e| e.payload).collect();
                (k.clone(), events)
            })
            .filter(|(_, v)| !v.is_empty())
            .collect()
    }

    pub fn evict_all_expired(&mut self) {
        for buf in self.buffers.values_mut() {
            buf.evict_expired();
        }
    }

    pub fn total_pending(&self) -> usize {
        self.buffers.values().map(|b| b.len()).sum()
    }

    pub fn remove(&mut self, key: &str) {
        self.buffers.remove(key);
    }
}
