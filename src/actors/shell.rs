use std::io;
use std::process::Command;
use std::sync::mpsc;
use std::time::{Instant, Duration};
use std::thread;

use crate::common::ds::Result;

// 本地命令行
#[derive(Debug, Clone)]
pub struct Shell {
    id: u32,                        // 命令序列号
    cmd: String,                    // 待执行命令
    timeout_sec: u64,               // 超时秒
    args: Option<Vec<String>>,      // 命令参数列表
    check_str: Option<String>,      // 校验字符串。存在指定字符串认为校验通过
    status: Option<i32>,            // 返回码
    stdout: Option<String>,         // 标准输出
    stderr: Option<String>,         // 错误输出
    result: Option<Result>,         // (校验后)执行结果
    start: Option<Instant>,         // 开始时间
    end: Option<Instant>,           // 结束时间
    cost: Option<Duration>,         // 执行耗时
}

impl Shell {
    pub fn new(id: &u32, cmd: &str, cmd_args: Option<&[&str]>, timeout_sec: &u64, check_str: Option<&str>) -> Self {
        assert!(!cmd.trim().is_empty());
        Self {
            id: *id,
            cmd: cmd.to_string(),
            args: cmd_args.map(|args| args.iter().map(|&arg| arg.trim().to_string()).collect()),
            timeout_sec: *timeout_sec,
            check_str: check_str.map(|s| s.trim().to_string()),
            status: None, 
            stdout: None,
            stderr: None,
            result: None,
            start: None,
            end: None,
            cost: None,
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn cmd(&self) -> &str {
        &self.cmd
    }

    pub fn args(&self) -> &Option<Vec<String>> {
        &self.args
    }

    pub fn timeout_sec(&self) -> &u64 {
        &self.timeout_sec
    }

    pub fn check_str(&self) -> &Option<String> {
        &self.check_str
    }

    pub fn status(&self) -> &Option<i32> {
        &self.status
    }

    pub fn stdout(&self) -> &Option<String> {
        &self.stdout
    }

    pub fn stderr(&self) -> &Option<String> {
        &self.stderr
    }

    pub fn result(&self) -> &Option<Result> {
        &self.result
    }

    pub fn start(&self) -> &Option<Instant> {
        &self.start
    }

    pub fn end(&self) -> &Option<Instant> {
        &self.end
    }

    pub fn cost(&self) -> &Option<Duration> {
        &self.cost
    }

    pub fn execute(&mut self) {
        self.start = Some(Instant::now());

        let args: Vec<String> = self.args.clone().unwrap_or_default();

        match execute_with_timeout(&self.cmd, &args, &Duration::from_secs(self.timeout_sec)) {
            Ok(Some(result)) => {
                self.stdout = result.stdout.clone();
                self.stderr = result.stderr.clone();
                self.status = result.status;
                
                self.result = if result.status == Some(-1) && result.stderr == Some("Command timed out".to_string()) {
                    Some(Result::Failed)
                } else if result.status == Some(-2) && result.stderr == Some("Receiver disconnected before getting a result".to_string()) {
                    Some(Result::Error)
                } else if result.status != Some(0) {
                    Some(Result::Failed)
                } else if let Some(ref check_str) = self.check_str {
                    if let Some(ref stdout) = self.stdout {
                        if stdout.contains(check_str) {
                            Some(Result::Success)
                        } else {
                            Some(Result::Failed)
                        }
                    } else {
                        Some(Result::Failed)
                    }
                } else {
                    Some(Result::Success)
                };
            },
            Ok(None) => {
                self.result = Some(Result::Error);
            },
            Err(e) => {
                eprintln!("Error executing command: {}", e);
                self.status = Some(-1);
                self.stderr = Some(e.to_string());
                self.result = Some(Result::Error);
            },
        }

        self.end = Some(Instant::now());
        self.cost = Some(self.end.unwrap() - self.start.unwrap());
    }
}

struct CmdResult {
    status: Option<i32>,
    stdout: Option<String>,
    stderr: Option<String>,
}

fn execute_with_timeout(cmd: &str, cmd_args: &[String], timeout: &Duration) -> io::Result<Option<CmdResult>> {
    let command = cmd.to_string();
    let args: Vec<String> = cmd_args.iter().map(|arg| arg.to_string()).collect();
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let result = Command::new(command)
            .args(&args)
            .output()
            .map(|output| CmdResult {
                status: output.status.code(),
                stdout: Some(String::from_utf8_lossy(&output.stdout).to_string()),
                stderr: Some(String::from_utf8_lossy(&output.stderr).to_string()),
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e));
        tx.send(result).ok();
    });

    match rx.recv_timeout(*timeout) {
        Ok(result) => result.map(Some),
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(Some(CmdResult {
            stdout: None,
            stderr: Some("Command timed out".to_string()),
            status: Some(-1),
        })),
        Err(mpsc::RecvTimeoutError::Disconnected) => Ok(Some(CmdResult {
            stdout: None,
            stderr: Some("Receiver disconnected before getting a result".to_string()),
            status: Some(-2),
        })),
    }
}

#[cfg(test)]
mod unit_test_shell {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_cmd_01() {
        let mut cmd = Shell::new(&0u32, "ls", Some(&["-l", "-a"]), &10u64, None);
        cmd.execute();
        println!("{:#?}", cmd);
        assert!(cmd.result.is_some_and(|result| result == Result::Success));
        assert!(cmd.stdout.is_some_and(|stdout| stdout.contains("total ")));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_cmd_02() {
        let mut cmd = Shell::new(&0u32, "df", Some(&["-h", "-T"]), &10u64, None);
        cmd.execute();
        println!("{:#?}", cmd);
        assert!(cmd.result.is_some_and(|result| result == Result::Success));
        assert!(cmd.stdout.is_some_and(|stdout| stdout.contains("% /")));
    }

}