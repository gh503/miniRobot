use chrono::{Datelike, Utc};
use toml;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
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

impl Config {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        // 获取绝对路径
        let abs_path = fs::canonicalize(Path::new(file_path))?;
        let config_str = fs::read_to_string(&abs_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // 生成src/version.rs
    let config = Config::new("Cargo.toml")?;
    generate_version_rs(config);

    Ok(())
}

fn generate_version_rs(config: Config) {
    let copyright = format!("
Copyrigt (C) {} gh503. License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.  There is NO WARRANTY, to the extent permitted by law.", Utc::now().year());
    let about = format!("{} By Rust Edtion {}.", config.package.description, config.package.edition);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    let mut file = fs::File::create(&dest_path).expect("Failed to create version.rs");
    writeln!(file, "pub const NAME: &str = \"{}\";", config.package.name).expect("Failed to write to version.rs");
    writeln!(file, "pub const VERSION: &str = \"{}\";", config.package.version).expect("Failed to write to version.rs");
    writeln!(file, "pub const AUTHORS: &str = \"{}\";", config.package.authors.join(", ")).expect("Failed to write to version.rs");
    writeln!(file, "pub const ABOUT: &str = \"{}\";", about).expect("Failed to write to version.rs");
    writeln!(file, "pub const COPYRIGHT: &str = \"{}\";", copyright).expect("Failed to write to version.rs");
}
