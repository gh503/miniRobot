use std::path::Path;
use std::process::Command;

use crate::monitor::event::*;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[cfg(target_os = "windows")]
pub fn os_send_alert(e: Event) -> bool {
    let message = format!("{:?}: {} at {}!", e.category(), e.description(), e.timestamp());
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!("New-BurntToastNotification -Text \"{}\" -Appname \"{}\"", message, NAME))
        .output()
        .expect("Failed to send notification");
    output.status.success()
}

#[cfg(target_os = "linux")]
pub fn os_send_alert(e: Event) -> bool {
    if e.severity().eq(&Severity::Debug) {
        return true;
    }

    if Path::new("/usr/bin/notify-send").exists() {
        let message = format!("{:?}: {} at {}!", e.category(), e.description(), e.timestamp());
        let output = Command::new("notify-send")
            .arg(NAME)
            .arg(message)
            .output()
            .expect("Failed to send notification");
        output.status.success()
    } else {
        eprintln!("notify-send is not installed. Please install it using your package manager.");
        false
    }
}

#[cfg(target_os = "macos")]
pub fn os_send_alert(e: Event) -> bool {
    let message = format!("{:?}: {} at {}!", e.category(), e.description(), e.timestamp());
    let output = Command::new("osascript")
        .arg("-e")
        .arg(format!("display notification \"{}\" with title \"{}\"", message, NAME))
        .output()
        .expect("Failed to send notification");
    output.status.success()
}
