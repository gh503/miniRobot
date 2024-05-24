use dotenv::dotenv;
use std::env;
extern crate env_logger;
extern crate log;
extern crate colored;

use log::{info, warn, error, debug, trace};
use colored::*;

use mini_robot::host;

fn main() {
    // 加载 .env 文件中的环境变量
    dotenv().ok();
    
    // 初始化日志记录
    env_logger::init();

    let app_name = env::var("APP_NAME").unwrap_or_else(|_| "Unknown".to_string());
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let app_port: u16 = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("APP_PORT must be a number");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

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
    // json格式化
    // println!("json字符串：{}", host.to_json());
    // 过滤进程
    let pid_or_keyword = "1";
    let filter = Some(host::Filter::ByPid(&pid_or_keyword));
    println!("filter: {}, result:\n{}", pid_or_keyword, host.get_filtered_processes_as_json(filter.clone()));
    println!("filter: {}, result:\n{:#?}", pid_or_keyword, host.get_filtered_processes_as_list(Some(filter.unwrap())));
}
