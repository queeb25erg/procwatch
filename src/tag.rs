use std::collections::HashMap;

/// Tags are key-value metadata attached to process alerts and metrics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagSet {
    tags: HashMap<String, String>,
}

impl TagSet {
    pub fn new() -> Self {
        TagSet {
            tags: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<String, String>) -> Self {
        TagSet { tags: map }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(|s| s.as_str())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.tags.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.tags.remove(key)
    }

    pub fn len(&self) -> usize {
        self.tags.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.tags.iter()
    }

    /// Merge another TagSet into this one; existing keys are overwritten.
    pub fn merge(&mut self, other: &TagSet) {
        for (k, v) in &other.tags {
            self.tags.insert(k.clone(), v.clone());
        }
    }

    /// Returns a formatted string like "env=prod,service=api" for use in payloads.
    pub fn to_label_string(&self) -> String {
        let mut pairs: Vec<String> = self
            .tags
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        pairs.sort();
        pairs.join(",")
    }
}

impl Default for TagSet {
    fn default() -> Self {
        TagSet::new()
    }
}
