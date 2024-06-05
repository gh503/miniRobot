use std::env;
use std::fs;
use std::str;

use colored::Colorize;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct OSInfo {
    os_type: String,
    os_name: String,
    os_version: String,
    os_arch: String,
}

impl OSInfo {
    pub fn new() -> OSInfo {
        // Get OS type
        let os_type = get_os_type();

        // Get OS name and OS version
        let (os_name, os_version) = get_os_name_version();

        // Get OS architecture
        let os_arch = std::env::consts::ARCH.to_string();

        OSInfo {
            os_type,
            os_name,
            os_version,
            os_arch,
        }
    }

    pub fn os_type(&self) -> &str {
        &self.os_type
    }

    pub fn os_name(&self) -> &str {
        &self.os_name
    }

    pub fn os_arch(&self) -> &str {
        &self.os_arch
    }

    pub fn os_version(&self) -> &str {
        &self.os_version
    }

    pub fn display(&self) -> String {
        let output = format!("OS:\n  Type: {}\n  Name: {}\n  Version: {}\n  Arch: {}",
            &self.os_type.green().to_string(),
            &self.os_name.green().to_string(),
            &self.os_version.green().to_string(),
            &self.os_arch.green().to_string()
        );
        println!("{}\n", output);
        output
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }
}

fn get_os_type() -> String {
    match env::consts::OS {
        "windows" => {
            return "Windows".to_string()
        }
        "linux" => {
            return "Linux".to_string()
        }
        "macos" => {
            return "Darwin".to_string()
        }
        _ => {
            return "Unknown".to_string()
        }
    }
}

#[cfg(target_os = "windows")]
fn get_os_name_version() -> (String, String) {
    use std::process::Command;

    let mut os_name =  String::new();
    let mut os_version = String::new();
    let output = Command::new("cmd")
        .args(&["/C", "systeminfo"])
        .output()
        .expect("failed to execute process");
    let output_str = str::from_utf8(&output.stdout).expect("failed to convert output to string");
    for line in output_str.lines() {
        if line.starts_with("OS 名称:") {
            os_name = line.split(":").nth(1).unwrap().trim().to_string();
        } else if line.starts_with("OS 版本:") {
            os_version = line.split(":").nth(1).unwrap().trim().to_string();
        }
    }
    (os_name, os_version)
}

#[cfg(target_os = "linux")]
fn get_os_name_version() -> (String, String) {
    let mut os_name =  String::new();
    let mut os_version = String::new();
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("NAME=") {
                os_name = line.replace("NAME=", "").trim_matches('"').to_string();
            }
            if line.starts_with("VERSION=") {
                os_version = line.replace("VERSION=", "").trim_matches('"').to_string();
            }
        }
    }
    (os_name, os_version)
}

#[cfg(target_os = "macos")]
fn get_os_name_version() -> (String, String) {
    use std::process::Command;

    let mut os_name =  String::new();
    let mut os_version = String::new();
    let output = Command::new("sw_vers")
        .output()
        .expect("failed to execute process");
    let output_str = str::from_utf8(&output.stdout).expect("failed to convert output to string");
    for line in output_str.lines() {
        if line.starts_with("ProductName:") {
            os_name = line.split(":").nth(1).unwrap().trim().to_string();
        } else if line.starts_with("ProductVersion:") {
            os_version = line.split(":").nth(1).unwrap().trim().to_string();
        }
    }
    (os_name, os_version)
}

#[cfg(test)]
mod unit_test_os {
    use super::*;

    #[test]
    fn test_os_info_01() {
        let os_info = OSInfo::new();
        print!("calling display:");
        let output = os_info.display();
        assert!(output.contains("Windows")
            || output.contains("Linux")
            || output.contains("Darwin")
            || output.contains("Unknown"),
            "failed since os info mismatch"
        )
    }

    #[test]
    fn test_os_info_02() {
        let os_info = OSInfo::new();
        print!("calling to_json:");
        let output = os_info.to_json();
        assert!(output.contains("os_type")
            && output.contains("os_name")
            && output.contains("os_version")
            && output.contains("os_arch"),
            "failed since some os info missing"
        )
    }

    #[test]
    fn test_os_info_03() {
        let os_info = OSInfo::new();
        print!("calling os_type:");
        let output = os_info.os_type();
        assert!(output.len() > 0, "failed since os type missing")
    }

    #[test]
    fn test_os_info_04() {
        let os_info = OSInfo::new();
        print!("calling os_name:");
        let output = os_info.os_name();
        assert!(output.len() > 0, "failed since os name missing")
    }

    #[test]
    fn test_os_info_05() {
        let os_info = OSInfo::new();
        print!("calling os_version:");
        let output = os_info.os_version();
        assert!(output.len() > 0, "failed since os version missing")
    }

    #[test]
    fn test_os_info_06() {
        let os_info = OSInfo::new();
        print!("calling os_arch:");
        let output = os_info.os_arch();
        assert!(output.len() > 0, "failed since os arch missing")
    }
}
