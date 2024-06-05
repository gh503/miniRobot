#[cfg(target_os = "windows")]
pub fn hostname() -> String {
    let mut hostname = String::new();
    match env::var("COMPUTERNAME") {
        Ok(computer_name) => hostname = computer_name,
        Err(e) => eprintln!("无法获取主机名: {}", e),
    }
    hostname
}

#[cfg(target_os = "linux")]
pub fn hostname() -> String {
    let mut hostname = String::new();
    if let Ok(content) = std::fs::read_to_string("/etc/hostname") {
        hostname = content.trim().to_string();
    }
    hostname
}

#[cfg(target_os = "macos")]
pub fn hostname() -> String {
    let mut hostname = String::new();
    if let Ok(content) = std::fs::read_to_string("/etc/hostname") {
        hostname = content.trim().to_string();
    }
    hostname
}
