use crate::info::disk::DiskInfo;
use crate::monitor::event::*;

// 磁盘使用情况监控项
pub fn check_disk_usage(threshold_max: &u8, is_active: &bool) -> Option<Event> {
    let mut alert_contens = String::new();

    let disk_info = DiskInfo::new();
    let partition_info = disk_info.partition_info();
    for p in partition_info {
        let name = p.get("name").unwrap();
        let percentage = p.get("use%").unwrap().trim_end_matches("%").parse::<u8>().expect("failed to get partition used percentage");
        if percentage >= *threshold_max {
            alert_contens.push_str(&format!("{}-{}%", name, percentage));
        }
    }
    
    if alert_contens.is_empty() {
        None
    } else {
        if *is_active {
            Some(Event::new("磁盘使用率监控",
                &format!("使用率达到告警阈值{}%: {}", threshold_max, alert_contens),
                Priority::Low,
                Severity::Warning,
                "本地主机监控"
            ))
        } else {
            Some(Event::new("磁盘使用率监控",
                &format!("使用率达到告警阈值{}%: {}", threshold_max, alert_contens),
                Priority::Low,
                Severity::Debug,
                "本地主机监控"
            ))
        }
    }
}
