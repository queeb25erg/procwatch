use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

#[derive(Debug, Clone)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub uptime: Duration,
    pub checks_passed: usize,
    pub checks_failed: usize,
    pub last_check: Instant,
}

impl HealthReport {
    pub fn new(status: HealthStatus, uptime: Duration, passed: usize, failed: usize) -> Self {
        Self {
            status,
            uptime,
            checks_passed: passed,
            checks_failed: failed,
            last_check: Instant::now(),
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy)
    }

    pub fn summary(&self) -> String {
        let status_str = match &self.status {
            HealthStatus::Healthy => "healthy".to_string(),
            HealthStatus::Degraded(msg) => format!("degraded: {}", msg),
            HealthStatus::Unhealthy(msg) => format!("unhealthy: {}", msg),
        };
        format!(
            "status={} uptime={:.1}s passed={} failed={}",
            status_str,
            self.uptime.as_secs_f64(),
            self.checks_passed,
            self.checks_failed,
        )
    }
}

pub struct HealthChecker {
    started_at: Instant,
    checks_passed: usize,
    checks_failed: usize,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            started_at: Instant::now(),
            checks_passed: 0,
            checks_failed: 0,
        }
    }

    pub fn record_pass(&mut self) {
        self.checks_passed += 1;
    }

    pub fn record_fail(&mut self) {
        self.checks_failed += 1;
    }

    pub fn report(&self) -> HealthReport {
        let uptime = self.started_at.elapsed();
        let total = self.checks_passed + self.checks_failed;
        let status = if total == 0 {
            HealthStatus::Healthy
        } else {
            let fail_ratio = self.checks_failed as f64 / total as f64;
            if fail_ratio == 0.0 {
                HealthStatus::Healthy
            } else if fail_ratio < 0.5 {
                HealthStatus::Degraded(format!("{} checks failed", self.checks_failed))
            } else {
                HealthStatus::Unhealthy(format!("{}/{} checks failed", self.checks_failed, total))
            }
        };
        HealthReport::new(status, uptime, self.checks_passed, self.checks_failed)
    }
}
