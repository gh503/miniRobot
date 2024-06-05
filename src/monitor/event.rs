use chrono::Utc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    id: String,                 // 事件ID
    name: String,               // 事件名称
    timestamp: String,          // 事件时间戳
    description: String,        // 事件描述
    priority: Priority,         // 事件优先级
    severity: Severity,         // 事件严重等级
    category: String,           // 事件类别
    status: Status,             // 事件状态
    progress: String,           // 事件进展描述
}

// 定义优先级枚举
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,     // 低优先级
    Medium,  // 中等优先级
    High,    // 高优先级
}

// 定义严重等级枚举
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Debug,     // 调试级别
    Info,      // 信息级别
    Warning,   // 警告级别
    Critical,  // 严重级别
}

// 定义事件状态枚举
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending,     // 未处理
    InProgress,  // 处理中
    Resolved,    // 已解决
    Closed,      // 已关闭
}

impl Event {
    // 创建一个新的事件
    pub fn new(name: &str, description: &str, priority: Priority, severity: Severity, category: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            timestamp: Utc::now().naive_utc().to_string(),
            description: description.to_string(),
            priority,
            severity,
            category: category.to_string(),
            status: Status::Pending,
            progress: String::new(),
        }
    }

    // 获取事件ID
    pub fn id(&self) -> &str {
        &self.id
    }

    // 获取事件名称
    pub fn name(&self) -> &str {
        &self.name
    }

    // 获取事件时间戳
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    // 获取事件描述
    pub fn description(&self) -> &str {
        &self.description
    }

    // 获取事件优先级
    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    // 获取事件严重等级
    pub fn severity(&self) -> &Severity {
        &self.severity
    }

    // 获取事件类别
    pub fn category(&self) -> &str {
        &self.category
    }

    // 获取事件状态
    pub fn status(&self) -> &Status {
        &self.status
    }

    // 获取事件进展
    pub fn progress(&self) -> &str {
        &self.progress
    }

    // 设置事件名称
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // 设置事件描述
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    // 设置事件优先级
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    // 设置事件严重等级
    pub fn set_severity(&mut self, severity: Severity) {
        self.severity = severity;
    }

    // 设置事件类别
    pub fn set_category(&mut self, category: &str) {
        self.category = category.to_string();
    }

    // 设置事件状态
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    // 设置事件进展
    pub fn set_progress(&mut self, progress: &str) {
        self.progress.push_str(&format!("\n{} 更新进展:\n{}", Utc::now().naive_utc(), progress));
    }
}

#[cfg(test)]
mod unit_test_events {
    use super::*;

    #[test]
    fn test_events_01() {
        // 创建一个新的事件
        let mut event = Event::new(
            "user_login",
            "User logged into the system",
            Priority::High,
            Severity::Info,
            "authentication",
        );

        // 打印事件信息
        println!("Event: {:?}", event);

        // 修改事件状态
        event.set_status(Status::Resolved);

        // 打印修改后的事件信息
        println!("Updated Event: {:?}", event);
    }
}
