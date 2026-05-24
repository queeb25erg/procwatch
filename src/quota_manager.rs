use crate::quota::QuotaTracker;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct QuotaManager {
    tracker: Arc<Mutex<QuotaTracker>>,
    default_window: Duration,
}

impl QuotaManager {
    pub fn new(default_window: Duration) -> Self {
        Self {
            tracker: Arc::new(Mutex::new(QuotaTracker::new())),
            default_window,
        }
    }

    pub fn register_process(&self, pid: u32, cpu_limit: f64, mem_limit_mb: f64) {
        let mut tracker = self.tracker.lock().unwrap();
        let key_cpu = format!("cpu:{}", pid);
        let key_mem = format!("mem:{}", pid);
        tracker.register(&key_cpu, cpu_limit, self.default_window);
        tracker.register(&key_mem, mem_limit_mb, self.default_window);
    }

    pub fn check_cpu(&self, pid: u32, usage: f64) -> bool {
        let mut tracker = self.tracker.lock().unwrap();
        let key = format!("cpu:{}", pid);
        tracker.consume(&key, usage)
    }

    pub fn check_memory(&self, pid: u32, usage_mb: f64) -> bool {
        let mut tracker = self.tracker.lock().unwrap();
        let key = format!("mem:{}", pid);
        tracker.consume(&key, usage_mb)
    }

    pub fn remaining_cpu(&self, pid: u32) -> Option<f64> {
        let mut tracker = self.tracker.lock().unwrap();
        let key = format!("cpu:{}", pid);
        tracker.remaining(&key)
    }

    pub fn remaining_memory(&self, pid: u32) -> Option<f64> {
        let mut tracker = self.tracker.lock().unwrap();
        let key = format!("mem:{}", pid);
        tracker.remaining(&key)
    }

    pub fn deregister_process(&self, pid: u32) {
        let mut tracker = self.tracker.lock().unwrap();
        tracker.remove(&format!("cpu:{}", pid));
        tracker.remove(&format!("mem:{}", pid));
    }
}
