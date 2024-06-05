use std::process::Command;
use std::str;

use colored::Colorize;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct CpuInfo {
    cpu_socket_count: String,
    cpu_core_count: String,
    cpu_thread_count: String,
}

impl CpuInfo {
    /// 创建新的 CpuInfo 实例
    pub fn new() -> CpuInfo {
        // 获取 CPU 信息
        let (cpu_socket_count, cpu_core_count, cpu_thread_count) = get_cpu_info();

        CpuInfo {
            cpu_socket_count,
            cpu_core_count,
            cpu_thread_count,
        }
    }

    /// 获取 CPU 插槽数
    pub fn cpu_socket_count(&self) -> &str {
        &self.cpu_socket_count
    }

    /// 获取 CPU 核心数
    pub fn cpu_core_count(&self) -> &str {
        &self.cpu_core_count
    }

    /// 获取 CPU 线程数
    pub fn cpu_thread_count(&self) -> &str {
        &self.cpu_thread_count
    }

    /// 显示 CPU 信息
    pub fn display(&self) -> String {
        let output = format!(
            "CPU:\n  Socket(s): {}\n  Core(s): {}\n  Thread(s): {}",
            self.cpu_socket_count.green().to_string(),
            self.cpu_core_count.green().to_string(),
            self.cpu_thread_count.green().to_string()
        );
        println!("{}\n", output);
        output
    }

    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| {
            eprintln!("序列化 CPU 信息到 JSON 失败: {}", err);
            "{}".to_string()
        })
    }
}

#[cfg(target_os = "windows")]
fn get_cpu_info() -> (String, String, String) {
    let mut cpu_socket_count: u32 = 0;
    let mut cpu_core_count = 0;
    let mut cpu_thread_count = 0;

    // 执行 wmic 命令获取 CPU 信息
    let output = Command::new("wmic")
        .arg("cpu")
        .arg("get")
        .arg("NumberOfCores,NumberOfLogicalProcessors,DeviceID")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines().skip(1) { // 跳过标题行
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            if let Ok(cores) = parts[0].parse::<u32>() {
                cpu_core_count += cores;
            }
            if let Ok(logical_processors) = parts[1].parse::<u32>() {
                cpu_thread_count += logical_processors;
            }
            cpu_socket_count += 1; // 每个 DeviceID 代表一个 CPU
        }
    }

    (
        cpu_socket_count.to_string(),
        cpu_core_count.to_string(),
        cpu_thread_count.to_string(),
    )
}

#[cfg(target_os = "linux")]
fn get_cpu_info() -> (String, String, String) {
    let mut cpu_socket_count: u32 = 0;

    // 执行 lscpu 命令获取 CPU 信息
    let output = Command::new("lscpu")
        .output()
        .expect("Failed to execute lscpu command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    let mut core = 0;
    let mut thread = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            match parts[0] {
                "Socket(s)" => {
                    if let Ok(count) = parts[1].parse::<u32>() {
                        cpu_socket_count = count;
                    }
                }
                "Core(s) per socket" => {
                    if let Ok(count) = parts[1].parse::<u32>() {
                        core = count;
                    }
                }
                "Thread(s) per core" => {
                    if let Ok(count) = parts[1].parse::<u32>() {
                        thread = count;
                    }
                }
                _ => {}
            }
        }
    }

    let cpu_core_count = cpu_socket_count * core;
    let cpu_thread_count = cpu_core_count * thread;

    (
        cpu_socket_count.to_string(),
        cpu_core_count.to_string(),
        cpu_thread_count.to_string(),
    )
}

#[cfg(target_os = "macos")]
fn get_cpu_info() -> (String, String, String) {
    let mut cpu_socket_count: u32 = 0;
    let mut cpu_core_count = 0;
    let mut cpu_thread_count = 0;

    // 执行 sysctl -a 命令获取 CPU 信息
    let output = Command::new("sysctl")
        .arg("-a")
        .output()
        .expect("Failed to execute sysctl command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    let mut physical_cpu_count = None;
    let mut core_count = None;
    let mut thread_count = None;

    // 解析每一行，提取所需的信息
    for line in lines {
        let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            match parts[0] {
                "hw.physicalcpu" => {
                    if let Ok(count) = parts[1].parse::<u32>() {
                        physical_cpu_count = Some(count);
                    }
                }
                "hw.ncpu" => {
                    if let Ok(count) = parts[1].parse::<u32>() {
                        core_count = Some(count);
                    }
                }
                "hw.logicalcpu" => {
                    if let Ok(count) = parts[1].parse::<u32>() {
                        thread_count = Some(count);
                    }
                }
                _ => {}
            }
        }
    }

    // 输出结果
    match (physical_cpu_count, core_count, thread_count) {
        (Some(physical), Some(core), Some(thread)) => {
            cpu_socket_count = physical;
            cpu_core_count = core;
            cpu_thread_count = thread;
        }
        _ => eprintln!("无法找到所有必要的信息"),
    }

    (
        cpu_socket_count.to_string(),
        cpu_core_count.to_string(),
        cpu_thread_count.to_string(),
    )
}

#[cfg(test)]
mod unit_test_cpu {
    use super::*;

    #[test]
    fn test_cpu_info_01() {
        let cpu_info = CpuInfo::new();
        println!("调用 display 方法:");
        let output = cpu_info.display();
        assert!(output.contains("Socket") && output.contains("Core") && output.contains("Thread"));
    }

    #[test]
    fn test_cpu_info_02() {
        let cpu_info = CpuInfo::new();
        println!("调用 to_json 方法:");
        let json_output = cpu_info.to_json();
        assert!(json_output.contains("{")
            && json_output.contains("cpu_socket_count")
            && json_output.contains("cpu_core_count")
            && json_output.contains("cpu_thread_count"));
    }

    #[test]
    fn test_cpu_info_03() {
        let cpu_info = CpuInfo::new();
        println!("调用 cpu_socket_count 方法:");
        let socket = cpu_info.cpu_socket_count();
        assert!(socket.parse::<u64>().unwrap_or(0) >= 1, "CPU 插槽数无效");
    }

    #[test]
    fn test_cpu_info_04() {
        let cpu_info = CpuInfo::new();
        println!("调用 cpu_core_count 方法:");
        let core = cpu_info.cpu_core_count();
        assert!(core.parse::<u64>().unwrap_or(0) >= 1, "CPU 核心数无效");
    }

    #[test]
    fn test_cpu_info_05() {
        let cpu_info = CpuInfo::new();
        println!("调用 cpu_thread_count 方法:");
        let thread = cpu_info.cpu_thread_count();
        assert!(thread.parse::<u64>().unwrap_or(0) >= 1, "CPU 线程数无效");
    }
}
