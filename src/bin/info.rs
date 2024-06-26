extern crate clap;
use clap::{Arg, Command};

use minirobot::info::hostinfo::HostInfo;
use minirobot::info::process::*;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
    // 创建命令行参数解析器
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .after_help(COPYRIGHT)
        .arg(
            Arg::new("filter-out-str")
                .short('f')
                .long("filter-out-str")
                .value_name("FILTEROUT")
                .help("Specify the processes keyword to filter out")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output in JSON format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pid")
                .short('p')
                .long("pid")
                .value_name("PID")
                .help("Specify the PID of the process to display")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("proc-str")
                .long("proc-str")
                .value_name("KEYWORD")
                .help("Specify the processes keyword to filter")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    // 检查命令行参数并执行相应操作
    let pid_string;
    let filter = if let Some(keyword) = matches.get_one::<String>("proc-str") {
        Some(Filter::ByKeyword(keyword))
    } else if let Some(pid) = matches.get_one::<u32>("pid") {
        pid_string = pid.to_string();
        Some(Filter::ByPid(&pid_string))
    } else {
        None
    };

    let mask = if let Some(keyword) = matches.get_one::<String>("filter-out-str") {
        Some(Filter::ByKeyword(keyword))
    } else {
        None
    };

    // 创建 HostInfo 对象
    let host_info = HostInfo::new();

    // 根据命令行参数获取进程信息并显示
    if matches.get_flag("json") {
        if let Some(filter) = filter {
            println!("{}", get_filtered_processes_as_json(Some(filter), mask));
        } else {
            println!("{}", host_info.to_json());
        }
    } else {
        if let Some(filter) = filter {
            println!("{:#?}", get_filtered_processes_as_list(Some(filter), mask));
        } else {
            host_info.display();
        }
    }
}
