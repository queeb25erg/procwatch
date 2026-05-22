use crate::baseline_checker::BaselineAnomaly;

pub struct BaselineReporter;

impl BaselineReporter {
    pub fn format_anomaly(anomaly: &BaselineAnomaly) -> String {
        let mut parts = Vec::new();

        if anomaly.cpu_deviation_pct.abs() > 0.0 {
            let direction = if anomaly.cpu_deviation_pct > 0.0 { "above" } else { "below" };
            parts.push(format!(
                "CPU {:.1}% {} baseline ({:.1}% vs {:.1}%)",
                anomaly.cpu_deviation_pct.abs(),
                direction,
                anomaly.current_cpu,
                anomaly.baseline_cpu
            ));
        }

        if anomaly.mem_deviation_pct.abs() > 0.0 {
            let direction = if anomaly.mem_deviation_pct > 0.0 { "above" } else { "below" };
            parts.push(format!(
                "MEM {:.1}% {} baseline ({:.1} MB vs {:.1} MB)",
                anomaly.mem_deviation_pct.abs(),
                direction,
                anomaly.current_mem,
                anomaly.baseline_mem
            ));
        }

        format!(
            "[BASELINE] pid={} name={} — {}",
            anomaly.pid,
            anomaly.process_name,
            parts.join(", ")
        )
    }

    pub fn severity(anomaly: &BaselineAnomaly) -> &'static str {
        let max_dev = anomaly
            .cpu_deviation_pct
            .abs()
            .max(anomaly.mem_deviation_pct.abs());
        if max_dev >= 100.0 {
            "critical"
        } else if max_dev >= 50.0 {
            "warning"
        } else {
            "info"
        }
    }
}
