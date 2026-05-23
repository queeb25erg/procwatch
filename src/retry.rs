use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

impl RetryPolicy {
    pub fn new(max_attempts: u32, initial_delay: Duration, max_delay: Duration, multiplier: f64) -> Self {
        Self { max_attempts, initial_delay, max_delay, multiplier }
    }

    pub fn delay_for(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return self.initial_delay;
        }
        let factor = self.multiplier.powi(attempt as i32);
        let millis = (self.initial_delay.as_millis() as f64 * factor) as u64;
        let computed = Duration::from_millis(millis);
        computed.min(self.max_delay)
    }

    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_attempts
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetryOutcome {
    Succeeded { attempts: u32 },
    Exhausted { attempts: u32 },
}

pub struct RetryExecutor {
    policy: RetryPolicy,
}

impl RetryExecutor {
    pub fn new(policy: RetryPolicy) -> Self {
        Self { policy }
    }

    pub fn execute<F, E>(&self, mut action: F) -> Result<RetryOutcome, E>
    where
        F: FnMut(u32) -> Result<(), E>,
    {
        for attempt in 0..=self.policy.max_attempts {
            match action(attempt) {
                Ok(()) => return Ok(RetryOutcome::Succeeded { attempts: attempt + 1 }),
                Err(e) => {
                    if !self.policy.should_retry(attempt + 1) {
                        return Err(e);
                    }
                }
            }
        }
        Ok(RetryOutcome::Exhausted { attempts: self.policy.max_attempts })
    }
}
