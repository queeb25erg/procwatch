use crate::baseline::BaselineStore;
use crate::metrics::ProcessMetrics;

#[derive(Debug, Clone)]
pub struct BaselineAnomaly {
    pub pid: u32,
    pub process_name: String,
    pub cpu_deviation_pct: f64,
    pub mem_deviation_pct: f64,
    pub current_cpu: f64,
    pub current_mem: f64,
    pub baseline_cpu: f64,
    pub baseline_mem: f64,
}

#[derive(Debug, Clone)]
pub struct BaselineCheckerConfig {
    pub min_samples: usize,
    pub cpu_deviation_threshold_pct: f64,
    pub mem_deviation_threshold_pct: f64,
}

impl Default for BaselineCheckerConfig {
    fn default() -> Self {
        Self {
            min_samples: 10,
            cpu_deviation_threshold_pct: 50.0,
            mem_deviation_threshold_pct: 30.0,
        }
    }
}

pub struct BaselineChecker {
    config: BaselineCheckerConfig,
}

impl BaselineChecker {
    pub fn new(config: BaselineCheckerConfig) -> Self {
        Self { config }
    }

    pub fn check(
        &self,
        metrics: &ProcessMetrics,
        store: &BaselineStore,
    ) -> Option<BaselineAnomaly> {
        let baseline = store.get(metrics.pid)?;

        if !baseline.is_mature(self.config.min_samples) {
            return None;
        }

        let cpu_dev = baseline.cpu_deviation(metrics.cpu_percent);
        let mem_dev = baseline.mem_deviation(metrics.mem_mb);

        let cpu_exceeded = cpu_dev.abs() >= self.config.cpu_deviation_threshold_pct;
        let mem_exceeded = mem_dev.abs() >= self.config.mem_deviation_threshold_pct;

        if cpu_exceeded || mem_exceeded {
            Some(BaselineAnomaly {
                pid: metrics.pid,
                process_name: metrics.name.clone(),
                cpu_deviation_pct: cpu_dev,
                mem_deviation_pct: mem_dev,
                current_cpu: metrics.cpu_percent,
                current_mem: metrics.mem_mb,
                baseline_cpu: baseline.avg_cpu,
                baseline_mem: baseline.avg_mem,
            })
        } else {
            None
        }
    }
}
