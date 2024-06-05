use std::collections::HashMap;
use std::process::Command;
use std::str;

use colored::Colorize;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct DiskInfo {
    // disk items: name, total bytes
    block_device_info: Vec<HashMap<String, String>>,
    // partition items: partition, free, total, use%
    partition_info: Vec<HashMap<String, String>>,
}

impl DiskInfo {
    pub fn new() -> DiskInfo {
        // Get disk information
        let (block_device_info, partition_info) = get_disk_info();

        DiskInfo {
            block_device_info,
            partition_info,
        }
    }
    
    pub fn block_device_info(&self) -> &Vec<HashMap<String, String>> {
        &self.block_device_info
    }

    pub fn partition_info(&self) -> &Vec<HashMap<String, String>> {
        &self.partition_info
    }

    pub fn display(&self) -> String {
        let mut output = String::from("Disk Size Info:");
        for map in &self.block_device_info {
            if let Some(device) = map.get("name") {
                let total = map.get("total").unwrap().green().to_string();
                let model = map.get("model").unwrap().blue().to_string();
                output.push_str(&format!("\n  disk {:<4}: {:>10} total ({})", device, total, model));
            }
        }
        output.push_str("\n\nPartition (Linux/MacOS 1k-blocks) / (Windows Size) Info:");
        for map in &self.partition_info {
            if let Some(partition) = map.get("partition") {
                let free = map.get("free").unwrap().green().to_string();
                let total = map.get("total").unwrap().blue().to_string();
                let percentage = map.get("use%").unwrap().yellow().to_string();
                output.push_str(&format!("\n  {:<15}:{:>20} free{:>20} total, use {:>4}", partition, free, total, percentage))}
        }
        println!("{}\n", output);
        output
    }

    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| {
            eprintln!("序列化 DISK 信息到 JSON 失败: {}", err);
            "{}".to_string()
        })
    }
}

#[cfg(target_os = "windows")]
fn get_disk_info() -> (Vec<HashMap<String, String>>, Vec<HashMap<String, String>>) {
    use regex::Regex;
    use crate::local::utils::format_size;

    let mut block_device_info: Vec<HashMap<String, String>>= Vec::new();
    let mut partition_info: Vec<HashMap<String, String>> = Vec::new();

    // 获取磁盘设备信息
    let output = Command::new("wmic")
        .arg("diskdrive")
        .arg("where")
        .arg("MediaType='Fixed hard disk media'")
        .arg("get")
        .arg("Caption,Size,MediaType,Model")
        .output()
        .expect("Failed to execute wmic command");
    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    // 定义正则表达式来匹配 Caption、Size、MediaType 和 Model
    let re = Regex::new(r"(.+?)\s+(\d+)\s+(.+?)\s+(.+)")
        .expect("Failed to compile regex");
    for cap in re.captures_iter(output_str) {
        let media_type = cap[3].trim();
        if media_type == "Fixed hard disk media" {
            block_device_info.push(HashMap::from([
                ("name".to_string(), cap[1].trim().to_string()),
                ("total".to_string(), format_size(cap[2].parse().expect("Failed to parse size")).to_string()),
                ("type".to_string(), media_type.to_string()),
                ("model".to_string(), cap[4].trim().to_string())
            ]));
        }
    }

    // 执行 wmic 命令来获取逻辑磁盘信息
    let output = Command::new("wmic")
        .arg("logicaldisk")
        .arg("where")
        .arg("DriveType=3")
        .arg("get")
        .arg("DeviceID,Size,FreeSpace")
        .output()
        .expect("Failed to execute wmic command");
    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    // 定义正则表达式来匹配 DeviceID、Size 和 FreeSpace
    let re = Regex::new(r"(\w:)\s+(\d+)\s+(\d+)")
        .expect("Failed to compile regex");
    for cap in re.captures_iter(output_str) {
        let device_id = cap[1].trim().to_string();
        let total_size: u64 = cap[2].parse().expect("Failed to parse total size");
        let free_space: u64 = cap[3].parse().expect("Failed to parse free space");
        let used_space = total_size - free_space;
        let usage_percentage = (used_space as f64 / total_size as f64) * 100.0;

        partition_info.push(HashMap::from([
            ("partition".to_string(), device_id),
            ("free".to_string(), format_size(free_space)),
            ("total".to_string(), format_size(total_size)),
            ("use%".to_string(), format!("{}%", usage_percentage))
        ]));
    }

    (block_device_info, partition_info)
}

#[cfg(target_os = "linux")]
fn get_disk_info() -> (Vec<HashMap<String, String>>, Vec<HashMap<String, String>>) {
    let mut block_device_info: Vec<HashMap<String, String>>= Vec::new();
    let mut partition_info: Vec<HashMap<String, String>> = Vec::new();
    let output = Command::new("lsblk")
        .arg("-d")  // 只显示磁盘设备
        .arg("-o")
        .arg("NAME,SIZE,TYPE,MODEL")
        .output()
        .expect("Failed to execute lsblk");
    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    for line in output_str.lines().skip(1) { // 跳过标题行
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() >= 4 {
            block_device_info.push(HashMap::from([
                ("name".to_string(), columns[0].to_string()),
                ("total".to_string(),columns[1].to_string()),
                ("type".to_string(), columns[2].to_string()),
                ("model".to_string(), columns[3..columns.len()].join(" ").to_string())
            ]));
        }
    }

    let output = Command::new("df")
        .arg("-xtmpfs")
        .arg("-xefi")
        .arg("-xefivarfs")
        .output()
        .expect("Failed to execute df");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    for line in lines.iter().skip(1) {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 6 {
            partition_info.push(HashMap::from([
                ("partition".to_string(), parts[5].to_string()),
                ("free".to_string(), parts[3].to_string()),
                ("total".to_string(), parts[1].to_string()),
                ("use%".to_string(), parts[4].to_string())
            ]));
        }
    }

    (block_device_info, partition_info)
}

#[cfg(target_os = "macos")]
fn get_disk_info() -> (Vec<HashMap<String, String>>, Vec<HashMap<String, String>>) {
    let mut block_device_info: Vec<HashMap<String, String>>= Vec::new();
    let mut partition_info: Vec<HashMap<String, String>> = Vec::new();
    // 获取物理磁盘设备和大小
    let output = Command::new("diskutil")
        .arg("list")
        .output()
        .expect("Failed to execute diskutil command");
    let output_str = str::from_utf8(&output.stdout).expect("Failed to parse diskutil output");
    for line in output_str.lines() {
        if line.contains("disk") {
            let disk_name = line.split_whitespace().next().unwrap_or("").to_string();
            let d_output = Command::new("sh")
                .arg("-c")
                .arg(format!("df /dev/{}", disk_name))
                .output()
                .expect("Failed to execute df command");
            let tmp_str = str::from_utf8(&d_output.stdout).unwrap();
            let parts: Vec<&str> = tmp_str.split_whitespace().collect();
            if parts.len() >= 9 {
                block_device_info.push(HashMap::from([
                    ("name".to_string(), disk_name),
                    ("total".to_string(), parts[8].to_string())
                ]));
            }
        }
    }

    // 获取分区挂载和分区大小及剩余空间大小（以字节为单位）
    let output = Command::new("df")
        .arg("-k") // 设置块大小为1字节
        .output()
        .expect("Failed to execute df command");
    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    let re = Regex::new(r"^(/[\w/]+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)%\s+(/.*)$")
        .expect("Failed to compile regex");
    for cap in re.captures_iter(output_str) {
        let total_size: u64 = cap[2].parse().expect("Failed to parse total size");
        let available_space: u64 = cap[4].parse().expect("Failed to parse available space");
        let usage_percentage: u8 = cap[5].parse().expect("Failed to parse usage percentage");
        let mounted_on = cap[6].trim();
        partition_info.push(HashMap::from([
            ("partition".to_string(), mounted_on.to_string()),
            ("free".to_string(), available_space.to_string()),
            ("total".to_string(), total_size.to_string()),
            ("use%".to_string(), format!("{}%", usage_percentage))
        ]));
    }

    (block_device_info, partition_info)
}

#[cfg(test)]
mod unit_test_disk {
    use super::*;
    
    #[test]
    fn test_disk_info_01() {
        let disk_info = DiskInfo::new();
        println!("calling fn display:");
        let output = disk_info.display();
        assert!(output.contains(" disk ") && output.contains(" free"))
    }

    #[test]
    fn test_disk_info_02() {
        let disk_info = DiskInfo::new();
        println!("calling fn block_device_info:");
        let disk_size_info = disk_info.block_device_info();
        assert!(disk_size_info.len() > 0);
        for map in disk_size_info {
            for key in vec!["name", "total", "type", "model"] {
                if let Some(value) = map.get(key) {
                    assert!(!value.is_empty(), "failed since some disk device info invalid.");
                } else {
                    assert!(!key.is_empty(), "failed since some disk device info not exists.");
                }
            }
        }
    }

    #[test]
    fn test_disk_info_03() {
        let disk_info = DiskInfo::new();
        println!("calling fn partition_info:");
        let partition_info = disk_info.partition_info();
        for map in partition_info {
            for key in vec!["partition", "free", "total", "use%"] {
                if let Some(value) = map.get(key) {
                    assert!(!value.is_empty(), "failed since some disk partition info invalid");
                } else {
                    assert!(!key.is_empty(), "failed since some disk partition info not exists");
                }
            }
        }
    }

    #[test]
    fn test_disk_info_04() {
        let disk_info = DiskInfo::new();
        println!("calling fn to_json:");
        let disk_info_json = disk_info.to_json();
        println!("json output: {}", disk_info_json);
        assert!(disk_info_json.contains("[{")
            && disk_info_json.contains("block_device_info")
            && disk_info_json.contains("partition_info"));
    }
}
