use std::time::Duration;

/// Exponential backoff strategy for retry logic (e.g., webhook delivery).
#[derive(Debug, Clone)]
pub struct Backoff {
    base_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
    current_attempt: u32,
}

impl Backoff {
    pub fn new(base_delay: Duration, max_delay: Duration, multiplier: f64) -> Self {
        Self {
            base_delay,
            max_delay,
            multiplier,
            current_attempt: 0,
        }
    }

    /// Returns the delay for the current attempt and advances the attempt counter.
    pub fn next_delay(&mut self) -> Duration {
        let delay_secs = self.base_delay.as_secs_f64()
            * self.multiplier.powi(self.current_attempt as i32);
        let delay = Duration::from_secs_f64(delay_secs).min(self.max_delay);
        self.current_attempt += 1;
        delay
    }

    /// Returns the delay for the current attempt without advancing.
    pub fn peek_delay(&self) -> Duration {
        let delay_secs = self.base_delay.as_secs_f64()
            * self.multiplier.powi(self.current_attempt as i32);
        Duration::from_secs_f64(delay_secs).min(self.max_delay)
    }

    pub fn attempt(&self) -> u32 {
        self.current_attempt
    }

    pub fn reset(&mut self) {
        self.current_attempt = 0;
    }

    pub fn is_exhausted(&self, max_attempts: u32) -> bool {
        self.current_attempt >= max_attempts
    }
}

impl Default for Backoff {
    fn default() -> Self {
        Self::new(
            Duration::from_millis(500),
            Duration::from_secs(30),
            2.0,
        )
    }
}
