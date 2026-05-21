use crate::config::AlertConfig;
use crate::metrics::ProcessMetrics;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    pub pid: u32,
    pub process_name: String,
    pub alert_type: AlertType,
    pub value: f64,
    pub threshold: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    CpuThresholdExceeded,
    MemoryThresholdExceeded,
}

pub fn evaluate_alerts(metrics: &ProcessMetrics, config: &AlertConfig) -> Vec<Alert> {
    let mut alerts = Vec::new();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if metrics.cpu_percent > config.cpu_threshold_percent {
        alerts.push(Alert {
            pid: metrics.pid,
            process_name: metrics.name.clone(),
            alert_type: AlertType::CpuThresholdExceeded,
            value: metrics.cpu_percent,
            threshold: config.cpu_threshold_percent,
            timestamp,
        });
    }

    let memory_mb = metrics.memory_bytes as f64 / 1024.0 / 1024.0;
    if memory_mb > config.memory_threshold_mb {
        alerts.push(Alert {
            pid: metrics.pid,
            process_name: metrics.name.clone(),
            alert_type: AlertType::MemoryThresholdExceeded,
            value: memory_mb,
            threshold: config.memory_threshold_mb,
            timestamp,
        });
    }

    alerts
}
