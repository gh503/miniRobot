use crate::common::config::{read_config, MonitorConfig};
use crate::monitor::event::Event;
use crate::monitor::system::disk_usage::check_disk_usage;
use crate::monitor::system::open_port::check_open_port;
use crate::monitor::task::status::check_task_status;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[derive(Debug)]
pub struct Monitor {
    config: MonitorConfig,
    events: Vec<Event>,
    // 监控策略
}
impl Monitor {
    pub fn new(config_json: &str) -> Self {
        Monitor {
            config: read_config::<MonitorConfig>(config_json),
            events: Vec::new(),
        }
    }

    // 监控并产生事件
    pub fn start(&mut self) {

        // 磁盘监控
        let mut event = check_disk_usage(&self.config.disk_usage.threshold,
                                                                      &self.config.disk_usage.is_active);
        if event.is_some() {
            self.events.push(event.unwrap());
        }

        // 开放端口监控
        event = check_open_port(&self.config.open_port.high_risk_ports, &self.config.open_port.is_active);
        if event.is_some() {
            self.events.push(event.unwrap());
        }

        // 任务状态监控
        event = check_task_status(&self.config.task_status.default_statuses,
                                  &self.config.task_status.task_list,
                                  &self.config.task_status.is_active);
        if event.is_some() {
            self.events.push(event.unwrap());
        }

    }
}
