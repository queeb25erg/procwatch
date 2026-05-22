use crate::baseline::Baseline;
use crate::metrics::ProcessMetrics;

#[derive(Debug, Clone, PartialEq)]
pub enum AnomalyKind {
    CpuSpike,
    MemorySpike,
    CpuDrop,
    MemoryDrop,
}

#[derive(Debug, Clone)]
pub struct Anomaly {
    pub pid: u32,
    pub process_name: String,
    pub kind: AnomalyKind,
    pub observed: f64,
    pub expected: f64,
    pub deviation: f64,
}

pub struct AnomalyDetector {
    pub cpu_threshold_sigma: f64,
    pub mem_threshold_sigma: f64,
}

impl AnomalyDetector {
    pub fn new(cpu_threshold_sigma: f64, mem_threshold_sigma: f64) -> Self {
        Self {
            cpu_threshold_sigma,
            mem_threshold_sigma,
        }
    }

    pub fn detect(&self, metrics: &ProcessMetrics, baseline: &Baseline) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        if baseline.cpu_stddev > 0.0 {
            let cpu_deviation =
                (metrics.cpu_usage - baseline.cpu_mean) / baseline.cpu_stddev;
            if cpu_deviation.abs() >= self.cpu_threshold_sigma {
                let kind = if cpu_deviation > 0.0 {
                    AnomalyKind::CpuSpike
                } else {
                    AnomalyKind::CpuDrop
                };
                anomalies.push(Anomaly {
                    pid: metrics.pid,
                    process_name: metrics.name.clone(),
                    kind,
                    observed: metrics.cpu_usage,
                    expected: baseline.cpu_mean,
                    deviation: cpu_deviation,
                });
            }
        }

        if baseline.mem_stddev > 0.0 {
            let mem_deviation =
                (metrics.memory_mb - baseline.mem_mean) / baseline.mem_stddev;
            if mem_deviation.abs() >= self.mem_threshold_sigma {
                let kind = if mem_deviation > 0.0 {
                    AnomalyKind::MemorySpike
                } else {
                    AnomalyKind::MemoryDrop
                };
                anomalies.push(Anomaly {
                    pid: metrics.pid,
                    process_name: metrics.name.clone(),
                    kind,
                    observed: metrics.memory_mb,
                    expected: baseline.mem_mean,
                    deviation: mem_deviation,
                });
            }
        }

        anomalies
    }
}
