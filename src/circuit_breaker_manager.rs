use std::collections::HashMap;
use std::time::Duration;
use crate::circuit_breaker::{CircuitBreaker, CircuitState};

pub struct CircuitBreakerManager {
    breakers: HashMap<String, CircuitBreaker>,
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
}

impl CircuitBreakerManager {
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout_secs: u64) -> Self {
        Self {
            breakers: HashMap::new(),
            failure_threshold,
            success_threshold,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    fn get_or_create(&mut self, key: &str) -> &mut CircuitBreaker {
        self.breakers.entry(key.to_string()).or_insert_with(|| {
            CircuitBreaker::new(
                self.failure_threshold,
                self.success_threshold,
                self.timeout,
            )
        })
    }

    pub fn allow_request(&mut self, key: &str) -> bool {
        self.get_or_create(key).allow_request()
    }

    pub fn record_success(&mut self, key: &str) {
        self.get_or_create(key).record_success();
    }

    pub fn record_failure(&mut self, key: &str) {
        self.get_or_create(key).record_failure();
    }

    pub fn state(&mut self, key: &str) -> CircuitState {
        self.get_or_create(key).state().clone()
    }

    pub fn open_circuits(&self) -> Vec<&str> {
        self.breakers
            .iter()
            .filter(|(_, cb)| cb.is_open())
            .map(|(k, _)| k.as_str())
            .collect()
    }

    pub fn reset(&mut self, key: &str) {
        self.breakers.remove(key);
    }
}
