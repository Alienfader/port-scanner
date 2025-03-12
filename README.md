

```markdown
# Alienfader's Simple Port Scanner

This is a simple asynchronous port scanner written in Bash. The tool scans TCP ports on a specified target using `netcat` (nc) and attempts a basic banner grab from open ports. It is designed for authorized security testing and educational purposes only.

## Features

- **Asynchronous Scanning:** Uses `xargs` with parallel processing to scan multiple ports concurrently.
- **Customizable Port Range:** Defaults to scanning all ports (1–65535), but you can specify a range or a comma-separated list of ports.
- **Basic Banner Grabbing:** Retrieves service banners from open ports when possible.
- **Simple and Lightweight:** Written entirely in Bash with minimal dependencies.

## Disclaimer

**DISCLAIMER:** This tool is intended solely for authorized security testing and educational purposes. Unauthorized scanning of networks is illegal and may result in severe consequences. Use this tool responsibly.

## Requirements

- **Bash** (version 4+ recommended)
- **netcat (nc):** Ensure you have a version that supports the required flags.
- **xargs:** Must support the `-P` option for concurrency.
- **Root Privileges:** Scanning certain ports may require elevated privileges.

## Installation

Clone or download the repository and make the script executable:

```bash
chmod +x port_scanner.sh
```

## Usage

```bash
sudo ./port_scanner.sh TARGET [-p PORTS] [--concurrency N]
```

### Examples

- **Scan all ports on a target (default: 1–65535):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10
  ```

- **Scan a specific port range (e.g., ports 20 to 25):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 20-25
  ```

- **Scan specific ports (e.g., ports 22, 80, and 443):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 22,80,443
  ```

- **Adjust concurrency (e.g., 200 parallel scans):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 20-25 --concurrency 200
  ```

## How It Works

The script performs the following tasks:
1. **Parse Input:**  
   It accepts a target (IP, IP range, or CIDR notation) and an optional ports argument (range or comma-separated list). If no ports are specified, it defaults to scanning ports 1–65535.

2. **Port Scanning:**  
   For each port in the specified list, the script uses `netcat` in zero-I/O mode to test if the port is open. A successful connection indicates that the port is open.

3. **Banner Grabbing:**  
   When a connection is successful, the script sends a newline to the port and reads any returned data as a banner.

4. **Concurrency:**  
   The script leverages `xargs -P` to run scans concurrently, allowing multiple ports to be checked at once.

## Limitations and Potential Improvements

- **Performance:**  
  Scanning all 65,535 ports in Bash may be slower than using dedicated tools like `nmap`.

- **Banner Grabbing:**  
  The current banner grabbing is basic and may not work reliably for all services or protocols.

- **Error Handling:**  
  Further improvements could include better logging and error handling for timeouts or failed scans.

- **Netcat Variations:**  
  Different versions of netcat may require adjustments to the script due to differences in supported flags.

## Author

**Alienfader**

If you have any questions or suggestions, feel free to reach out.

## License

This project is licensed for educational and authorized security testing purposes only.
```


Below is a sample README file in Markdown for the Rust port scanner:

---

```markdown
# Alienfader's Rust Port Scanner

A high-performance asynchronous port scanner written in Rust using the Tokio runtime. This tool scans TCP ports on a specified target, supports scanning port ranges or comma-separated ports, and attempts basic banner grabbing with a customizable payload. It supports single IP addresses, CIDR notation (for both IPv4 and IPv6), and IPv4 ranges (using a hyphen).

> **DISCLAIMER:**  
> This tool is intended solely for authorized security testing and educational purposes. Unauthorized scanning of networks is illegal and may result in severe consequences.

## Features

- **Asynchronous Scanning:** Leverages Tokio and Futures for high performance.
- **Configurable Options:**  
  - **Port Specification:** Scan all ports (default: 1–65535) or specify a range or list (e.g., `20-25` or `22,80,443`).
  - **Concurrency Control:** Adjust the number of simultaneous connections.
  - **Timeout Configuration:** Set a custom timeout (in seconds) for each connection attempt.
  - **Custom Banner Payload:** Provide a custom payload for banner grabbing.
- **Target Parsing:** Supports single IP addresses, CIDR notation (IPv4/IPv6), and IPv4 ranges.

## Requirements

- **Rust** (version 1.56+ recommended) – [Install from rust-lang.org](https://www.rust-lang.org/tools/install)
- **Cargo** – Rust’s package manager

## Installation

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

## Usage

Run the scanner with appropriate privileges (e.g., using `sudo`):

```bash
sudo ./target/release/rust_port_scanner <TARGET> [OPTIONS]
```

### Examples

- **Scan All Ports (Default: 1–65535) on a Target:**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10
  ```

- **Scan a Specific Port Range (e.g., 20-25):**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 20-25
  ```

- **Scan Specific Ports (e.g., 22, 80, 443):**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 22,80,443
  ```

- **Adjust Concurrency and Timeout:**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 20-25 --concurrency 200 --timeout 5
  ```

- **Use a Custom Banner Payload (e.g., HTTP Request):**

  ```bash
  sudo ./target/release/rust_port_scanner 192.168.1.10 -p 80 --banner-payload "GET / HTTP/1.0\r\n\r\n"
  ```


## License

This project is provided for educational and authorized security testing purposes only.

## Author

**Alienfader**
```
=======

---

```markdown
# Alienfader's Simple Port Scanner

This is a simple asynchronous port scanner written in Bash. The tool scans TCP ports on a specified target using `netcat` (nc) and attempts a basic banner grab from open ports. It is designed for authorized security testing and educational purposes only.

## Features

- **Asynchronous Scanning:** Uses `xargs` with parallel processing to scan multiple ports concurrently.
- **Customizable Port Range:** Defaults to scanning all ports (1–65535), but you can specify a range or a comma-separated list of ports.
- **Basic Banner Grabbing:** Retrieves service banners from open ports when possible.
- **Simple and Lightweight:** Written entirely in Bash with minimal dependencies.

## Disclaimer

**DISCLAIMER:** This tool is intended solely for authorized security testing and educational purposes. Unauthorized scanning of networks is illegal and may result in severe consequences. Use this tool responsibly.

## Requirements

- **Bash** (version 4+ recommended)
- **netcat (nc):** Ensure you have a version that supports the required flags.
- **xargs:** Must support the `-P` option for concurrency.
- **Root Privileges:** Scanning certain ports may require elevated privileges.

## Installation

Clone or download the repository and make the script executable:

```bash
chmod +x port_scanner.sh
```

## Usage

```bash
sudo ./port_scanner.sh TARGET [-p PORTS] [--concurrency N]
```

### Examples

- **Scan all ports on a target (default: 1–65535):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10
  ```

- **Scan a specific port range (e.g., ports 20 to 25):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 20-25
  ```

- **Scan specific ports (e.g., ports 22, 80, and 443):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 22,80,443
  ```

- **Adjust concurrency (e.g., 200 parallel scans):**

  ```bash
  sudo ./port_scanner.sh 192.168.1.10 -p 20-25 --concurrency 200
  ```

## How It Works

The script performs the following tasks:
1. **Parse Input:**  
   It accepts a target (IP, IP range, or CIDR notation) and an optional ports argument (range or comma-separated list). If no ports are specified, it defaults to scanning ports 1–65535.

2. **Port Scanning:**  
   For each port in the specified list, the script uses `netcat` in zero-I/O mode to test if the port is open. A successful connection indicates that the port is open.

3. **Banner Grabbing:**  
   When a connection is successful, the script sends a newline to the port and reads any returned data as a banner.

4. **Concurrency:**  
   The script leverages `xargs -P` to run scans concurrently, allowing multiple ports to be checked at once.

## Author

**Alienfader**

If you have any questions or suggestions, feel free to reach out.

## License

This project is licensed for educational and authorized security testing purposes only.
```
>>>>>>> 0d7b91ccc90e8d6b339bf90391ae156c19622301
