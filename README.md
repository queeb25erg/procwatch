# procwatch

Lightweight daemon that monitors process resource usage and sends alerts via webhook.

## Installation

```bash
cargo install procwatch
```

Or build from source:

```bash
git clone https://github.com/yourusername/procwatch && cd procwatch && cargo build --release
```

## Usage

Create a configuration file `procwatch.toml`:

```toml
[monitor]
process = "nginx"
cpu_threshold = 80.0
memory_threshold = 512

[webhook]
url = "https://hooks.example.com/alerts"
```

Start the daemon:

```bash
procwatch --config procwatch.toml
```

procwatch will poll the target process at a regular interval and POST a JSON alert payload to the configured webhook URL whenever a threshold is exceeded.

### Alert Payload Example

```json
{
  "process": "nginx",
  "pid": 1234,
  "cpu_percent": 85.3,
  "memory_mb": 490,
  "triggered_at": "2024-06-01T12:00:00Z"
}
```

## Flags

| Flag | Description |
|------|-------------|
| `--config <path>` | Path to config file (default: `./procwatch.toml`) |
| `--interval <secs>` | Poll interval in seconds (default: `5`) |
| `--verbose` | Enable verbose logging |

## License

MIT © 2024 Your Name