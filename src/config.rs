use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub poll_interval_secs: u64,
    pub webhook: WebhookConfig,
    pub alerts: AlertConfig,
    pub processes: Vec<ProcessFilter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlertConfig {
    pub cpu_threshold_percent: f64,
    pub memory_threshold_mb: f64,
    pub repeat_interval_cycles: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessFilter {
    pub name: Option<String>,
    pub pid: Option<u32>,
}

impl Config {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file {:?}: {}", path, e))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config: {}", e))?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.webhook.url.is_empty() {
            anyhow::bail!("webhook.url must not be empty");
        }
        if self.poll_interval_secs == 0 {
            anyhow::bail!("poll_interval_secs must be greater than 0");
        }
        if self.alerts.cpu_threshold_percent <= 0.0 || self.alerts.cpu_threshold_percent > 100.0 {
            anyhow::bail!("cpu_threshold_percent must be between 0 and 100");
        }
        if self.alerts.memory_threshold_mb <= 0.0 {
            anyhow::bail!("memory_threshold_mb must be greater than 0");
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
    fn valid_config_parses() {
        let toml = r#"
            poll_interval_secs = 5
            [webhook]
            url = "http://example.com/hook"
            [alerts]
            cpu_threshold_percent = 80.0
            memory_threshold_mb = 512.0
            [[processes]]
            name = "nginx"
        "#;
        let f = write_config(toml);
        let cfg = Config::from_file(f.path()).unwrap();
        assert_eq!(cfg.poll_interval_secs, 5);
        assert_eq!(cfg.alerts.cpu_threshold_percent, 80.0);
    }

    #[test]
    fn invalid_cpu_threshold_rejected() {
        let toml = r#"
            poll_interval_secs = 5
            [webhook]
            url = "http://example.com/hook"
            [alerts]
            cpu_threshold_percent = 150.0
            memory_threshold_mb = 512.0
        "#;
        let f = write_config(toml);
        assert!(Config::from_file(f.path()).is_err());
    }
}
