use std::env;
use std::process::Command;
use std::str;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct OSInfo {
    os_type: String,
    os_name: String,
    os_version: String,
    os_arch: String,
}

impl OSInfo {
    pub fn new() -> OSInfo {
        // Get OS type
        let os_type = env::consts::OS.to_string();

        // Get OS name
        let os_name = get_os_name();

        // Get OS version
        let os_version = get_os_version();

        // Get OS architecture
        let os_arch = std::env::consts::ARCH.to_string();

        OSInfo {
            os_type,
            os_name,
            os_version,
            os_arch,
        }
    }

    pub fn get_os_type(&self) -> &str {
        &self.os_type
    }

    pub fn get_os_name(&self) -> &str {
        &self.os_name
    }

    pub fn get_os_arch(&self) -> &str {
        &self.os_arch
    }

    pub fn get_os_version(&self) -> &str {
        &self.os_version
    }

    pub fn display(&self) {
        println!("OS Type: {}", self.get_os_type().green());
        println!("OS Name: {}", self.get_os_name().green());
        println!("OS Version: {}", self.get_os_version().green());
        println!("OS Architecture: {}", self.get_os_arch().green());
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }
}

fn get_os_name() -> String {
    match env::consts::OS {
        "windows" => {
            let output = Command::new("ver")
                .output()
                .expect("Failed to execute ver command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            if let Some(line) = lines.get(0) {
                if let Some(pos) = line.find("[Version") {
                    let os_name = &line[..pos].trim();
                    return os_name.to_string();
                }
            }

            "Unknown".to_string()
        }

        "linux" => {
            use std::fs;

            if let Ok(os_version) = fs::read_to_string("/etc/os-release") {
                for line in os_version.lines() {
                    if line.starts_with("NAME=") {
                        return line.replace("NAME=", "").trim_matches('"').to_string();
                    }
                }
            }
            "Unknown".to_string()
        }
        "macos" => {
            "macOS".to_string()
        }

        _ => {
            "".to_string()
        }
    }
}

fn get_os_version() -> String {
    match env::consts::OS {
        "windows" => {
            let output = Command::new("wmic")
                .args(&["os", "get", "Caption,Version"])
                .output()
                .expect("Failed to execute wmic command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            if lines.len() > 1 {
                return lines[1].trim().to_string();
            }

            "Unknown".to_string()
        }

        "linux" => {
            use std::fs;

            if let Ok(os_version) = fs::read_to_string("/etc/os-release") {
                for line in os_version.lines() {
                    if line.starts_with("VERSION_ID=") {
                        return line.replace("VERSION_ID=", "").trim_matches('"').to_string();
                    }
                }
            }
            "Unknown".to_string()
        }

        "macos" => {
            let output = Command::new("sw_vers")
                .output()
                .expect("Failed to execute sw_vers command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            for line in lines {
                if line.starts_with("ProductVersion:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() == 2 {
                        return parts[1].to_string();
                    }
                }
            }
            "Unknown".to_string()
        }

        _ => {
            String::new()
        }
    }
}