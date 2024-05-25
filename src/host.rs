use std::net::IpAddr;
use std::str;
use hostname::get;
use colored::Colorize;
use pnet::datalink;
use serde::{Serialize, Deserialize};
use serde_json;
use sysinfo::System;

use crate::local;
use local::cpu::CpuInfo;
use local::disk::DiskInfo;
use local::memory::MemInfo;
use local::network::NetworkInterface;
use local::os::OSInfo;
use local::process::ProcessInfo;


#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    hostname: String,
    os_info: OSInfo,
    cpu_info: CpuInfo,
    disk_info: DiskInfo,
    mem_info: MemInfo,
    net_info: Vec<NetworkInterface>,
    active_net_info: Vec<NetworkInterface>,
    process_info: Vec<ProcessInfo>,
}

#[derive(Debug, Clone)]
pub enum Filter<'a> {
    ByPid(&'a str),
    ByKeyword(&'a str),
    ByMask(&'a str),
}

impl Host {
    pub fn new() -> Host {
        // Get hostname
        let hostname = get()
            .and_then(|os_str| Ok(os_str.into_string().ok()))
            .unwrap_or_else(|_| Some("Unknown".to_string()));

        let os_info = OSInfo::new();
        let cpu_info = CpuInfo::new();
        let disk_info = DiskInfo::new();
        let mem_info = MemInfo::new();
        let (net_info , active_net_info) = get_nics();
        let process_info = gather_process_info();

        Host {
            hostname: hostname.expect("REASON"),
            os_info,
            cpu_info,
            disk_info,
            mem_info,
            net_info,
            active_net_info,
            process_info,
        }
    }

    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }

    pub fn get_os_info(&self) -> &OSInfo {
        &self.os_info
    }

    pub fn get_cpu_info(&self) -> &CpuInfo {
        &self.cpu_info
    }

    pub fn get_disk_info(&self) -> &DiskInfo {
        &self.disk_info
    }

    pub fn get_mem_info(&self) -> &MemInfo {
        &self.mem_info
    }

    pub fn get_net_info(&self) -> &Vec<NetworkInterface> {
        &self.net_info
    }

    pub fn get_active_net_info(&self) -> &Vec<NetworkInterface> {
        &self.active_net_info
    }

    pub fn get_process_info(&self) -> &Vec<ProcessInfo> {
        &self.process_info
    }

    pub fn display(&self) {
        println!("Hostname: {}", self.get_hostname().green());
        self.os_info.display();
        self.cpu_info.display();
        self.disk_info.display();
        self.mem_info.display();
        for nic in self.get_active_net_info().iter() {
            nic.display();
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn get_filtered_processes_as_json(&self, filter: Option<Filter>, mask: Option<Filter>) -> String {
        let processes = gather_process_info();
        let filtered_processes = filter_processes(&processes, filter, mask);
        serde_json::to_string(&filtered_processes).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn get_filtered_processes_as_list(&self, filter: Option<Filter>, mask: Option<Filter>) -> Vec<ProcessInfo> {
        let processes = gather_process_info();
        filter_processes(&processes, filter, mask)
    }
}

pub fn get_nics() -> (Vec<NetworkInterface>, Vec<NetworkInterface>) {
    let mut interfaces_info = Vec::new();
    let mut active_interfaces_info = Vec::new();

    // 获取所有网络接口
    let interfaces = datalink::interfaces();

    for interface in interfaces {
        let name = interface.name.clone();
        let mac = interface.mac.map_or("N/A".to_string(), |mac| mac.to_string());
        let status = if interface.is_up() { "UP".to_string() } else { "DOWN".to_string() };
        let mut ipv4_addrs = Vec::new();
        let mut ipv6_addrs = Vec::new();

        for ip in interface.ips {
            match ip.ip() {
                IpAddr::V4(ipv4) => ipv4_addrs.push(ipv4.to_string()),
                IpAddr::V6(ipv6) => ipv6_addrs.push(ipv6.to_string()),
            }
        }

        interfaces_info.push(NetworkInterface::new(&name, &mac, &status, &ipv4_addrs, &ipv6_addrs));
        if status == "UP".to_string() && (ipv4_addrs.len() != 0 || ipv6_addrs.len() != 0) {
            active_interfaces_info.push(NetworkInterface::new(&name, &mac, &status, &ipv4_addrs, &ipv6_addrs));
        }
    }

    (interfaces_info, active_interfaces_info)
}

fn gather_process_info() -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_all();
    system.processes().values().map(|process| {
        let pid = process.pid().to_string();
        let command = process.cmd().join(" ").split_whitespace().next().unwrap_or_default().to_string();
        let exe_path = process.exe().map_or_else(|| String::new(), |p| p.display().to_string());
        let full_command = process.cmd().join(" ");

        ProcessInfo::new(&pid, &command, &exe_path, &full_command)
    }).collect()
}

fn filter_processes(processes: &[ProcessInfo], filter: Option<Filter>, mask: Option<Filter>) -> Vec<ProcessInfo> {
    processes.iter().filter(|process| {
        // 根据filter决定是否包含进程
        let include = match &filter {
            Some(Filter::ByPid(pid)) => &process.pid == pid,
            Some(Filter::ByKeyword(keyword)) => process.command.contains(keyword) || process.full_command.contains(keyword),
            Some(Filter::ByMask(_)) => true,
            None => true,
        };

        // 如果include为true，再根据mask决定是否排除进程
        if include {
            match &mask {
                Some(Filter::ByKeyword(mask_keyword)) => !process.full_command.contains(mask_keyword),
                _ => true,
            }
        } else {
            false
        }
    }).cloned().collect()
}
