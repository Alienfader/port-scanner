use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use ipnetwork::IpNetwork;
use std::collections::BTreeSet;
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// High-performance Asynchronous Port Scanner in Rust
#[derive(Parser, Debug)]
#[command(author = "Alienfader", version, about = "High-performance asynchronous port scanner", long_about = None)]
struct Args {
    /// Target IP address, IP range (e.g., "192.168.1.10-192.168.1.20") or CIDR (e.g., "192.168.1.0/24")
    target: String,

    /// Ports to scan. Use comma-separated values (e.g., "22,80,443") or a range (e.g., "20-25"). Defaults to "1-65535".
    #[arg(short, long, default_value = "1-65535")]
    ports: String,

    /// Number of concurrent connections (default: 100)
    #[arg(short, long, default_value_t = 100)]
    concurrency: usize,

    /// Timeout (in seconds) for each connection attempt (default: 3)
    #[arg(short, long, default_value_t = 3)]
    timeout: u64,

    /// Payload to send for banner grabbing (default: "\r\n")
    #[arg(long, default_value = "\r\n")]
    banner_payload: String,
}

/// Parses a ports string into a vector of port numbers.
/// Supports comma-separated lists and ranges (e.g., "20-25").
fn parse_ports(ports_str: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut port_set = BTreeSet::new();
    if ports_str.contains(',') {
        for part in ports_str.split(',') {
            let port: u16 = part.trim().parse()?;
            port_set.insert(port);
        }
    } else if ports_str.contains('-') {
        let parts: Vec<&str> = ports_str.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid port range format".into());
        }
        let start: u16 = parts[0].parse()?;
        let end: u16 = parts[1].parse()?;
        for p in start..=end {
            port_set.insert(p);
        }
    } else {
        let port: u16 = ports_str.trim().parse()?;
        port_set.insert(port);
    }
    Ok(port_set.into_iter().collect())
}

/// Parses the target string into a list of IP addresses.
/// Supports single IP, CIDR notation, or IP range (using a hyphen).
/// Note: IPv6 range scanning via hyphen is not supported.
fn parse_targets(target_str: &str) -> Result<Vec<IpAddr>, Box<dyn Error>> {
    let target_str = target_str.trim();
    if target_str.contains('/') {
        // CIDR notation (supports both IPv4 and IPv6)
        let network: IpNetwork = target_str.parse()?;
        Ok(network.iter().collect())
    } else if target_str.contains('-') {
        // IP range scanning (only supports IPv4)
        let parts: Vec<&str> = target_str.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid IP range format".into());
        }
        let start: IpAddr = parts[0].trim().parse()?;
        let end: IpAddr = parts[1].trim().parse()?;
        match (start, end) {
            (IpAddr::V4(s), IpAddr::V4(e)) => {
                let start_u32 = u32::from(s);
                let end_u32 = u32::from(e);
                if end_u32 < start_u32 {
                    return Err("End IP is lower than start IP".into());
                }
                let mut ips = Vec::new();
                for ip_u32 in start_u32..=end_u32 {
                    ips.push(IpAddr::V4(ip_u32.into()));
                }
                Ok(ips)
            }
            _ => Err("IPv6 range scanning with hyphen is not supported".into()),
        }
    } else {
        // Single IP address
        let ip: IpAddr = target_str.parse()?;
        Ok(vec![ip])
    }
}

/// Attempts to connect to the given target and port.
/// If successful, sends a custom banner payload and reads a response.
async fn scan_port(
    target: IpAddr,
    port: u16,
    timeout_duration: Duration,
    banner_payload: &str,
) -> Option<(u16, Option<String>)> {
    let socket = SocketAddr::new(target, port);
    match timeout(timeout_duration, TcpStream::connect(&socket)).await {
        Ok(Ok(mut stream)) => {
            // Connection succeeded; port is open.
            let _ = stream.write_all(banner_payload.as_bytes()).await;
            let mut buf = vec![0; 1024];
            let banner = match timeout(timeout_duration, stream.read(&mut buf)).await {
                Ok(Ok(n)) if n > 0 => Some(String::from_utf8_lossy(&buf[..n]).to_string()),
                _ => None,
            };
            Some((port, banner))
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // DISCLAIMER: This tool is intended solely for authorized security testing and educational purposes.
    // Unauthorized scanning of networks may be illegal.
    let args = Args::parse();

    let targets = parse_targets(&args.target)?;
    let ports = parse_ports(&args.ports)?;
    let timeout_duration = Duration::from_secs(args.timeout);

    println!(
        "Starting scan for {} target(s) and {} port(s)...",
        targets.len(),
        ports.len()
    );
    println!("Disclaimer: Use this tool only for authorized security testing.\n");

    for target in targets {
        println!("Scanning target: {}", target);
        let semaphore = Arc::new(Semaphore::new(args.concurrency));
        let mut futures = FuturesUnordered::new();
        for &port in &ports {
            let sem = semaphore.clone();
            let t = target;
            let banner_payload = args.banner_payload.clone();
            futures.push(tokio::spawn(async move {
                let _permit = sem.acquire().await;
                scan_port(t, port, timeout_duration, &banner_payload).await
            }));
        }
        while let Some(res) = futures.next().await {
            if let Ok(Some((port, banner))) = res {
                if let Some(b) = banner {
                    println!("Port {} is open - Banner: {}", port, b.trim());
                } else {
                    println!("Port {} is open", port);
                }
                }
        }
    }
    println!("\nScan complete.");
    Ok(())
}
