use std::process::Command;
use std::time::Instant;

fn is_host_alive(ip: &str) -> bool {
    let output = Command::new("ping")
        .args(&["-n", "1", "-w", "1000", ip])
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn get_hostname(ip: &str) -> Option<String> {
    let output = Command::new("nbtstat")
        .args(&["-A", ip])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("<00>") && line.contains("UNIQUE") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                return Some(parts[0].to_string());
            }
        }
    }

    None
}


fn main() {
    println!("Scanning 192.168.1.0/24...");

    let start = Instant::now();

    for i in 1..=254 {
        let ip = format!("192.168.1.{}", i);

        if is_host_alive(&ip) {
            let hostname = get_hostname(&ip).unwrap_or_else(|| "unknown".to_string());
            println!("[+] Host found: {} ({})", ip, hostname);
        }
    }

    let duration = Instant::now() - start;
    
    println!("Scan completed in {:.2?}", duration);
}