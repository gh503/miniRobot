use std::io::{self, Write};

use chrono::Local;
use colored::*;
use env_logger::fmt::Formatter;
use log::Level;

pub fn init() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(custom_format)
        .init();
}

fn custom_format(buf: &mut Formatter, record: &log::Record) -> io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.9f%:z");
    let level = match record.level() {
        Level::Error => "ERROR".red(),
        Level::Warn => "WARN ".yellow(),
        Level::Info => "INFO ".green(),
        Level::Debug => "DEBUG".blue(),
        Level::Trace => "TRACE".white(),
    };
    writeln!(
        buf,
        "[{} {} {:<5}] {}",
        timestamp,
        level,
        record.target(),
        record.args()
    )
    .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to write log message"))
}
