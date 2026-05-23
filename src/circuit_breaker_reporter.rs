use crate::circuit_breaker::{CircuitBreaker, CircuitState};

#[derive(Debug, serde::Serialize)]
pub struct CircuitBreakerReport {
    pub key: String,
    pub state: String,
    pub failure_count: u32,
    pub is_open: bool,
}

impl CircuitBreakerReport {
    pub fn from_breaker(key: &str, breaker: &CircuitBreaker) -> Self {
        let state_str = match breaker.state() {
            CircuitState::Closed => "closed",
            CircuitState::Open => "open",
            CircuitState::HalfOpen => "half_open",
        };
        Self {
            key: key.to_string(),
            state: state_str.to_string(),
            failure_count: breaker.failure_count(),
            is_open: breaker.is_open(),
        }
    }

    pub fn summary_line(&self) -> String {
        format!(
            "[circuit_breaker] key={} state={} failures={} open={}",
            self.key, self.state, self.failure_count, self.is_open
        )
    }
}

pub fn report_open_circuits(open_keys: &[&str]) {
    if open_keys.is_empty() {
        return;
    }
    eprintln!(
        "[circuit_breaker] WARNING: {} circuit(s) are open: {}",
        open_keys.len(),
        open_keys.join(", ")
    );
}
