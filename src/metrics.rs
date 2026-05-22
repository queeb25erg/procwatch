use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub memory_bytes: u64,
    pub memory_percent: f64,
}

fn read_proc_stat(pid: u32) -> io::Result<Vec<String>> {
    let path = format!("/proc/{}/stat", pid);
    let content = fs::read_to_string(path)?;
    Ok(content.split_whitespace().map(String::from).collect())
}

fn read_proc_status(pid: u32) -> io::Result<u64> {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if line.starts_with("VmRSS:") {
            let kb: u64 = line
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            return Ok(kb * 1024);
        }
    }
    Ok(0)
}

fn total_memory_bytes() -> u64 {
    fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|content| {
            content.lines().find(|l| l.starts_with("MemTotal:")).and_then(|line| {
                line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok())
            })
        })
        .unwrap_or(1)
        * 1024
}

/// Returns the uptime of the system in seconds by reading `/proc/uptime`.
fn system_uptime_seconds() -> f64 {
    fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|content| {
            content
                .split_whitespace()
                .next()
                .and_then(|v| v.parse::<f64>().ok())
        })
        .unwrap_or(1.0)
}

pub fn collect(pid: u32) -> io::Result<ProcessMetrics> {
    let stat = read_proc_stat(pid)?;
    let name = stat
        .get(1)
        .map(|s| s.trim_matches(|c| c == '(' || c == ')').to_string())
        .unwrap_or_default();

    let utime: u64 = stat.get(13).and_then(|v| v.parse().ok()).unwrap_or(0);
    let stime: u64 = stat.get(14).and_then(|v| v.parse().ok()).unwrap_or(0);
    let total_ticks = utime + stime;
    let clk_tck = 100u64;
    // Divide total CPU ticks by clock ticks per second and system uptime to get
    // a percentage of CPU usage over the lifetime of the process.
    let uptime = system_uptime_seconds();
    let cpu_percent = (total_ticks as f64 / clk_tck as f64) / uptime * 100.0;
    let cpu_percent = cpu_percent.min(100.0);

    let memory_bytes = read_proc_status(pid)?;
    let total_mem = total_memory_bytes();
    let memory_percent = (memory_bytes as f64 / total_mem as f64) * 100.0;

    Ok(ProcessMetrics {
        pid,
        name,
        cpu_percent,
        memory_bytes,
        memory_percent,
    })
}
