mod config;

use config::Config;
use std::path::PathBuf;
use std::process;

fn default_config_path() -> PathBuf {
    PathBuf::from("/etc/procwatch/config.toml")
}

fn main() {
    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(default_config_path);

    let config = match Config::from_file(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[procwatch] Failed to load config from {:?}: {}", config_path, e);
            process::exit(1);
        }
    };

    println!("[procwatch] Starting with poll interval {}s", config.poll_interval_secs);
    println!("[procwatch] Webhook: {}", config.webhook_url);
    println!("[procwatch] Monitoring {} process(es)", config.processes.len());

    for proc in &config.processes {
        println!(
            "[procwatch]   - {} (cpu>{:.0}%, mem>{}MB)",
            proc.name, proc.cpu_threshold_pct, proc.mem_threshold_mb
        );
    }

    // Main loop placeholder — monitoring logic will be added in subsequent modules.
    println!("[procwatch] Daemon running. Press Ctrl+C to stop.");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(config.poll_interval_secs));
    }
}
