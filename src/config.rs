use toml;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub env: EnvConfig,
    pub general: GeneralConfig,
    pub security: SecurityConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub rust_log: String,
    pub rust_backtrace: u32,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub app_name: String,
    pub app_environment: String,
    pub app_node_id: String,
    pub app_port: u16,
    pub peers_file: String,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub algorithm: String,
    pub key: String,
    pub auth_password: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        // 获取绝对路径
        let abs_path = fs::canonicalize(Path::new(file_path))?;
        let config_str = fs::read_to_string(&abs_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn load_peers(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let peers_str = fs::read_to_string(&self.general.peers_file)?;
        let peers: Vec<String> = serde_json::from_str(&peers_str)?;
        Ok(peers)
    }
}
