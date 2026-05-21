use serde::Deserialize;
use std::fs;
use std::path::Path;
use crate::process_filter::ProcessFilter;

/// Top-level daemon configuration loaded from a TOML file.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// How often to poll process metrics, in seconds.
    #[serde(default = "default_interval")]
    pub poll_interval_secs: u64,

    /// Webhook URL to POST alerts to.
    pub webhook_url: String,

    /// Optional bearer token for webhook authentication.
    pub webhook_token: Option<String>,

    /// CPU usage threshold (%) above which an alert is triggered.
    #[serde(default = "default_cpu_threshold")]
    pub cpu_alert_threshold: f64,

    /// Memory usage threshold (MB) above which an alert is triggered.
    #[serde(default = "default_mem_threshold")]
    pub mem_alert_mb_threshold: f64,

    /// Minimum seconds between repeated alerts for the same process.
    #[serde(default = "default_throttle_secs")]
    pub alert_throttle_secs: u64,

    /// Optional filter controlling which processes are monitored.
    #[serde(default)]
    pub filter: ProcessFilter,
}

fn default_interval() -> u64 { 10 }
fn default_cpu_threshold() -> f64 { 80.0 }
fn default_mem_threshold() -> f64 { 512.0 }
fn default_throttle_secs() -> u64 { 300 }

impl Config {
    /// Load and parse a TOML configuration file from `path`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration values, returning an error for invalid settings.
    fn validate(&self) -> anyhow::Result<()> {
        if self.webhook_url.is_empty() {
            anyhow::bail!("webhook_url must not be empty");
        }
        if self.poll_interval_secs == 0 {
            anyhow::bail!("poll_interval_secs must be greater than 0");
        }
        if !(0.0..=100.0).contains(&self.cpu_alert_threshold) {
            anyhow::bail!("cpu_alert_threshold must be between 0 and 100");
        }
        if self.mem_alert_mb_threshold <= 0.0 {
            anyhow::bail!("mem_alert_mb_threshold must be positive");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_config(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "{}", content).unwrap();
        f
    }

    #[test]
    fn parses_minimal_config() {
        let f = write_config(r#"webhook_url = "https://hooks.example.com/abc""#);
        let cfg = Config::from_file(f.path()).unwrap();
        assert_eq!(cfg.poll_interval_secs, 10);
        assert_eq!(cfg.cpu_alert_threshold, 80.0);
        assert!(cfg.filter.is_empty());
    }

    #[test]
    fn parses_filter_section() {
        let f = write_config(
            r#"
            webhook_url = "https://hooks.example.com/abc"
            [filter]
            name_contains = "nginx"
            min_cpu_percent = 5.0
            "#,
        );
        let cfg = Config::from_file(f.path()).unwrap();
        assert_eq!(cfg.filter.name_contains.as_deref(), Some("nginx"));
        assert_eq!(cfg.filter.min_cpu_percent, Some(5.0));
    }

    #[test]
    fn rejects_empty_webhook_url() {
        let f = write_config(r#"webhook_url = """");
        assert!(Config::from_file(f.path()).is_err());
    }
}
