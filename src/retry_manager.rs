use std::collections::HashMap;
use std::time::Instant;
use crate::retry::{RetryPolicy, RetryOutcome};

#[derive(Debug)]
pub struct RetryState {
    pub attempt: u32,
    pub last_attempt: Instant,
    pub policy: RetryPolicy,
}

impl RetryState {
    pub fn new(policy: RetryPolicy) -> Self {
        Self {
            attempt: 0,
            last_attempt: Instant::now(),
            policy,
        }
    }

    pub fn next_delay(&self) -> std::time::Duration {
        self.policy.delay_for(self.attempt)
    }

    pub fn can_retry(&self) -> bool {
        self.policy.should_retry(self.attempt)
    }

    pub fn record_attempt(&mut self) {
        self.attempt += 1;
        self.last_attempt = Instant::now();
    }

    pub fn reset(&mut self) {
        self.attempt = 0;
        self.last_attempt = Instant::now();
    }
}

pub struct RetryManager {
    states: HashMap<String, RetryState>,
    default_policy: RetryPolicy,
}

impl RetryManager {
    pub fn new(default_policy: RetryPolicy) -> Self {
        Self {
            states: HashMap::new(),
            default_policy,
        }
    }

    pub fn get_or_create(&mut self, key: &str) -> &mut RetryState {
        let policy = self.default_policy.clone();
        self.states
            .entry(key.to_string())
            .or_insert_with(|| RetryState::new(policy))
    }

    pub fn record_success(&mut self, key: &str) {
        if let Some(state) = self.states.get_mut(key) {
            state.reset();
        }
    }

    pub fn record_failure(&mut self, key: &str) -> RetryOutcome {
        let state = self.get_or_create(key);
        state.record_attempt();
        if state.can_retry() {
            RetryOutcome::Succeeded { attempts: state.attempt }
        } else {
            RetryOutcome::Exhausted { attempts: state.attempt }
        }
    }

    pub fn remove(&mut self, key: &str) {
        self.states.remove(key);
    }
}
