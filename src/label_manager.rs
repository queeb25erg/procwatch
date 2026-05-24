use std::collections::HashMap;
use crate::label::{LabelSet, LabelSelector};

/// Manages label sets keyed by process name.
pub struct LabelManager {
    store: HashMap<String, LabelSet>,
    defaults: LabelSet,
}

impl LabelManager {
    pub fn new(defaults: LabelSet) -> Self {
        Self {
            store: HashMap::new(),
            defaults,
        }
    }

    /// Register or replace the label set for a process.
    pub fn set_labels(&mut self, process: impl Into<String>, labels: LabelSet) {
        self.store.insert(process.into(), labels);
    }

    /// Get labels for a process, merged with defaults.
    pub fn get_labels(&self, process: &str) -> LabelSet {
        let mut merged = self.defaults.clone();
        if let Some(specific) = self.store.get(process) {
            merged.merge(specific);
        }
        merged
    }

    /// Remove labels for a process.
    pub fn remove(&mut self, process: &str) -> Option<LabelSet> {
        self.store.remove(process)
    }

    /// Find all processes whose labels match the given selector.
    pub fn find_matching(&self, selector: &LabelSelector) -> Vec<&str> {
        self.store
            .iter()
            .filter(|(_, labels)| {
                let mut merged = self.defaults.clone();
                merged.merge(labels);
                merged.matches(selector)
            })
            .map(|(name, _)| name.as_str())
            .collect()
    }

    pub fn process_count(&self) -> usize {
        self.store.len()
    }
}
