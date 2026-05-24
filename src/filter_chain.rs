use crate::process_filter::ProcessFilter;
use crate::priority::Priority;

#[derive(Debug, Clone)]
pub struct FilterRule {
    pub filter: ProcessFilter,
    pub priority: Priority,
    pub label: Option<String>,
}

impl FilterRule {
    pub fn new(filter: ProcessFilter, priority: Priority) -> Self {
        Self {
            filter,
            priority,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

#[derive(Debug, Default)]
pub struct FilterChain {
    rules: Vec<FilterRule>,
}

impl FilterChain {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: FilterRule) {
        self.rules.push(rule);
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn remove_rule(&mut self, label: &str) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.label.as_deref() != Some(label));
        self.rules.len() < before
    }

    pub fn evaluate(&self, process_name: &str, pid: u32) -> Option<&FilterRule> {
        self.rules
            .iter()
            .find(|r| r.filter.matches(process_name, pid))
    }

    pub fn evaluate_all(&self, process_name: &str, pid: u32) -> Vec<&FilterRule> {
        self.rules
            .iter()
            .filter(|r| r.filter.matches(process_name, pid))
            .collect()
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn clear(&mut self) {
        self.rules.clear();
    }

    pub fn rules(&self) -> &[FilterRule] {
        &self.rules
    }
}
