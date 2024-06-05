use log::{debug, error, info, trace, warn};
use colored::*;
use std::env;

use minirobot::common::config::{read_config, GlobalConfig};
use minirobot::common::logger;
use minirobot::host;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
include!(concat!(env!("OUT_DIR"), "/configfile.rs"));

fn main() {
// fn main() {
    // 初始化日志记录
    logger::init();

    let global_config = read_config::<GlobalConfig>(GLOBAL_CONFIG_FILE);

    // 更新环境变量
    env::set_var("RUST_LOG", global_config.env.rust_log);
    env::set_var("RUST_BACKTRACE", global_config.env.rust_backtrace.to_string());

    let app_port: u16 = global_config.app.port;
    let app_env = global_config.app.environment;
    let database_url = global_config.database_url;
    info!("Starting {} in {} mode on port {}", NAME.green(), app_env.green(), app_port.to_string().green());
    debug!("Connecting to database at {}", database_url.green());

    // 使用不同日志级别和颜色打印信息
    info!("{}", "Information message".green());
    warn!("{}", "Warning message".yellow());
    error!("{}", "Error message".red());
    debug!("{}", "Debug message".blue());
    trace!("{}", "Trace message".magenta());

    // 创建 Host 对象并显示信息
    let host = host::Host::new();
    // 显示主机信息
    host.hostinfo().display();
}
