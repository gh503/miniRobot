extern crate clap;
use clap::{Arg, Command};

use minirobot::info::hostinfo::HostInfo;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
include!(concat!(env!("OUT_DIR"), "/configfile.rs"));

fn main() {
    // 创建命令行参数解析器
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .after_help(COPYRIGHT)
        // .arg(
        //     Arg::new("ctype")
        //         .short('c')
        //         .long("connect-type")
        //         .value_name("CONNECTION_TYPE")
        //         .help("Specify the type to connect target device")
        //         .value_parser(clap::value_parser!(String)),
        // )
        .get_matches();

    println!("{} environment manager running", NAME);
    println!("config file is {}", GLOBAL_CONFIG_FILE);

    // 创建 HostInfo 对象
    println!("运行主机当前信息:");
    let host_info = HostInfo::new();
    host_info.display();
}
