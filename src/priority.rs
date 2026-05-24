use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl Priority {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(Priority::Low),
            "medium" => Some(Priority::Medium),
            "high" => Some(Priority::High),
            "critical" => Some(Priority::Critical),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }

    pub fn weight(&self) -> u32 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 5,
            Priority::High => 10,
            Priority::Critical => 20,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PriorityRule {
    pub process_name: String,
    pub priority: Priority,
}

#[derive(Debug, Default)]
pub struct PriorityManager {
    rules: Vec<PriorityRule>,
    cache: HashMap<String, Priority>,
}

impl PriorityManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, process_name: &str, priority: Priority) {
        self.cache.remove(process_name);
        self.rules.push(PriorityRule {
            process_name: process_name.to_string(),
            priority,
        });
    }

    pub fn resolve(&mut self, process_name: &str) -> Priority {
        if let Some(cached) = self.cache.get(process_name) {
            return cached.clone();
        }
        let resolved = self
            .rules
            .iter()
            .filter(|r| r.process_name == process_name)
            .map(|r| r.priority.clone())
            .max()
            .unwrap_or(Priority::Low);
        self.cache.insert(process_name.to_string(), resolved.clone());
        resolved
    }

    pub fn remove_rules_for(&mut self, process_name: &str) {
        self.rules.retain(|r| r.process_name != process_name);
        self.cache.remove(process_name);
    }

    pub fn all_rules(&self) -> &[PriorityRule] {
        &self.rules
    }
}
