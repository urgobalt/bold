use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::io::Write;
use strum::Display;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Level {
    Default,
    Debug,
    Trace,
}

impl Into<log::LevelFilter> for Level {
    fn into(self) -> log::LevelFilter {
        match self {
            Level::Default => log::LevelFilter::Info,
            Level::Debug => log::LevelFilter::Debug,
            Level::Trace => log::LevelFilter::Trace,
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::Default
    }
}

pub fn init_logger() {
    use log::LevelFilter;
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
        log::Level::Error => level.as_str().bold().red(),
        log::Level::Warn => level.as_str().bold().yellow(),
        log::Level::Info => level.as_str().bold().cyan(),
        log::Level::Debug => level.as_str().bold().blue(),
        log::Level::Trace => level.as_str().bold().white(),
    }
}
