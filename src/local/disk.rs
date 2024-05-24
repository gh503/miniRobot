use std::env;
use std::process::Command;
use std::str;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    disk_size_info: Vec<String>,
    part_size_info: Vec<String>,
}

impl DiskInfo {
    pub fn new() -> DiskInfo {
        // Get disk information
        let (disk_size_info, part_size_info) = get_disk_info();

        DiskInfo {
            disk_size_info,
            part_size_info,
        }
    }
    
    pub fn get_disk_size_info(&self) -> &Vec<String> {
        &self.disk_size_info
    }

    pub fn get_part_size_info(&self) -> &Vec<String> {
        &self.part_size_info
    }

    pub fn display(&self) {
        println!("Disk Size Info:");
        for d in self.get_disk_size_info().iter().map(|d| d.green().to_string()).collect::<Vec<String>>() {
            println!("  {}", d);
        }
        println!("Part Size Info:");
        for p in self.get_part_size_info().iter().map(|p| p.green().to_string()).collect::<Vec<String>>() {
            println!("  {}", p);
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }
}

fn get_disk_info() -> (Vec<String>, Vec<String>) {

    let mut disk_size_info = Vec::new();
    let mut part_size_info = Vec::new();

    match env::consts::OS {
        "windows" => {
                let output1 = Command::new("wmic")
                    .args(&["logicaldisk", "get", "size,freespace,caption"])
                    .output()
                    .expect("Failed to execute wmic command");

                let output1_str = String::from_utf8_lossy(&output1.stdout);

                let lines: Vec<&str> = output1_str.lines().collect();
                for line in lines.iter().skip(1) {
                    let parts: Vec<&str> = line.trim().split_whitespace().collect();
                    if parts.len() == 3 {
                        let caption = parts[0];
                        let freespace = parts[1].parse::<u64>().unwrap_or(0);
                        let size = parts[2].parse::<u64>().unwrap_or(0);

                        disk_size_info.push(format!("Block {}: {} MBytes", caption, size/1024/1024));
                        part_size_info.push(format!("{}: {} MBytes free, {} MBytes total", caption, freespace/1024/1024, size/1024/1024));
                    }
                }

        }

        "linux" | "macos" => {
                let output = Command::new("lsblk")
                    .arg("-d")  // 只显示磁盘设备
                    .arg("-o")
                    .arg("NAME,SIZE")
                    .output()
                    .expect("Failed to execute lsblk");

                // 将输出转换为字符串
                let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
                // 解析并打印 NAME 和 SIZE 列
                for line in output_str.lines().skip(1) { // 跳过标题行
                    let columns: Vec<&str> = line.split_whitespace().collect();
                    if columns.len() == 2 {
                        disk_size_info.push(format!("Block {}: {}", columns[0], columns[1]));
                    }
                }


            let output = Command::new("df")
                .arg("-H")
                .arg("-xtmpfs")
                .arg("-xefi")
                .arg("-xefivarfs")
                .output()
                .expect("Failed to execute df command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            for line in lines.iter().skip(1) {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 6 {
                    let size = parts[1].parse::<String>().unwrap_or("0".to_string());
                    let used = parts[2].parse::<String>().unwrap_or("0".to_string());
                    let available = parts[3].parse::<String>().unwrap_or("0".to_string());
                    let mountpoint = parts[5];

                    part_size_info.push(format!("{}: {} used, {} available, {} total", mountpoint, used, available, size));
                }
            }
        }

        _ => {}
    }

    (disk_size_info, part_size_info)
}
