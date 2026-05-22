use std::collections::HashMap;
use crate::metrics::ProcessMetrics;

/// Stores baseline resource usage for processes to detect anomalies
#[derive(Debug, Clone)]
pub struct Baseline {
    pub pid: u32,
    pub avg_cpu: f64,
    pub avg_mem: f64,
    pub sample_count: usize,
}

impl Baseline {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            avg_cpu: 0.0,
            avg_mem: 0.0,
            sample_count: 0,
        }
    }

    pub fn update(&mut self, metrics: &ProcessMetrics) {
        let n = self.sample_count as f64;
        self.avg_cpu = (self.avg_cpu * n + metrics.cpu_percent) / (n + 1.0);
        self.avg_mem = (self.avg_mem * n + metrics.mem_mb) / (n + 1.0);
        self.sample_count += 1;
    }

    pub fn is_mature(&self, min_samples: usize) -> bool {
        self.sample_count >= min_samples
    }

    pub fn cpu_deviation(&self, current_cpu: f64) -> f64 {
        if self.avg_cpu == 0.0 {
            return 0.0;
        }
        ((current_cpu - self.avg_cpu) / self.avg_cpu) * 100.0
    }

    pub fn mem_deviation(&self, current_mem: f64) -> f64 {
        if self.avg_mem == 0.0 {
            return 0.0;
        }
        ((current_mem - self.avg_mem) / self.avg_mem) * 100.0
    }
}

#[derive(Debug, Default)]
pub struct BaselineStore {
    baselines: HashMap<u32, Baseline>,
}

impl BaselineStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, metrics: &ProcessMetrics) {
        let baseline = self.baselines.entry(metrics.pid).or_insert_with(|| Baseline::new(metrics.pid));
        baseline.update(metrics);
    }

    pub fn get(&self, pid: u32) -> Option<&Baseline> {
        self.baselines.get(&pid)
    }

    pub fn remove(&mut self, pid: u32) {
        self.baselines.remove(&pid);
    }

    pub fn len(&self) -> usize {
        self.baselines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.baselines.is_empty()
    }
}
