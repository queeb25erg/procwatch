use std::collections::HashMap;
use crate::metrics::ProcessMetrics;

/// Aggregates metrics across multiple samples for a given process.
#[derive(Debug, Clone)]
pub struct AggregatedMetrics {
    pub pid: u32,
    pub name: String,
    pub sample_count: usize,
    pub avg_cpu_percent: f64,
    pub max_cpu_percent: f64,
    pub avg_mem_bytes: u64,
    pub max_mem_bytes: u64,
}

#[derive(Debug, Default)]
pub struct Aggregator {
    samples: HashMap<u32, Vec<ProcessMetrics>>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_sample(&mut self, metrics: ProcessMetrics) {
        self.samples.entry(metrics.pid).or_default().push(metrics);
    }

    pub fn aggregate(&self, pid: u32) -> Option<AggregatedMetrics> {
        let samples = self.samples.get(&pid)?;
        if samples.is_empty() {
            return None;
        }

        let name = samples[0].name.clone();
        let count = samples.len();

        let avg_cpu = samples.iter().map(|s| s.cpu_percent).sum::<f64>() / count as f64;
        let max_cpu = samples.iter().map(|s| s.cpu_percent).cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg_mem = samples.iter().map(|s| s.mem_bytes).sum::<u64>() / count as u64;
        let max_mem = samples.iter().map(|s| s.mem_bytes).max().copied().unwrap_or(0);

        Some(AggregatedMetrics {
            pid,
            name,
            sample_count: count,
            avg_cpu_percent: avg_cpu,
            max_cpu_percent: max_cpu,
            avg_mem_bytes: avg_mem,
            max_mem_bytes: max_mem,
        })
    }

    pub fn aggregate_all(&self) -> Vec<AggregatedMetrics> {
        self.samples.keys().filter_map(|pid| self.aggregate(*pid)).collect()
    }

    pub fn clear(&mut self, pid: u32) {
        self.samples.remove(&pid);
    }

    pub fn clear_all(&mut self) {
        self.samples.clear();
    }

    pub fn tracked_pids(&self) -> Vec<u32> {
        self.samples.keys().copied().collect()
    }
}
