use std::env;
extern crate env_logger;
extern crate log;
extern crate colored;

use log::{info, warn, error, debug, trace};
use colored::*;

use mini_robot::host;
use mini_robot::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() {
    // 读取配置文件
    let config_file = "config.toml";
    let config = Config::new(config_file)?;
    // 更新环境变量
    env::set_var("RUST_LOG", config.env.rust_log);
    env::set_var("RUST_BACKTRACE", config.env.rust_backtrace.to_string());
    
    // 初始化日志记录
    env_logger::init();

    let app_name = config.general.app_name;
    let app_env = config.general.app_environment;
    let app_port: u16 = config.general.app_port;
    let database_url = config.database.url;

    info!("Starting {} in {} mode on port {}", app_name, app_env, app_port);
    debug!("Connecting to database at {}", database_url);

    // 使用不同日志级别和颜色打印信息
    info!("{}", "Information message".green());
    warn!("{}", "Warning message".yellow());
    error!("{}", "Error message".red());
    debug!("{}", "Debug message".blue());
    trace!("{}", "Trace message".magenta());

    // 创建 Host 对象并显示信息
    let host = host::Host::new();
    // 显示主机信息
    host.display();

    Ok(())
}
