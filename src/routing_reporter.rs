use crate::routing_manager::RoutingManager;
use crate::alert::Alert;

pub struct RoutingReport {
    pub alert_process: String,
    pub matched_routes: usize,
    pub target_names: Vec<String>,
}

pub struct RoutingReporter {
    manager: RoutingManager,
}

impl RoutingReporter {
    pub fn new(manager: RoutingManager) -> Self {
        RoutingReporter { manager }
    }

    pub fn report(&self, alert: &Alert) -> RoutingReport {
        let targets = self.manager.resolve_targets(alert);
        let target_names = targets.iter().map(|t| format!("{:?}", t)).collect();
        RoutingReport {
            alert_process: alert.process_name.clone(),
            matched_routes: target_names.len(),
            target_names,
        }
    }

    pub fn summary(&self, alert: &Alert) -> String {
        let report = self.report(alert);
        if report.matched_routes == 0 {
            format!("[routing] no routes matched for '{}'", report.alert_process)
        } else {
            format!(
                "[routing] '{}' -> {} target(s): {}",
                report.alert_process,
                report.matched_routes,
                report.target_names.join(", ")
            )
        }
    }
}
