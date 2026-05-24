use crate::quota_manager::QuotaManager;

#[derive(Debug, Clone)]
pub struct QuotaReport {
    pub pid: u32,
    pub remaining_cpu: Option<f64>,
    pub remaining_memory_mb: Option<f64>,
    pub cpu_exhausted: bool,
    pub memory_exhausted: bool,
}

impl QuotaReport {
    pub fn is_any_exhausted(&self) -> bool {
        self.cpu_exhausted || self.memory_exhausted
    }

    pub fn summary(&self) -> String {
        format!(
            "pid={} cpu_remaining={:.2?} mem_remaining={:.2?} exhausted={}",
            self.pid,
            self.remaining_cpu,
            self.remaining_memory_mb,
            self.is_any_exhausted()
        )
    }
}

pub struct QuotaReporter {
    manager: QuotaManager,
}

impl QuotaReporter {
    pub fn new(manager: QuotaManager) -> Self {
        Self { manager }
    }

    pub fn report(&self, pid: u32) -> QuotaReport {
        let remaining_cpu = self.manager.remaining_cpu(pid);
        let remaining_memory_mb = self.manager.remaining_memory(pid);

        QuotaReport {
            pid,
            cpu_exhausted: remaining_cpu.map(|v| v <= 0.0).unwrap_or(false),
            memory_exhausted: remaining_memory_mb.map(|v| v <= 0.0).unwrap_or(false),
            remaining_cpu,
            remaining_memory_mb,
        }
    }

    pub fn report_all(&self, pids: &[u32]) -> Vec<QuotaReport> {
        pids.iter().map(|&pid| self.report(pid)).collect()
    }

    pub fn exhausted_processes(&self, pids: &[u32]) -> Vec<QuotaReport> {
        self.report_all(pids)
            .into_iter()
            .filter(|r| r.is_any_exhausted())
            .collect()
    }
}
