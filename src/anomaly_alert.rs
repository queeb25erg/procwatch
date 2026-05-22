use crate::alert::Alert;
use crate::anomaly::Anomaly;
use crate::anomaly_reporter::build_report;

pub fn anomaly_to_alert(anomaly: &Anomaly) -> Alert {
    let report = build_report(anomaly);
    Alert {
        pid: anomaly.pid,
        process_name: report.process_name,
        message: report.summary,
        details: report.details,
        severity: classify_severity(anomaly.deviation.abs()),
    }
}

fn classify_severity(sigma: f64) -> &'static str {
    if sigma >= 4.0 {
        "critical"
    } else if sigma >= 3.0 {
        "high"
    } else {
        "medium"
    }
}

pub fn anomalies_to_alerts(anomalies: &[Anomaly]) -> Vec<Alert> {
    anomalies.iter().map(anomaly_to_alert).collect()
}
