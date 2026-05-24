use std::collections::HashMap;

/// A set of key-value labels attached to a process or alert.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelSet {
    labels: HashMap<String, String>,
}

impl LabelSet {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self { labels: map }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.labels.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.labels.get(key).map(|s| s.as_str())
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.labels.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.labels.contains_key(key)
    }

    pub fn matches(&self, selector: &LabelSelector) -> bool {
        selector.matches(self)
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.labels.iter()
    }

    pub fn merge(&mut self, other: &LabelSet) {
        for (k, v) in &other.labels {
            self.labels.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
}

impl Default for LabelSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Selects label sets that contain all required key-value pairs.
#[derive(Debug, Clone)]
pub struct LabelSelector {
    required: HashMap<String, String>,
}

impl LabelSelector {
    pub fn new() -> Self {
        Self {
            required: HashMap::new(),
        }
    }

    pub fn require(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.required.insert(key.into(), value.into());
        self
    }

    pub fn matches(&self, labels: &LabelSet) -> bool {
        self.required
            .iter()
            .all(|(k, v)| labels.get(k.as_str()) == Some(v.as_str()))
    }

    pub fn is_empty(&self) -> bool {
        self.required.is_empty()
    }
}

impl Default for LabelSelector {
    fn default() -> Self {
        Self::new()
    }
}
