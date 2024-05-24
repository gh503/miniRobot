use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInfo {
    pub pid: String,
    pub command: String,
    #[serde(skip)]
    pub exe_path: String,
    #[serde(skip)]
    pub full_command: String,
}

impl ProcessInfo {
    pub fn new(pid: &str, command: &str, exe_path: &str, full_command: &str) -> Self {
        Self {
            pid: pid.to_string(),
            command: command.to_string(),
            exe_path: exe_path.to_string(),
            full_command: full_command.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string())
    }
}