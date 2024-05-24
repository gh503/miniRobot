use std::env;
use std::process::Command;
use std::str;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemInfo {
    memory_total: String,
    memory_using: String,
    swap_total: String,
    swap_using: String,
}

impl MemInfo {
    pub fn new() -> MemInfo {

        // Get memory information
        let (memory_total, memory_using, swap_total, swap_using) = get_memory_info();

        MemInfo {
            memory_total,
            memory_using,
            swap_total,
            swap_using,
        }
    }

    pub fn get_memory_total(&self) -> &str {
        &self.memory_total
    }

    pub fn get_memory_using(&self) -> &str {
        &self.memory_using
    }

    pub fn get_swap_total(&self) -> &str {
        &self.swap_total
    }

    pub fn get_swap_using(&self) -> &str {
        &self.swap_using
    }

    pub fn display(&self) {
        println!("Memory Total: {}", self.get_memory_total().green());
        println!("Memory Using: {}", self.get_memory_using().green());
        println!("Swap Total: {}", self.get_swap_total().green());
        println!("Swap Using: {}", self.get_swap_using().green());
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }
}

fn get_memory_info() -> (String, String, String, String) {
    match env::consts::OS {
        "windows" => {
            let output = Command::new("wmic")
                .args(&["OS", "get", "TotalVisibleMemorySize,FreePhysicalMemory,TotalVirtualMemorySize,FreeVirtualMemory"])
                .output()
                .expect("Failed to execute wmic command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            let mut mem_total = 0;
            let mut mem_free = 0;
            let mut swap_total = 0;
            let mut swap_free = 0;

            for line in lines {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "TotalVisibleMemorySize" => mem_total = parts[1].parse().unwrap_or(0),
                        "FreePhysicalMemory" => mem_free = parts[1].parse().unwrap_or(0),
                        "TotalVirtualMemorySize" => swap_total = parts[1].parse().unwrap_or(0),
                        "FreeVirtualMemory" => swap_free = parts[1].parse().unwrap_or(0),
                        _ => {}
                    }
                }
            }
        
            let mem_using = mem_total - mem_free;
            let swap_using = swap_total - swap_free;
        
            (
                format!("{} MBytes", mem_total / 1024),
                format!("{} MBytes", mem_using / 1024),
                format!("{} MBytes", swap_total / 1024),
                format!("{} MBytes", swap_using / 1024)
            )
        }

        "linux" => {
            let output = Command::new("free")
                .args(&["--mega"])
                .output()
                .expect("Failed to execute free command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            let mut mem_total = 0;
            let mut mem_using = 0;
            let mut swap_total = 0;
            let mut swap_using = 0;

            for line in lines {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    match parts[0] {
                        "Mem:" => {
                            mem_total = parts[1].parse().unwrap_or(0);
                            mem_using = parts[2].parse().unwrap_or(0);
                        }
                        "Swap:" => {
                            swap_total = parts[1].parse().unwrap_or(0);
                            swap_using = parts[2].parse().unwrap_or(0);
                        }
                        _ => {}
                    }
                }
            }

            (
                format!("{} MBytes", mem_total),
                format!("{} MBytes", mem_using),
                format!("{} MBytes", swap_total),
                format!("{} MBytes", swap_using)
            )
        }

        "macos" => {
            let output = Command::new("sysctl")
                .args(&["-n", "hw.memsize", "vm.swapusage"])
                .output()
                .expect("Failed to execute sysctl command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            let mut mem_total = 0;
            let mem_using = 0;
            let mut swap_total = 0;
            let mut swap_using = 0;

            for line in lines {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    match parts[0] {
                        "hw.memsize:" => mem_total = parts[1].parse().unwrap_or(0),
                        "vm.swapusage:" => {
                            swap_total = parts[1].parse().unwrap_or(0);
                            swap_using = parts[2].parse().unwrap_or(0);
                        }
                        _ => {}
                    }
                }
            }

            (
                format!("{} MBytes", mem_total),
                format!("{} MBytes", mem_using),
                format!("{} MBytes", swap_total),
                format!("{} MBytes", swap_using)
            )
        }

        _ => {
            (
                String::new(),
                String::new(),
                String::new(),
                String::new()
            )
        }
    }
}
