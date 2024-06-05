use crate::info::service;
use crate::monitor::event::*;

pub fn check_open_port(high_risk_ports: &Vec<u16>, is_active: &bool) -> Option<Event> {
    let mut alert_contents = String::new();

    let services = service::get_services();
    for s in services {
        if high_risk_ports.contains(&s.port()) {
            alert_contents.push_str(&format!("Port{} OPEN", s.port()));
        }
    }

    if alert_contents.is_empty() {
        None
    } else {
        if *is_active {
            Some(Event::new("本地开放端口监控",
                &alert_contents,
                Priority::High,
                Severity::Warning,
                "本地主机监控"
            ))
        } else {
            Some(Event::new("本地开放端口监控",
                &alert_contents,
                Priority::High,
                Severity::Debug,
                "本地主机监控"
            ))
        }
    }

}
