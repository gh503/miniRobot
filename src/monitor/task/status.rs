use crate::monitor::event::*;
use crate::common::ds::TaskStatus;
use crate::common::config::TaskListMonitorConfig;

pub fn check_task_status(default_statuses: &Vec<TaskStatus>, 
                         task_status_list: &Vec<TaskListMonitorConfig>,
                         is_active: &bool) -> Option<Event> {
    let mut alert_contents = String::new();

    if alert_contents.is_empty() {
        None
    } else {
        if *is_active {
            Some(Event::new("任务状态监控",
                &format!("任务状态变化: {}", alert_contents),
                Priority::Low,
                Severity::Info,
                "本地主机监控"
            ))
        } else {
            Some(Event::new("任务状态监控",
                &format!("任务状态变化: {}", alert_contents),
                Priority::Low,
                Severity::Debug,
                "本地主机监控"
            ))
        }
    }
}