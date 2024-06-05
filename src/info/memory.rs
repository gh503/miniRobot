use std::process::Command;
use std::str;

use colored::Colorize;
use serde::Serialize;
use serde_json;

use crate::common::api::format_size;

#[derive(Debug, Serialize)]
pub struct MemInfo {
    memory_total: String,
    memory_using: String,
    memory_free: String,
    swap_total: String,
    swap_using: String,
    swap_free: String
}

impl MemInfo {
    pub fn new() -> MemInfo {

        // Get memory information
        let (memory_total, memory_using, memory_free,
             swap_total, swap_using, swap_free) = get_memory_info();

        MemInfo {
            memory_total,
            memory_using,
            memory_free,
            swap_total,
            swap_using,
            swap_free
        }
    }

    pub fn memory_total(&self) -> &str {
        &self.memory_total
    }

    pub fn memory_using(&self) -> &str {
        &self.memory_using
    }

    pub fn memory_free(&self) -> &str {
        &self.memory_free
    }

    pub fn swap_total(&self) -> &str {
        &self.swap_total
    }

    pub fn swap_using(&self) -> &str {
        &self.swap_using
    }

    pub fn swap_free(&self) -> &str {
        &self.swap_free
    }

    pub fn display(&self) -> String {
        let output = format!("Memory:\n  {} free {} using {} total\n  swap {} free {} using {} total",
            &format_size(self.memory_free.parse::<u64>().unwrap()).green().to_string(),
            &format_size(self.memory_using.parse::<u64>().unwrap()).yellow().to_string(),
            &format_size(self.memory_total.parse::<u64>().unwrap()).blue().to_string(),
            &format_size(self.swap_free.parse::<u64>().unwrap()).green().to_string(),
            &format_size(self.swap_using.parse::<u64>().unwrap()).yellow().to_string(),
            &format_size(self.swap_total.parse::<u64>().unwrap()).blue().to_string(),
        );
        println!("{}\n", output);
        output
    }

    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| {
            eprintln!("序列化 Memory 信息到 JSON 失败: {}", err);
            "{}".to_string()
        })
    }
}

#[cfg(target_os = "windows")]
fn get_memory_info() -> (String, String, String, String, String, String) {
    let mut mem_total = 0u128;
    let mut mem_using = 0u128;
    let mut mem_free = 0u128;
    let mut swap_total = 0u128;
    let mut swap_using = 0u128;
    let mut swap_free = 0u128;
    // 获取物理内存信息
    let physical_memory_output = Command::new("wmic")
        .arg("OS")
        .arg("get")
        .arg("TotalVisibleMemorySize,FreePhysicalMemory")
        .arg("/format:csv")
        .output()
        .expect("Failed to execute command");
    let physical_memory_info = String::from_utf8_lossy(&physical_memory_output.stdout);
    for line in physical_memory_info.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            mem_total = parts[1].trim().parse::<u64>().unwrap_or(0);
            mem_free = parts[2].trim().parse::<u64>().unwrap_or(0);
            mem_using = mem_total - mem_free;
        }
    }

    let swap_memory_output = Command::new("wmic")
        .arg("pagefile")
        .arg("list")
        .arg("full")
        .output()
        .expect("Failed to execute command");
    let swap_memory_info = String::from_utf8_lossy(&swap_memory_output.stdout);
    for line in swap_memory_info.lines() {
        if line.starts_with("AllocatedBaseSize=") {
            let value = line.split('=').nth(1).unwrap().trim().parse::<u64>().unwrap_or(0);
            swap_total += value * 1024 * 1024; // 转换为Bytes
        } else if line.starts_with("CurrentUsage=") {
            let value = line.split('=').nth(1).unwrap().trim().parse::<u64>().unwrap_or(0);
            swap_using += value * 1024 * 1024; // 转换为Bytes
        }
    }
    swap_free = swap_total - swap_using;
    (
        mem_total.to_string(),
        mem_using.to_string(),
        mem_free.to_string(),
        swap_total.to_string(),
        swap_using.to_string(),
        swap_free.to_string()
    )
}

#[cfg(target_os = "linux")]
fn get_memory_info() -> (String, String, String, String, String, String) {
    let mut mem_total = 0u128;
    let mut mem_using = 0u128;
    let mut mem_free = 0u128;
    let mut swap_total = 0u128;
    let mut swap_using = 0u128;
    let mut swap_free = 0u128;
    let output = Command::new("free")
        .arg("--bytes")
        .output()
        .expect("Failed to execute free command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    for line in lines {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            match parts[0] {
                "Mem:" => {
                    mem_total = parts[1].parse().unwrap_or(0);
                    mem_using = parts[2].parse().unwrap_or(0);
                    mem_free = parts[3].parse().unwrap_or(0);
                }
                "Swap:" => {
                    swap_total = parts[1].parse().unwrap_or(0);
                    swap_using = parts[2].parse().unwrap_or(0);
                    swap_free = parts[3].parse().unwrap_or(0);
                }
                _ => {}
            }
        }
    }
    (
        mem_total.to_string(),
        mem_using.to_string(),
        mem_free.to_string(),
        swap_total.to_string(),
        swap_using.to_string(),
        swap_free.to_string()
    )
}

#[cfg(target_os = "macos")]
fn get_memory_info() -> (String, String, String, String, String, String) {
    let mut mem_total = 0u128;
    let mut mem_using = 0u128;
    let mut mem_free = 0u128;
    let mut swap_total = 0u128;
    let mut swap_using = 0u128;
    let mut swap_free = 0u128;
    // 获取物理内存信息
    let physical_memory_output = Command::new("sysctl")
        .arg("hw.memsize")
        .output()
        .expect("Failed to execute command");
    let physical_memory_info = String::from_utf8_lossy(&physical_memory_output.stdout);
    mem_total = physical_memory_info
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    // 获取vm_stat信息
    let vm_stat_output = Command::new("vm_stat")
        .output()
        .expect("Failed to execute command");
    let vm_stat_info = String::from_utf8_lossy(&vm_stat_output.stdout);

    let mut page_size = 0u128;
    let mut free_pages = 0u128;
    let mut active_pages = 0u128;
    let mut speculative_pages = 0u128;
    let mut inactive_pages = 0u128;
    let mut wired_pages = 0u128;

    for line in vm_stat_info.lines() {
        if line.starts_with("Mach Virtual Memory Statistics") {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let key = parts[0].trim();
        let value = parts[1].trim().trim_end_matches('.').replace(".", "").parse::<u64>().unwrap();
        match key {
            "Pages free" => free_pages = value,
            "Pages active" => active_pages = value,
            "Pages speculative" => speculative_pages = value,
            "Pages inactive" => inactive_pages = value,
            "Pages wired down" => wired_pages = value,
            "Page size of" => page_size = value,
            _ => {},
        }
    }

    mem_free = free_pages * page_size;
    mem_using = (active_pages + speculative_pages + inactive_pages + wired_pages) * page_size;

    // 获取交换空间信息
    let swap_info_output = Command::new("sysctl")
        .arg("vm.swapusage")
        .output()
        .expect("Failed to execute command");

    let swap_info = String::from_utf8_lossy(&swap_info_output.stdout);
    let swap_parts: Vec<&str> = swap_info.split(',').collect();
    swap_total = swap_parts[0].split_whitespace().nth(2).unwrap().parse::<u64>().unwrap() * 1024 * 1024;
    swap_using = swap_parts[1].split_whitespace().nth(2).unwrap().parse::<u64>().unwrap() * 1024 * 1024;
    swap_free = swap_parts[2].split_whitespace().nth(2).unwrap().parse::<u64>().unwrap() * 1024 * 1024;
    (
        mem_total.to_string(),
        mem_using.to_string(),
        mem_free.to_string(),
        swap_total.to_string(),
        swap_using.to_string(),
        swap_free.to_string()
    )
}

#[cfg(test)]
mod unit_test_memory {
    use super::*;

    #[test]
    fn test_memory_info_01() {
        let mem_info = MemInfo::new();
        println!("calling fn display:");
        let output = mem_info.display();
        assert!(output.contains("Memory:") && output.contains("swap"), "failed since some memory info not exists")
    }

    #[test]
    fn test_memory_info_02() {
        let mem_info = MemInfo::new();
        println!("calling fn to_json:");
        let json_output = mem_info.to_json();
        assert!(json_output.contains("{")
            && json_output.contains("memory_total")
            && json_output.contains("memory_using")
            && json_output.contains("memory_free")
            && json_output.contains("swap_total")
            && json_output.contains("swap_using")
            && json_output.contains("swap_free"),
            "memory info json some field missing"
        )
    }

    #[test]
    fn test_memory_info_03() {
        let mem_info = MemInfo::new();
        println!("calling fn memory_total");
        assert!(mem_info.memory_total().ne("0 bytes"), "physical memory total is 0");
    }

    #[test]
    fn test_memory_info_04() {
        let mem_info = MemInfo::new();
        println!("calling fn memory_using");
        assert!(mem_info.memory_using().ne("0 bytes"), "physical memory using is 0");
    }

    #[test]
    fn test_memory_info_05() {
        let mem_info = MemInfo::new();
        println!("calling fn memory_free");
        assert!(mem_info.memory_free().ne("0 bytes"), "physical memory free is 0");
    }

    #[test]
    fn test_memory_info_06() {
        let mem_info = MemInfo::new();
        println!("calling fn swap_total");
        assert!(mem_info.swap_total().len() > 0, "swap memory total empty");
    }

    #[test]
    fn test_memory_info_07() {
        let mem_info = MemInfo::new();
        println!("calling fn swap_using");
        assert!(mem_info.swap_using().len() > 0, "swap memory using empty");
    }

    #[test]
    fn test_memory_info_08() {
        let mem_info = MemInfo::new();
        println!("calling fn swap_free");
        assert!(mem_info.swap_free().len() > 0, "swap memory free empty");
    }
}
