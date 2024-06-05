use std::fs::File;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::from_reader;

use crate::common::ds::TaskStatus;

pub fn read_config<T: DeserializeOwned>(config_file: &str,) -> T {
    let file = File::open(config_file).expect("无法打开配置文件");
    from_reader(file).expect("无法解析配置文件")
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub env: EnvConfig,
    pub app: AppConfig,
    pub security: SecurityConfig,
    pub database_url: String,
    pub peers_file: String,
    pub monitor_config: String,
}

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub rust_log: String,
    pub rust_backtrace: u32,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub environment: String,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub encrypt: SecurityEnryptConfig,
    pub auth_remote: SecurityAuthRemoteConfig,
}

#[derive(Debug, Deserialize)]
pub struct SecurityEnryptConfig {
    pub algorithm: String,
    pub key: String,
    pub auth_password: String,
}

#[derive(Debug, Deserialize)]
pub struct SecurityAuthRemoteConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct MonitorConfig {
    pub disk_usage: DiskUsageMonitorConfig,
    pub open_port: OpenPortMonitorConfig,
    pub task_status: TaskStatusMonitorConfig,
}

#[derive(Debug, Deserialize)]
pub struct DiskUsageMonitorConfig {
    pub threshold: u8,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct OpenPortMonitorConfig {
    pub high_risk_ports: Vec<u16>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct TaskStatusMonitorConfig {
    pub default_statuses: Vec<TaskStatus>,
    pub task_list: Vec<TaskListMonitorConfig>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct TaskListMonitorConfig {
    pub name: String,
    pub statuses: Vec<TaskStatus>,
}
