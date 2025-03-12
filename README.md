
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

---

You can save the above text as `README.md` in your project directory.
