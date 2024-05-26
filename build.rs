use toml;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

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
    about: String,
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

    clean_additional();

    // 生成src/version.rs
    let config = Config::new("Cargo.toml")?;
    generate_version_rs(config);
    
    pyo3_build_config::use_pyo3_cfgs();
    println!("cargo:rustc-link-lib=python3.10");

    // Call pyoxidizer to build the embedded Python interpreter
    std::process::Command::new("pyoxidizer")
        .arg("build")
        .status()
        .expect("Failed to run PyOxidizer");

    Ok(())
}

fn clean_additional() {
    let files_to_clean: Vec<PathBuf> = vec![
        Path::new("src").join("version.rs"),
    ];
    let dirs_to_clean: Vec<String>= vec![
    ];

    for f in files_to_clean {
        if let Err(e) = fs::remove_file(&f) {
            eprintln!("Failed to delete file {:?}: {}.", f, e);
        } else {
            println!("Cleaned file {:?}", f);
        }
    }

    for d in dirs_to_clean {
        if let Err(e) = fs::remove_dir_all(&d) {
            println!("Failed to clean directory {:?}: {}", d, e);
        } else {
            println!("Cleaned directory: {:?}", d);
        }
    }
}

fn generate_version_rs(config: Config) {
    let about = format!("{} By Rust {}", config.package.about, config.package.edition);

    let mut file = fs::File::create("src/version.rs").expect("Failed to create version.rs");
    writeln!(file, "pub const NAME: &str = \"{}\";", config.package.name).expect("Failed to write to version.rs");
    writeln!(file, "pub const VERSION: &str = \"{}\";", config.package.version).expect("Failed to write to version.rs");
    writeln!(file, "pub const AUTHORS: &str = \"{}\";", config.package.authors.join(", ")).expect("Failed to write to version.rs");
    writeln!(file, "pub const ABOUT: &str = \"{}\";", about).expect("Failed to write to version.rs");
}