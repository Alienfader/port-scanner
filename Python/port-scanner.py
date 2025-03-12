#!/usr/bin/env python3
"""
Simple Asynchronous Port Scanner

Usage:
    python3 port_scanner.py TARGET [-p PORTS] [--concurrency N]

Examples:
    # Scan all ports on a single IP (default: 1-65535):
    python3 port_scanner.py 192.168.1.10

    # Scan ports 20 to 25 on a single IP:
    python3 port_scanner.py 192.168.1.10 -p 20-25

    # Scan ports 22, 80, and 443 on a CIDR block:
    python3 port_scanner.py 192.168.1.0/24 -p 22,80,443

Dependencies:
    - Python 3.7+
    - No external libraries are required

DISCLAIMER:
    This tool is intended solely for authorized security testing and educational purposes.
    Unauthorized scanning of networks may be illegal.
"""

import asyncio
import argparse
import ipaddress

async def scan_port(ip, port, semaphore, timeout=3):
    """
    Attempts to connect to a specified port on the target IP.
    Returns a tuple: (port, is_open, banner)
    """
    open_port = False
    banner = None
    try:
        async with semaphore:
            reader, writer = await asyncio.wait_for(
                asyncio.open_connection(ip, port), timeout=timeout
            )
            open_port = True
            # Attempt a simple banner grab by sending a newline
            try:
                writer.write(b'\r\n')
                await writer.drain()
                banner_data = await asyncio.wait_for(reader.read(1024), timeout=timeout)
                if banner_data:
                    banner = banner_data.decode(errors='replace').strip()
            except Exception:
                banner = None

            writer.close()
            await writer.wait_closed()
    except Exception:
        pass
    return port, open_port, banner

async def scan_target(ip, ports, concurrency):
    """
    Scans a list of ports on a single target IP concurrently.
    Returns a list of tuples with open port numbers and any banner data.
    """
    semaphore = asyncio.Semaphore(concurrency)
    tasks = [scan_port(ip, port, semaphore) for port in ports]
    results = await asyncio.gather(*tasks)
    return [(port, banner) for port, open_port, banner in results if open_port]

def parse_ports(ports_str):
    """
    Parses the port input string.
    Supports comma-separated values (e.g., "22,80,443") and ranges (e.g., "20-25").
    """
    ports = set()
    for part in ports_str.split(','):
        part = part.strip()
        if '-' in part:
            start, end = part.split('-')
            ports.update(range(int(start), int(end) + 1))
        else:
            ports.add(int(part))
    return sorted(ports)

def parse_targets(target_str):
    """
    Parses the target input.
    Supports:
      - A single IP address (e.g., "192.168.1.10")
      - An IP range with a hyphen (e.g., "192.168.1.10-192.168.1.20")
      - CIDR notation (e.g., "192.168.1.0/24")
    Returns a list of IP addresses as strings.
    """
    targets = []
    target_str = target_str.strip()
    if '/' in target_str:
        try:
            net = ipaddress.ip_network(target_str, strict=False)
            targets = [str(ip) for ip in net.hosts()]
        except ValueError:
            print("Invalid CIDR notation provided.")
    elif '-' in target_str:
        start_str, end_str = target_str.split('-')
        try:
            start_ip = ipaddress.ip_address(start_str.strip())
            end_ip = ipaddress.ip_address(end_str.strip())
            if int(end_ip) < int(start_ip):
                print("Invalid IP range: End IP is lower than Start IP.")
            else:
                for ip_int in range(int(start_ip), int(end_ip) + 1):
                    targets.append(str(ipaddress.ip_address(ip_int)))
        except ValueError:
            print("Invalid IP address range provided.")
    else:
        try:
            ipaddress.ip_address(target_str)
            targets = [target_str]
        except ValueError:
            print("Invalid IP address provided.")
    return targets

async def main():
    parser = argparse.ArgumentParser(description="Simple Asynchronous Port Scanner Tool")
    parser.add_argument("target", help="Target IP address, IP range (e.g., 192.168.1.10-192.168.1.20) or CIDR (e.g., 192.168.1.0/24)")
    parser.add_argument("-p", "--ports", default="1-65535",
                        help="Optional: Ports to scan. Use comma-separated values (e.g., '22,80,443') or a range (e.g., '20-25'). Default is '1-65535'.")
    parser.add_argument("--concurrency", type=int, default=100, help="Number of concurrent connections (default: 100)")
    args = parser.parse_args()
    
    ports = parse_ports(args.ports)
    targets = parse_targets(args.target)
    
    if not targets:
        print("No valid targets to scan.")
        return

    for target in targets:
        print(f"\nScanning target: {target} ({len(ports)} ports). This may take a while...")
        open_ports = await scan_target(target, ports, args.concurrency)
        if open_ports:
            print("Open ports:")
            for port, banner in open_ports:
                msg = f"Port {port} is open"
                if banner:
                    msg += f" - Banner: {banner}"
                print(msg)
        else:
            print("No open ports found.")
    print("Scan complete.")

if __name__ == "__main__":
    # DISCLAIMER: This tool is intended solely for authorized security testing and educational purposes.
    # Unauthorized scanning of networks may be illegal.
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nScan interrupted by user.")
