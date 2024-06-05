use std::net::IpAddr;
use std::str;

use colored::Colorize;
use pnet::datalink;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct NetworkInterface {
    name: String,
    mac: String,
    status: String,
    ipv4: Vec<String>,
    ipv6: Vec<String>,
}

impl NetworkInterface {
    pub fn new(name: &str, mac: &str, status: &str, ipv4: &Vec<String>, ipv6: &Vec<String>) -> Self {
        NetworkInterface {
            name: name.to_string(),
            mac: mac.to_string(),
            status: status.to_string(),
            ipv4: ipv4.to_vec(),
            ipv6: ipv6.to_vec(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mac(&self) -> &str {
        &self.mac
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn ipv4(&self) -> &Vec<String> {
        &self.ipv4
    }

    pub fn ipv6(&self) -> &Vec<String> {
        &self.ipv6
    }

    pub fn display(&self) -> String {
        let mut ipv4 = "".to_string();
        for n in self.ipv4.iter().map(|n| n.green().to_string()).collect::<Vec<String>>() {
            ipv4.push_str(&format!(" {}", n));
        }
        let mut ipv6 = "".to_string();
        for n in self.ipv6.iter().map(|n| n.green().to_string()).collect::<Vec<String>>() {
            ipv6.push_str(&format!(" {}", n));
        }
        let output;
        if self.status.eq("UP") {
            output = format!("NIC {} status {}\n  MAC: {}\n  IPv4: {}\n  IPv6: {}",
                &self.name.green().to_string(),
                &self.status.green().to_string(),
                &self.mac.green().to_string(),
                &ipv4,
                &ipv6
            );
        } else {
            output = format!("NIC {} status {}\n  MAC: {}\n  IPv4: {}\n  IPv6: {}",
                &self.name.green().to_string(),
                &self.status.red().to_string(),
                &self.mac.green().to_string(),
                &ipv4,
                &ipv6
            );
        }
        println!("{}\n", output);
        output
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
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


#[cfg(test)]
mod unit_test_network {
    use super::*;

    #[test]
    fn test_network_info_01() {
        let net_info = NetworkInterface::new(
            "lo", 
            "00:00:00:00:00:00",
            "UP",
            &Vec::from(["0.0.0.0".to_string()]),
            &Vec::from(["::".to_string()]),
        );
        println!("calling fn display:");
        let output = net_info.display();
        assert!(output.contains("lo") && output.contains("UP"), "failed since nic info mismatch");
    }

    #[test]
    fn test_network_info_02() {
        let net_info = NetworkInterface::new(
            "lo", 
            "00:00:00:00:00:00",
            "UP",
            &Vec::from(["0.0.0.0".to_string()]),
            &Vec::from(["::".to_string()]),
        );
        println!("calling fn to_json:");
        let output = net_info.to_json();
        assert!(output.contains("lo") && output.contains("UP")
            && output.contains("name")
            && output.contains("status")
            && output.contains("mac")
            && output.contains("ipv4")
            && output.contains("ipv6"),
            "failed since some network json info missing"
        );
    }

    #[test]
    fn test_network_info_03() {
        let net_info = NetworkInterface::new(
            "lo", 
            "00:00:00:00:00:00",
            "UP",
            &Vec::from(["0.0.0.0".to_string()]),
            &Vec::from(["::".to_string()]),
        );
        println!("calling fn name:");
        let output = net_info.name();
        assert!(output.eq("lo"), "failed since nic name mismatch");
    }

    #[test]
    fn test_network_info_04() {
        let net_info = NetworkInterface::new(
            "lo", 
            "00:00:00:00:00:00",
            "UP",
            &Vec::from(["0.0.0.0".to_string()]),
            &Vec::from(["::".to_string()]),
        );
        println!("calling fn status:");
        let output = net_info.status();
        assert!(output.eq("UP"), "failed since nic status mismatch");
    }

    #[test]
    fn test_network_info_05() {
        let net_info = NetworkInterface::new(
            "lo", 
            "00:00:00:00:00:00",
            "UP",
            &Vec::from(["0.0.0.0".to_string()]),
            &Vec::from(["::".to_string()]),
        );
        println!("calling fn mac:");
        let output = net_info.mac();
        assert!(output.eq("00:00:00:00:00:00"), "failed since nic mac mismatch");
    }

    #[test]
    fn test_network_info_06() {
        let net_info = NetworkInterface::new(
            "lo", 
            "00:00:00:00:00:00",
            "UP",
            &Vec::from(["0.0.0.0".to_string()]),
            &Vec::from(["::".to_string()]),
        );
        println!("calling fn ipv4:");
        let output = net_info.ipv4();
        assert!(output.join(" ").contains("0.0.0.0"), "failed since nic ipv4 mismatch");
    }

    #[test]
    fn test_network_info_07() {
        let net_info = NetworkInterface::new(
            "lo", 
            "00:00:00:00:00:00",
            "UP",
            &Vec::from(["0.0.0.0".to_string()]),
            &Vec::from(["::".to_string()]),
        );
        println!("calling fn ipv6:");
        let output = net_info.ipv6();
        assert!(output.join(" ").contains("::"), "failed since nic ipv6 mismatch");
    }
}
