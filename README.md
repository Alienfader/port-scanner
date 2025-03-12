
```markdown
# Alienfader's Port Scanners

This repository contains two implementations of a simple asynchronous port scanner designed for authorized security testing and educational purposes. Both tools scan TCP ports on a specified target and attempt basic banner grabbing. They are lightweight, customizable, and serve as a learning resource for network scanning techniques.

> **DISCLAIMER:**  
> These tools are intended solely for authorized security testing and educational purposes. Unauthorized scanning of networks is illegal and may result in severe consequences. Use them responsibly.

## Table of Contents

- [Overview](#overview)
- [Bash Port Scanner](#bash-port-scanner)
  - [Features](#features)
  - [Requirements](#requirements)
  - [Installation and Usage](#installation-and-usage)
- [Rust Port Scanner](#rust-port-scanner)
  - [Features](#features-1)
  - [Requirements](#requirements-1)
  - [Installation and Usage](#installation-and-usage-1)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)

## Overview

This repository provides two implementations of a port scanner:

1. **Bash Port Scanner:**  
   A simple, lightweight tool written entirely in Bash. It uses `netcat` for port testing and `xargs` for concurrent scanning.
   
2. **Rust Port Scanner:**  
   A high-performance asynchronous scanner built with Rust using the Tokio runtime. It offers configurable options such as custom timeouts, concurrency control, and banner payloads.

---

## Bash Port Scanner

### Features

- **Asynchronous Scanning:** Uses `xargs` with parallel processing to scan multiple ports concurrently.
- **Customizable Port Range:** Defaults to scanning all ports (1–65535), but you can specify a range or comma-separated list.
- **Basic Banner Grabbing:** Attempts to retrieve service banners from open ports.
- **Simple and Lightweight:** Written entirely in Bash with minimal dependencies.

### Requirements

- **Bash** (version 4+ recommended)
- **netcat (nc):** A version that supports the required flags
- **xargs:** Must support the `-P` option for parallel processing
- **Root Privileges:** May be required to scan certain ports

### Installation and Usage

1. **Clone or Download the Repository** and navigate to the directory.
2. **Make the Script Executable:**

   ```bash
   chmod +x port_scanner.sh
   ```

3. **Usage:**

   ```bash
   sudo ./port_scanner.sh TARGET [-p PORTS] [--concurrency N]
   ```

#### Examples

- **Scan all ports on a target (default: 1–65535):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10
  ```

- **Scan a specific port range (e.g., 20-25):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 20-25
  ```

- **Scan specific ports (e.g., 22,80,443):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 22,80,443
  ```

- **Adjust concurrency (e.g., 200 parallel scans):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 20-25 --concurrency 200
  ```

### How It Works

- **Input Parsing:** Accepts a target (IP, IP range, or CIDR) and an optional ports argument.
- **Port Scanning:** Uses `netcat` in zero-I/O mode to test port connectivity.
- **Banner Grabbing:** Sends a newline to open ports and reads any returned data as a banner.
- **Concurrency:** Utilizes `xargs -P` to run port scans concurrently.

---

## Rust Port Scanner

### Features

- **Asynchronous Scanning:** Leverages the Tokio runtime and Futures for high-performance scanning.
- **Configurable Options:**
  - **Port Specification:** Scan all ports (default: 1–65535) or specify a range/list (e.g., `20-25` or `22,80,443`).
  - **Concurrency Control:** Adjust the number of simultaneous connections.
  - **Timeout Configuration:** Set a custom timeout (in seconds) for each connection.
  - **Custom Banner Payload:** Provide a custom payload for banner grabbing.
- **Flexible Target Parsing:** Supports single IP addresses, CIDR notation (IPv4/IPv6), and IPv4 ranges.

### Requirements

- **Rust** (version 1.56+ recommended)  
  [Install from rust-lang.org](https://www.rust-lang.org/tools/install)
- **Cargo:** Rust’s package manager

### Installation and Usage

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/Alienfader/rust_port_scanner.git
   cd rust_port_scanner
   ```

2. **Build the Project in Release Mode:**

   ```bash
   cargo build --release
   ```

   The compiled executable will be located at `./target/release/rust_port_scanner`.

3. **Usage:**

   ```bash
   sudo ./target/release/rust_port_scanner <TARGET> [OPTIONS]
   ```

#### Examples

- **Scan all ports on a target (default: 1–65535):**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10
  ```

- **Scan a specific port range (e.g., 20-25):**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 20-25
  ```

- **Scan specific ports (e.g., 22,80,443):**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 22,80,443
  ```

- **Adjust concurrency and timeout:**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 20-25 --concurrency 200 --timeout 5
  ```

- **Use a custom banner payload (e.g., an HTTP request):**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 80 --banner-payload "GET / HTTP/1.0\r\n\r\n"
  ```

### How It Works

- **Input Parsing:** Converts target strings (IP, CIDR, or IPv4 range) into lists of IP addresses and processes port specifications.
- **Port Scanning:** Establishes TCP connections with configurable timeouts.
- **Banner Grabbing:** Sends a custom payload to open ports and captures responses.
- **Concurrency:** Uses Tokio’s asynchronous concurrency primitives to scan ports in parallel.

---

## Contributing

Contributions, bug reports, and feature suggestions are welcome! Please fork the repository and submit pull requests with clear descriptions of your changes.

## License

This project is licensed for educational and authorized security testing purposes only.

## Author

**Alienfader**
```
