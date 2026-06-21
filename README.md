# NetSentinel

NetSentinel is a high-performance network traffic monitor built with Rust. It captures and audits TCP traffic in real-time, logging detailed packet metadata (Source/Destination IP, Ports, Flags, TTL, Window Size) to a JSONL file for security analysis and monitoring.

# Features

Real-time Monitoring: Captures live TCP traffic directly from the network interface.

Security Auditing: Logs critical packet metadata for security analysis.

Lightweight & Fast: Built with Rust and Tokio for high concurrency and low resource usage.

Structured Logging: Exports data in JSONL (JSON Lines) format, making it easy to parse with log analysis tools.

Filtering: Built-in filtering for target IP and specific ports (e.g., HTTP/HTTPS).

# Installation

Prerequisites

Rust (latest stable version)

libpcap development headers (required for pnet)

Ubuntu/Debian:

```bash
sudo apt update
sudo apt install libpcap-dev
```

Build

Clone the repository:

```bash
git clone https://github.com/AkhrorKhasanov/net_sentinel.git
cd net_sentinel
```

Build the project:

```bash
cargo build --release
```

# Usage
You must run the executable with sudo (root privileges) to capture network packets.

Basic usage
```bash
sudo ./target/release/net_sentinel --ip <YOUR_SERVER_IP>
```

Systemd Integration (Recommended)

To run NetSentinel as a background service:

Create a service file: /etc/systemd/system/net_sentinel.service

Add the following content:

Ini, TOML
[Unit]
Description=Net Sentinel Network Monitor
After=network.target

[Service]
ExecStart=/path/to/your/net_sentinel --ip 143.198.230.98
Restart=always
User=root

[Install]
WantedBy=multi-user.target
Enable and start:

```bash
sudo systemctl enable net_sentinel
sudo systemctl start net_sentinel
```

# Log Analysis

The logs are saved to logs/audit.jsonl. You can view them in real-time:

```bash
tail -f logs/audit.jsonl
```
Contributing
Contributions are welcome! Please feel free to submit a Pull Request or open an issue for any bugs or feature requests.
