use std::env;
use std::process::Command;
use std::str;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    cpu_socket_count: String,
    cpu_core_count: String,
    cpu_thread_count: String,
}

impl CpuInfo {
    pub fn new() -> CpuInfo {
        // Get CPU information
        let (cpu_socket_count, cpu_core_count, cpu_thread_count) = get_cpu_info();

        CpuInfo {
            cpu_socket_count,
            cpu_core_count,
            cpu_thread_count,
        }
    }

    pub fn get_cpu_socket_count(&self) -> &str {
        &self.cpu_socket_count
    }

    pub fn get_cpu_core_count(&self) -> &str {
        &self.cpu_core_count
    }

    pub fn get_cpu_thread_count(&self) -> &str {
        &self.cpu_thread_count
    }

    pub fn display(&self) {
        println!("CPU Socket Count: {}", self.get_cpu_socket_count().green());
        println!("CPU Core Count: {}", self.get_cpu_core_count().green());
        println!("CPU Thread Count: {}", self.get_cpu_thread_count().green());
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }
}

fn get_cpu_info() -> (String, String, String) {
    match env::consts::OS {
        "windows" => {
            let output = Command::new("wmic")
                .args(&["cpu", "get", "NumberOfCores,NumberOfLogicalProcessors"])
                .output()
                .expect("Failed to execute wmic command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            let mut core_count = 0;
            let mut thread_count = 0;

            for line in lines {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 3 {
                    if let Ok(count) = parts[1].parse::<usize>() {
                        core_count += count;
                    }
                    if let Ok(count) = parts[2].parse::<usize>() {
                        thread_count += count;
                    }
                }
            }

            let socket_count = 1; // Assuming one socket on Windows

            (socket_count.to_string(), core_count.to_string(), thread_count.to_string())
        }

        "linux" => {
            let output = Command::new("lscpu").output().expect("Failed to execute lscpu command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            let mut socket_set = std::collections::HashSet::new();
            let mut core_count = 0;
            let mut thread_count = 0;

            for line in lines {
                let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "Socket(s)" => {
                            if let Ok(count) = parts[1].parse::<usize>() {
                                socket_set.insert(count);
                            }
                        }
                        "Core(s) per socket" => {
                            if let Ok(count) = parts[1].parse::<usize>() {
                                core_count = count;
                            }
                        }
                        "Thread(s) per core" => {
                            if let Ok(count) = parts[1].parse::<usize>() {
                                thread_count = count;
                            }
                        }
                        _ => {}
                    }
                }
            }

            let socket_count = socket_set.len();
            let core_count = core_count * socket_count;
            let thread_count = thread_count * core_count;

            (socket_count.to_string(), core_count.to_string(), thread_count.to_string())
        }

        "macos" => {
            let output = Command::new("sysctl")
                .args(&["-n", "machdep.cpu.core_count", "machdep.cpu.thread_count"])
                .output()
                .expect("Failed to execute sysctl command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            let mut core_count = 0;
            let mut thread_count = 0;

            for line in lines {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    if parts[0] == "machdep.cpu.core_count:" {
                        if let Ok(count) = parts[1].parse::<usize>() {
                            core_count = count;
                        }
                    } else if parts[0] == "machdep.cpu.thread_count:" {
                        if let Ok(count) = parts[1].parse::<usize>() {
                            thread_count = count;
                        }
                    }
                }
            }

            let socket_count = 1; // Assuming one socket on macOS

            (socket_count.to_string(), core_count.to_string(), thread_count.to_string())
        }

        _ => {
            (
                String::new(),
                String::new(),
                String::new()
            )
        }
    }
}