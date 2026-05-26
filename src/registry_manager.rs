use crate::registry::Registry;
use crate::metrics::ProcessMetrics;

pub struct RegistryManager {
    registry: Registry,
    stale_threshold_secs: u64,
}

impl RegistryManager {
    pub fn new(stale_threshold_secs: u64) -> Self {
        Self {
            registry: Registry::new(),
            stale_threshold_secs,
        }
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn track(&self, pid: u32, name: String, metrics: ProcessMetrics) {
        self.registry.register(pid, name, metrics);
    }

    pub fn remove(&self, pid: u32) -> bool {
        self.registry.unregister(pid)
    }

    pub fn run_eviction(&self) -> usize {
        let evicted = self.registry.evict_stale(self.stale_threshold_secs);
        if evicted > 0 {
            log::info!("Registry evicted {} stale entries", evicted);
        }
        evicted
    }

    pub fn summary(&self) -> RegistrySummary {
        let entries = self.registry.all();
        let total = entries.len();
        let avg_age = if total == 0 {
            0
        } else {
            entries.iter().map(|e| e.age_secs()).sum::<u64>() / total as u64
        };
        RegistrySummary { total, avg_age_secs: avg_age }
    }
}

#[derive(Debug)]
pub struct RegistrySummary {
    pub total: usize,
    pub avg_age_secs: u64,
}
