use std::collections::HashSet;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Result {
    Success,
    Failed,
    Error,
}

// 任务状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Created,                           // 新建
    Wait,                              // 等待运行
    Running,                           // 运行中
    Finished,                          // 已完成
    Timeout,                           // 运行超时
    Stopped,                           // 被停止
    Cancelled,                         // 被取消
    Unavailable,                       // 不可执行（资源不可用、设置不当等问题）
}

// 运行角色
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunningRole {
    Actor,                             // 执行器
    AssetManager,                      // 资产管理器
    EnvManager,                        // 环境管理器
    TaskManager,                       // 任务管理器
    Inquirer,                          // 查询器
}

// 运行方式
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunningMode {
    Manaul,                            // 手动运行
    Auto,                              // 自动运行（minirobot进程启动后）
}

// 触发器
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trigger {
    TimeBased(TimeTrigger),
    EventBased(EventTrigger),
    EventAndTimeBased {
        event: EventTrigger,
        time: TimeTrigger,
    }
}

// 事件触发器
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventTrigger {
    SingleEvent(String),
    MultipleEvents(HashSet<String>),
}

// 时间触发器
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeTrigger {
    Daily,
    Hourly,
    Minutely,
    Secondly,
    SpecificTime { date: UserDate, time: UserTime },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDate {
    year: String,
    month: String,
    day: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserTime {
    hh: String,
    mm: String,
    ss: String,
}
