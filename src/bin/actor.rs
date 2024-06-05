extern crate clap;
use clap::{Arg, Command};

use minirobot::info::hostinfo::HostInfo;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
    // 创建命令行参数解析器
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .after_help(COPYRIGHT)
        .arg(
            Arg::new("ctype")
                .short('c')
                .long("connect-type")
                .value_name("CONNECTION_TYPE")
                .help("Specify the type to connect target device")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let valid_ctypes = vec!["console", "ftp", "http", "shell", "ssh", "telnet", "web"];
    // 检查命令行参数并执行相应操作
    let default_t = "".to_string();
    let ctype = matches.get_one::<String>("ctype").unwrap_or(&default_t).as_str();
    if valid_ctypes.contains(&ctype) {
        println!("Actor connect type: {}", ctype);
    } else {
        eprintln!("invalid actor connect type: {}", ctype);
        return ()
    }

    // 创建 HostInfo 对象
    println!("运行主机当前信息:");
    let host_info = HostInfo::new();
    host_info.display();
}
