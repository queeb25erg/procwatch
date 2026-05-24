use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Tracks whether monitored processes are still alive and responsive.
#[derive(Debug, Clone)]
pub struct WatchdogEntry {
    pub pid: u32,
    pub name: String,
    pub last_seen: Instant,
    pub missed_ticks: u32,
    pub max_missed_ticks: u32,
}

impl WatchdogEntry {
    pub fn new(pid: u32, name: impl Into<String>, max_missed_ticks: u32) -> Self {
        Self {
            pid,
            name: name.into(),
            last_seen: Instant::now(),
            missed_ticks: 0,
            max_missed_ticks,
        }
    }

    pub fn touch(&mut self) {
        self.last_seen = Instant::now();
        self.missed_ticks = 0;
    }

    pub fn tick(&mut self) -> WatchdogStatus {
        let elapsed = self.last_seen.elapsed();
        if elapsed > Duration::from_secs(5) {
            self.missed_ticks += 1;
        }
        if self.missed_ticks >= self.max_missed_ticks {
            WatchdogStatus::Unresponsive
        } else {
            WatchdogStatus::Ok
        }
    }

    pub fn is_stale(&self, timeout: Duration) -> bool {
        self.last_seen.elapsed() > timeout
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WatchdogStatus {
    Ok,
    Unresponsive,
}

#[derive(Debug, Default)]
pub struct Watchdog {
    entries: HashMap<u32, WatchdogEntry>,
    max_missed_ticks: u32,
}

impl Watchdog {
    pub fn new(max_missed_ticks: u32) -> Self {
        Self {
            entries: HashMap::new(),
            max_missed_ticks,
        }
    }

    pub fn register(&mut self, pid: u32, name: impl Into<String>) {
        self.entries.insert(
            pid,
            WatchdogEntry::new(pid, name, self.max_missed_ticks),
        );
    }

    pub fn unregister(&mut self, pid: u32) {
        self.entries.remove(&pid);
    }

    pub fn touch(&mut self, pid: u32) {
        if let Some(entry) = self.entries.get_mut(&pid) {
            entry.touch();
        }
    }

    pub fn tick_all(&mut self) -> Vec<(u32, String, WatchdogStatus)> {
        self.entries
            .values_mut()
            .map(|e| {
                let status = e.tick();
                (e.pid, e.name.clone(), status)
            })
            .collect()
    }

    pub fn unresponsive_pids(&mut self) -> Vec<u32> {
        self.tick_all()
            .into_iter()
            .filter(|(_, _, s)| *s == WatchdogStatus::Unresponsive)
            .map(|(pid, _, _)| pid)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
