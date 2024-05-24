use std::str;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_mac(&self) -> &str {
        &self.mac
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_ipv4(&self) -> &Vec<String> {
        &self.ipv4
    }

    pub fn get_ipv6(&self) -> &Vec<String> {
        &self.ipv6
    }

    pub fn display(&self) {
        println!("NIC {} status {}", self.name.green(), self.status.green());
        println!("  MAC: {}", self.mac.green());
        print!("  IPv4:");
        for n in self.ipv4.iter().map(|n| n.green().to_string()).collect::<Vec<String>>() {
            print!(" {}", n);
        }
        print!("\n  IPv6:");
        for n in self.ipv6.iter().map(|n| n.green().to_string()).collect::<Vec<String>>() {
            print!(" {}", n);
        }
        println!();
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }
}