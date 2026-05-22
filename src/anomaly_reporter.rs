use crate::anomaly::{Anomaly, AnomalyKind};

pub struct AnomalyReport {
    pub process_name: String,
    pub pid: u32,
    pub summary: String,
    pub details: String,
}

pub fn build_report(anomaly: &Anomaly) -> AnomalyReport {
    let kind_label = match anomaly.kind {
        AnomalyKind::CpuSpike => "CPU spike",
        AnomalyKind::CpuDrop => "CPU drop",
        AnomalyKind::MemorySpike => "Memory spike",
        AnomalyKind::MemoryDrop => "Memory drop",
    };

    let unit = match anomaly.kind {
        AnomalyKind::CpuSpike | AnomalyKind::CpuDrop => "%",
        AnomalyKind::MemorySpike | AnomalyKind::MemoryDrop => "MB",
    };

    let summary = format!(
        "[{}] {} detected for '{}' (PID {})",
        kind_label, kind_label, anomaly.process_name, anomaly.pid
    );

    let details = format!(
        "Process '{}' (PID {}) showed a {} anomaly: observed={:.2}{}, expected={:.2}{}, deviation={:.2}σ",
        anomaly.process_name,
        anomaly.pid,
        kind_label,
        anomaly.observed,
        unit,
        anomaly.expected,
        unit,
        anomaly.deviation
    );

    AnomalyReport {
        process_name: anomaly.process_name.clone(),
        pid: anomaly.pid,
        summary,
        details,
    }
}
