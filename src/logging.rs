use colored::{ColoredString, Colorize};
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logger() {
    let filter_level = match cfg!(debug_assertions) {
        true => LevelFilter::Trace,
        false => LevelFilter::Info,
    };
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{} {}", colored_status(record.level()), record.args()))
        .filter_level(filter_level)
        .init();
}

fn colored_status(level: log::Level) -> ColoredString {
    match level {
        Level::Error => level.as_str().bold().red(),
        Level::Warn => level.as_str().bold().yellow(),
        Level::Info => level.as_str().bold().cyan(),
        Level::Debug => level.as_str().bold().blue(),
        Level::Trace => level.as_str().bold().white(),
    }
}
