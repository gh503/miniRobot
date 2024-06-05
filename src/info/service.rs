use std::process::Command;
use std::str;

use colored::Colorize;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Service {
    ip_address: String,      // IP地址
    port: u16,               // 端口号
    protocol: String,        // 协议
    status: String,          // 服务状态
    pid: u32,                // 进程号
    remote_address: String,  // 对端地址
}

impl Service {
    pub fn new(ip_address: &str, port: &u16, protocol: &str, status: &str, pid: &u32, remote_address: &str) -> Self {
        Service {
            ip_address: ip_address.to_string(),
            port: *port,
            protocol: protocol.to_string(),
            status: status.to_string(),
            pid: *pid,
            remote_address: remote_address.to_string(),
        }
    }

    // 获取 IP 地址
    pub fn ip_address(&self) -> &str {
        &self.ip_address
    }

    // 获取端口号
    pub fn port(&self) -> u16 {
        self.port
    }

    // 获取协议
    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    // 获取服务状态
    pub fn status(&self) -> &str {
        &self.status
    }

    // 获取进程号
    pub fn pid(&self) -> u32 {
        self.pid
    }

    // 获取对端地址
    pub fn remote_address(&self) -> &str {
        &self.remote_address
    }

    pub fn display(&self) -> String {
        let output;
        let l_net_info = format!("{}:{} / {}", &self.ip_address, &self.port, &self.protocol);
        if self.pid.eq(&0u32) {
            output = format!("[PID] {:<6}  [L] {:<35}  [R] {:<35}  [STATUS] {:<15}",
                &self.pid, &l_net_info.green(), &self.remote_address.yellow(), &self.status.blue());
        } else {
            output = format!("[PID] {:<6}  [L] {:<35}  [R] {:<35}  [STATUS] {:<15}",
                &self.pid.to_string().bold(), &l_net_info.green(), &self.remote_address.yellow(), &self.status.blue());
        }
        print!("{}\n", output);
        output
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| {
            eprintln!("序列化 Service 信息到 JSON 失败: {}", err);
            "{}".to_string()
        })
    }
}

#[cfg(target_os = "windows")]
pub fn get_services() -> Vec<Service> {
    let output = Command::new("netstat")
        .arg("-ano")
        .output()
        .expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut services = Vec::new();

    for line in output_str.lines().skip(4) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let local_address = parts[1].to_string();
        let remote_address = parts[2].to_string();
        if local_address.starts_with("127.0.0.") || local_address.starts_with("[::1]")
            || remote_address.starts_with("127.0.0.") || remote_address.starts_with("[::1]") {
            continue;
        }
        let protocol = if parts[0].starts_with("TCP") { "tcp" } else { "udp" }.to_string();

        let status = String::new();
        let pid = 0u32;
        if parts.len() >= 5 {
            status = parts[3].to_string();
            pid = parts[4].parse().unwrap_or(0u32);
        } else {
            pid = parts[3].parse().unwrap_or(0u32)
        }

        let local_parts: Vec<&str> = local_address.rsplitn(2, ':').collect();
        if local_parts.len() == 2 {
            let ip_address = local_parts[1].trim_start_matches('[').trim_end_matches(']').to_string();
            let port: u16 = local_parts[0].parse().expect("Invalid port");
            services.push(Service::new(
                ip_address.as_str(),
                &port,
                protocol.as_str(),
                status.as_str(),
                &pid,
                remote_address.as_str(),
            ));
        }
    }

    services
}

#[cfg(target_os = "linux")]
pub fn get_services() -> Vec<Service> {
    let output = Command::new("netstat")
        .arg("-tupn")
        .arg("--all")
        .output()
        .expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut services = Vec::new();

    for line in output_str.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 7 {
            let local_address = parts[3].to_string();
            let remote_address = parts[4].to_string();
            if local_address.starts_with("127.0.0.") || local_address.starts_with("::1:") ||
                remote_address.starts_with("127.0.0.") || remote_address.starts_with("::1:") {
                continue;
            }

            let protocol = parts[0].to_string();
            let test_str = parts[5].to_string();
            let mut status = String::new();
            let pid;
            if test_str.contains('/') {
                pid = parts[5].split('/').next().unwrap_or("0").parse().unwrap_or(0u32);
            } else {
                status = parts[5].to_string();
                pid = parts[6].split('/').next().unwrap_or("0").parse().unwrap_or(0u32);
            }
            let local_parts: Vec<&str> = local_address.rsplitn(2, ':').collect();
            if local_parts.len() == 2 {
                let ip_address = local_parts[1].to_string();
                let port = local_parts[0].parse().expect("Invalid port!");
                services.push(Service::new(
                    ip_address.as_str(),
                    &port,
                    protocol.as_str(),
                    status.as_str(),
                    &pid,
                    remote_address.as_str(),
                ))
            }
        }
    }

    services
}

#[cfg(target_os = "macos")]
pub fn get_services() -> Vec<Service> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg("-n")
        .arg("-P")
        .output()
        .expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut services = Vec::new();

    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 9 {
            let pid = parts[1].parse().unwrap_or(0);
            let protocol = parts[7].to_string();
            let addresses = parts[8];

            let mut local_address = String::new();
            let mut remote_address = String::new();

            if let Some((local, remote)) = addresses.split_once("->") {
                local_address = local.to_string();
                remote_address = remote.to_string();
            } else {
                local_address = addresses.to_string();
            }
            if local_address.starts_with("127.0.0.") || local_address.starts_with("::1:")
                || remote_address.starts_with("127.0.0.") || remote_address.starts_with("::1:") {
                continue;
            }

            let local_parts: Vec<&str> = local_address.split(':').collect();
            if local_parts.len() == 2 {
                let ip_address = local_parts[0].to_string();
                let port: u16 = local_parts[1].parse().expect("Invalid port");
                services.push(Service {
                    ip_address,
                    port,
                    protocol,
                    status: String::new(), // `lsof` doesn't provide status
                    pid,
                    remote_address,
                });
            }
        }
    }

    services
}
