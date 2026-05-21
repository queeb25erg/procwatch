use crate::alert::Alert;
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize)]
struct WebhookPayload<'a> {
    alerts: &'a [Alert],
    host: &'a str,
    source: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Webhook returned non-success status: {0}")]
    Status(u16),
}

pub struct WebhookSender {
    client: Client,
    url: String,
    hostname: String,
}

impl WebhookSender {
    pub fn new(url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        Self { client, url, hostname }
    }

    pub async fn send(&self, alerts: &[Alert]) -> Result<(), WebhookError> {
        if alerts.is_empty() {
            return Ok(());
        }
        let payload = WebhookPayload {
            alerts,
            host: &self.hostname,
            source: "procwatch",
        };
        let response = self.client.post(&self.url).json(&payload).send().await?;
        if !response.status().is_success() {
            return Err(WebhookError::Status(response.status().as_u16()));
        }
        Ok(())
    }
}
