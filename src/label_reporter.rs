use crate::label::{LabelSet, LabelSelector};
use crate::label_manager::LabelManager;

pub struct LabelReport {
    pub process: String,
    pub labels: Vec<(String, String)>,
}

impl LabelReport {
    pub fn from_label_set(process: impl Into<String>, set: &LabelSet) -> Self {
        let mut labels: Vec<(String, String)> = set
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        labels.sort_by(|a, b| a.0.cmp(&b.0));
        Self {
            process: process.into(),
            labels,
        }
    }

    pub fn format(&self) -> String {
        let pairs: Vec<String> = self
            .labels
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("[{}] {}", self.process, pairs.join(", "))
    }
}

pub struct LabelReporter;

impl LabelReporter {
    /// Generate reports for all processes matching the selector.
    pub fn report_matching(
        manager: &LabelManager,
        selector: &LabelSelector,
    ) -> Vec<LabelReport> {
        let mut matches = manager.find_matching(selector);
        matches.sort();
        matches
            .into_iter()
            .map(|name| {
                let set = manager.get_labels(name);
                LabelReport::from_label_set(name, &set)
            })
            .collect()
    }

    /// Generate a report for a single process.
    pub fn report_process(manager: &LabelManager, process: &str) -> LabelReport {
        let set = manager.get_labels(process);
        LabelReport::from_label_set(process, &set)
    }
}
