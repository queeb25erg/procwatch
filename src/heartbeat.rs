use std::time::{Duration, Instant};

/// Tracks periodic heartbeat signals to detect stalled or unresponsive components.
#[derive(Debug, Clone)]
pub struct Heartbeat {
    pub name: String,
    pub interval: Duration,
    last_beat: Option<Instant>,
    missed_count: u32,
    pub max_missed: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeartbeatStatus {
    Alive,
    Stale { missed: u32 },
    Dead,
}

impl Heartbeat {
    pub fn new(name: impl Into<String>, interval: Duration, max_missed: u32) -> Self {
        Self {
            name: name.into(),
            interval,
            last_beat: None,
            missed_count: 0,
            max_missed,
        }
    }

    /// Record a heartbeat at the current instant.
    pub fn beat(&mut self) {
        self.last_beat = Some(Instant::now());
        self.missed_count = 0;
    }

    /// Evaluate the current status based on elapsed time.
    pub fn status(&mut self) -> HeartbeatStatus {
        let Some(last) = self.last_beat else {
            return HeartbeatStatus::Stale { missed: 0 };
        };

        let elapsed = last.elapsed();
        if elapsed <= self.interval {
            self.missed_count = 0;
            return HeartbeatStatus::Alive;
        }

        let missed = (elapsed.as_secs_f64() / self.interval.as_secs_f64()) as u32;
        self.missed_count = missed;

        if missed >= self.max_missed {
            HeartbeatStatus::Dead
        } else {
            HeartbeatStatus::Stale { missed }
        }
    }

    pub fn missed_count(&self) -> u32 {
        self.missed_count
    }

    pub fn last_beat(&self) -> Option<Instant> {
        self.last_beat
    }

    pub fn is_alive(&mut self) -> bool {
        self.status() == HeartbeatStatus::Alive
    }
}
