use crate::baseline::BaselineStore;
use crate::baseline_checker::{BaselineAnomaly, BaselineChecker, BaselineCheckerConfig};
use crate::baseline_reporter::BaselineReporter;
use crate::metrics::ProcessMetrics;

pub struct BaselineManager {
    store: BaselineStore,
    checker: BaselineChecker,
}

impl BaselineManager {
    pub fn new(config: BaselineCheckerConfig) -> Self {
        Self {
            store: BaselineStore::new(),
            checker: BaselineChecker::new(config),
        }
    }

    /// Feed a new metrics sample. Updates the baseline and checks for anomalies.
    /// Returns an anomaly report string if one is detected.
    pub fn process(&mut self, metrics: &ProcessMetrics) -> Option<(BaselineAnomaly, String, &'static str)> {
        let anomaly = self.checker.check(metrics, &self.store);
        self.store.update(metrics);

        anomaly.map(|a| {
            let msg = BaselineReporter::format_anomaly(&a);
            let severity = BaselineReporter::severity(&a);
            (a, msg, severity)
        })
    }

    pub fn evict(&mut self, pid: u32) {
        self.store.remove(pid);
    }

    pub fn baseline_count(&self) -> usize {
        self.store.len()
    }
}

impl Default for BaselineManager {
    fn default() -> Self {
        Self::new(BaselineCheckerConfig::default())
    }
}
