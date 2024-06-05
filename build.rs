use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

use chrono::{Datelike, Utc};
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct CargoConfig {
    package: PackageConfig,
}

// 定义 Package 结构体
#[derive(Debug, Deserialize)]
struct PackageConfig {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    edition: String,
}

impl CargoConfig {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        // 获取绝对路径
        let abs_path = fs::canonicalize(Path::new(file_path))?;
        let config_str = fs::read_to_string(&abs_path)?;
        let config: CargoConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let config = CargoConfig::new("Cargo.toml")?;

    // 生成src/version.rs
    let copyright = format!("
Copyrigt (C) {} gh503. License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.  There is NO WARRANTY, to the extent permitted by law.", Utc::now().year());
    let about = format!("{} by Rust Edtion {}.", config.package.description, config.package.edition);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    let mut file = fs::File::create(&dest_path).expect("Failed to create version.rs");
    writeln!(file, "pub const NAME: &str = \"{}\";", config.package.name).expect("Failed to write to version.rs");
    writeln!(file, "pub const VERSION: &str = \"{}\";", config.package.version).expect("Failed to write to version.rs");
    writeln!(file, "pub const AUTHORS: &str = \"{}\";", config.package.authors.join(", ")).expect("Failed to write to version.rs");
    writeln!(file, "pub const ABOUT: &str = \"{}\";", about).expect("Failed to write to version.rs");
    writeln!(file, "pub const COPYRIGHT: &str = \"{}\";", copyright).expect("Failed to write to version.rs");

    // 生成配置文件变量 config.rs
    let global_config_file: String;
    let global_monitor_file: String;

    #[cfg(target_os = "linux")]
    {
        global_config_file = format!("/etc/{}/config.json", config.package.name);
        global_monitor_file = format!("/etc/{}/monitor.json", config.package.name);
    }

    #[cfg(target_os = "macos")]
    {
        global_config_file = format!("/etc/{}/config.json", config.package.name);
        global_monitor_file = format!("/etc/{}/monitor.json", config.package.name);
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        global_config_file = "config.json".to_string();
        global_monitor_file = "monitor.json".to_string();
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("configfile.rs");
    let mut file = fs::File::create(&dest_path).expect("Failed to create configfile.rs");
    writeln!(file, "pub const GLOBAL_CONFIG_FILE: &str = \"{}\";", global_config_file).expect("Failed to write to configfile.rs");
    writeln!(file, "pub const GLOBAL_MONITOR_FILE: &str = \"{}\";", global_monitor_file).expect("Failed to write to configfile.rs");

    Ok(())
}
