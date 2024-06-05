use colored::Colorize;
use serde::Serialize;
use serde_json;

use crate::info;

#[derive(Debug, Serialize)]
pub struct HostInfo {
    hostname: String,
    os_info: info::os::OSInfo,
    cpu_info: info::cpu::CpuInfo,
    disk_info: info::disk::DiskInfo,
    mem_info: info::memory::MemInfo,
    net_info: Vec<info::network::NetworkInterface>,
    active_net_info: Vec<info::network::NetworkInterface>,
    process_info: Vec<info::process::ProcessInfo>,
    service_info: Vec<info::service::Service>,
}

impl HostInfo {
    pub fn new() -> Self {
        let (net_info , active_net_info) = info::network::get_nics();
        let process_info = info::process::gather_process_info();
        let service_info = info::service::get_services();
        HostInfo {
            hostname: info::hostname::hostname(),
            os_info: info::os::OSInfo::new(),
            cpu_info: info::cpu::CpuInfo::new(),
            disk_info: info::disk::DiskInfo::new(),
            mem_info: info::memory::MemInfo::new(),
            net_info,
            active_net_info,
            process_info,
            service_info,
        }
    }

    pub fn display(&self) -> String {
        let mut output = format!("\nHostname: {}", &self.hostname.green().bold().to_string());
        println!("{}\n", output);
        output.push_str(&self.os_info.display());
        output.push_str(&self.cpu_info.display());
        output.push_str(&self.mem_info.display());
        output.push_str(&self.disk_info.display());
        for nic in &self.net_info {
            output.push_str(&nic.display());
        }
        for service in &self.service_info {
            output.push_str(&service.display());
        }
        output
    }

    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| {
            eprintln!("序列化 Host 信息到 JSON 失败: {}", err);
            "{}".to_string()
        })
    }
}
