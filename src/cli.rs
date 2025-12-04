use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use strum::Display;

use crate::build::BuildAction;
use crate::compile::CompileAction;
use crate::init::InitAction;
use crate::logging;

#[derive(Debug, Parser)]
pub struct CliArguments {
    #[command(flatten)]
    pub global_flags: GlobalFlags,
    /// The action to be performed by the binary
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Clone, Args)]
#[command(next_help_heading = "Global Options")]
pub struct GlobalFlags {
    /// Change the logging level of the program
    #[arg(short = 'L', long, env = "BOLD_LOG_LEVEL", default_value_t = logging::Level::default(), global = true)]
    pub log_level: logging::Level,
    /// Relative path from project root to configuration file
    #[arg(short = 'C', long, env = "BOLD_CONFIG_FILE", global = true)]
    pub configuration_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Subcommand, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Action {
    /// Initialize a new project
    Init(InitAction),
    /// Build the project
    Build(BuildAction),
    /// Execute the compiler prepared by bold
    Compile(CompileAction),
}

pub fn get_cli_arguments() -> CliArguments {
    CliArguments::parse()
}
