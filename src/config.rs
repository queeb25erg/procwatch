use serde::Deserialize;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub poll_interval_secs: u64,
    pub webhook_url: String,
    pub processes: Vec<ProcessConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub cpu_threshold_pct: f32,
    pub mem_threshold_mb: u64,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            poll_interval_secs: 30,
            webhook_url: String::from("https://hooks.example.com/alert"),
            processes: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_valid_config() {
        let toml_content = r#"
poll_interval_secs = 15
webhook_url = "https://hooks.example.com/test"

[[processes]]
name = "nginx"
cpu_threshold_pct = 80.0
mem_threshold_mb = 512

[[processes]]
name = "postgres"
cpu_threshold_pct = 90.0
mem_threshold_mb = 2048
"#;
        let mut tmpfile = NamedTempFile::new().unwrap();
        tmpfile.write_all(toml_content.as_bytes()).unwrap();
        let config = Config::from_file(tmpfile.path()).unwrap();
        assert_eq!(config.poll_interval_secs, 15);
        assert_eq!(config.processes.len(), 2);
        assert_eq!(config.processes[0].name, "nginx");
        assert_eq!(config.processes[1].mem_threshold_mb, 2048);
    }

    #[test]
    fn test_missing_file_returns_error() {
        let result = Config::from_file(Path::new("/nonexistent/path/config.toml"));
        assert!(result.is_err());
    }
}
