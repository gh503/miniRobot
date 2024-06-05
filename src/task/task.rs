use std::time::{Instant, Duration};

use uuid::Uuid;

use crate::common::ds::{Result, TaskStatus};

#[derive(Debug, Clone)]
// 定义任务结构体
pub struct Task {
    id: String,                        // 任务ID
    name: String,                      // 任务名
    description: String,               // 任务描述
    status: TaskStatus,                // 任务状态
    create: Instant,                   // 任务创建时间
    // actor: Actor,                      // 任务执行器
    log_file: String,                  // 任务执行日志文件
    // jobs: Option<Vec<Job>>,            // 执行子任务序列
    start: Option<Instant>,            // 任务开始时间
    end: Option<Instant>,              // 任务结束时间
    cost: Option<Duration>,            // 任务执行时间
    result: Option<Result>,            // 任务结果状态
}

impl Task {
    // pub fn new(name: &str, description: &str, jobs: Option<Vec<Job>>, log_file: Option<&str>) -> Task {
    pub fn new(name: &str, description: &str, log_file: Option<&str>) -> Task {

        let id = Uuid::new_v4().to_string();

        #[cfg(target_os = "linux")]
        let default_log_file = format!("/var/log/minirobot/task/{}", id);

        #[cfg(target_os = "windows")]
        let default_log_file = format!("");

        #[cfg(target_os = "macos")]
        let default_log_file = format!("");
        
        Task {
            id: id,
            name: name.to_string(),
            description: description.to_string(),
            // jobs: jobs,
            status: TaskStatus::Created,
            create: Instant::now(),
            log_file: log_file.unwrap_or(&default_log_file).to_string(),
            result: None,
            start: None,
            end: None,
            cost: None,
        }
    }
}
