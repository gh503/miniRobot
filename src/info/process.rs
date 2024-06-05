use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
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

    pub fn display(&self) -> String {
        let output = format!("{:>10} {:<15} {:<30} {:<}", &self.pid, &self.command, &self.exe_path, &self.full_command);
        print!("{}\n", output);
        output
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| {
            eprintln!("序列化 Process 信息到 JSON 失败: {}", err);
            "{}".to_string()
        })
    }
}

pub fn gather_process_info() -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_all();
    system.processes().values().map(|process| {
        let pid = process.pid().to_string();
        let command = process.cmd().join(" ").split_whitespace().next().unwrap_or_default().to_string();
        let exe_path = process.exe().map_or_else(|| String::new(), |p| p.display().to_string());
        let full_command = process.cmd().join(" ");

        ProcessInfo::new(&pid, &command, &exe_path, &full_command)
    }).collect()
}

#[derive(Debug, Clone)]
pub enum Filter<'a> {
    ByPid(&'a str),
    ByKeyword(&'a str),
    ByMask(&'a str),
}

pub fn get_filtered_processes_as_json(filter: Option<Filter>, mask: Option<Filter>) -> String {
    let processes = gather_process_info();
    let filtered_processes = filter_processes(&processes, filter, mask);
    serde_json::to_string(&filtered_processes).unwrap_or_else(|_| "[]".to_string())
}

pub fn get_filtered_processes_as_list(filter: Option<Filter>, mask: Option<Filter>) -> Vec<ProcessInfo> {
    let processes = gather_process_info();
    filter_processes(&processes, filter, mask)
}

fn filter_processes(processes: &[ProcessInfo], filter: Option<Filter>, mask: Option<Filter>) -> Vec<ProcessInfo> {
    processes.iter().filter(|process| {
        // 根据filter决定是否包含进程
        let include = match &filter {
            Some(Filter::ByPid(pid)) => &process.pid == pid,
            Some(Filter::ByKeyword(keyword)) => process.command.contains(keyword) || process.full_command.contains(keyword),
            Some(Filter::ByMask(_)) => true,
            None => true,
        };

        // 如果include为true，再根据mask决定是否排除进程
        if include {
            match &mask {
                Some(Filter::ByKeyword(mask_keyword)) => !process.full_command.contains(mask_keyword),
                _ => true,
            }
        } else {
            false
        }
    }).cloned().collect()
}
