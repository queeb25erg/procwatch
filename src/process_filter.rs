use serde::Deserialize;

/// Criteria for filtering which processes to monitor.
#[derive(Debug, Clone, Deserialize)]
pub struct ProcessFilter {
    /// Only monitor processes whose name contains this substring (case-insensitive).
    pub name_contains: Option<String>,
    /// Only monitor processes with this exact PID.
    pub pid: Option<u32>,
    /// Only monitor processes owned by this user.
    pub user: Option<String>,
    /// Minimum CPU usage (%) before a process is considered for alerting.
    pub min_cpu_percent: Option<f64>,
    /// Minimum memory usage (MB) before a process is considered for alerting.
    pub min_mem_mb: Option<f64>,
}

impl ProcessFilter {
    /// Returns `true` if the given process entry matches all configured criteria.
    pub fn matches(&self, name: &str, pid: u32, user: &str, cpu: f64, mem_mb: f64) -> bool {
        if let Some(ref substr) = self.name_contains {
            if !name.to_lowercase().contains(&substr.to_lowercase()) {
                return false;
            }
        }
        if let Some(expected_pid) = self.pid {
            if pid != expected_pid {
                return false;
            }
        }
        if let Some(ref expected_user) = self.user {
            if user != expected_user {
                return false;
            }
        }
        if let Some(min_cpu) = self.min_cpu_percent {
            if cpu < min_cpu {
                return false;
            }
        }
        if let Some(min_mem) = self.min_mem_mb {
            if mem_mb < min_mem {
                return false;
            }
        }
        true
    }

    /// Returns `true` if no filter criteria are set (matches everything).
    pub fn is_empty(&self) -> bool {
        self.name_contains.is_none()
            && self.pid.is_none()
            && self.user.is_none()
            && self.min_cpu_percent.is_none()
            && self.min_mem_mb.is_none()
    }
}

impl Default for ProcessFilter {
    fn default() -> Self {
        Self {
            name_contains: None,
            pid: None,
            user: None,
            min_cpu_percent: None,
            min_mem_mb: None,
        }
    }
}
