use crate::anomaly::{Anomaly, AnomalyDetector};
use crate::baseline::Baseline;
use crate::metrics::ProcessMetrics;
use std::collections::HashMap;

pub struct AnomalyManager {
    detector: AnomalyDetector,
    baselines: HashMap<u32, Baseline>,
}

impl AnomalyManager {
    pub fn new(cpu_sigma: f64, mem_sigma: f64) -> Self {
        Self {
            detector: AnomalyDetector::new(cpu_sigma, mem_sigma),
            baselines: HashMap::new(),
        }
    }

    pub fn register_baseline(&mut self, pid: u32, baseline: Baseline) {
        self.baselines.insert(pid, baseline);
    }

    pub fn remove_baseline(&mut self, pid: u32) {
        self.baselines.remove(&pid);
    }

    pub fn check(&self, metrics: &ProcessMetrics) -> Vec<Anomaly> {
        match self.baselines.get(&metrics.pid) {
            Some(baseline) => self.detector.detect(metrics, baseline),
            None => vec![],
        }
    }

    pub fn check_all(&self, metrics_list: &[ProcessMetrics]) -> Vec<Anomaly> {
        metrics_list
            .iter()
            .flat_map(|m| self.check(m))
            .collect()
    }

    pub fn baseline_count(&self) -> usize {
        self.baselines.len()
    }
}
