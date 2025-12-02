use std::path::{Path, PathBuf};

use crate::compiler::CompilerType;
use crate::logging;
use clap::Parser;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use crate::DEFAULT_BUILD_FILE_NAME;

#[derive(Debug, Parser)]
pub struct CliArguments {
    /// Change the logging level of the program
    #[arg(long, default_value_t = logging::Level::default())]
    pub log_level: logging::Level,
    /// Relative path from project root to configuration file
    #[arg(short = 'C', long)]
    pub configuration_file: Option<PathBuf>,
    /// Name of build file
    #[arg(long, default_value_t = DEFAULT_BUILD_FILE_NAME.into())]
    pub build_filename: String,
    /// The compiler used to compile the build file
    #[arg(long)]
    pub compiler: Option<String>,
    /// Generate the configuration file with default options
    #[arg(long)]
    pub generate_config_file: bool,
    /// Filter for compiler resolver
    #[arg(long)]
    pub compiler_filter: Vec<CompilerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlArguments {
    pub compiler: Option<String>,
}

impl Default for TomlArguments {
    fn default() -> Self {
        Self { compiler: None }
    }
}

pub fn get_cli_arguments() -> CliArguments {
    debug!("Parsing command line arguments");
    CliArguments::parse()
}

/// Resolve configuraiton file and fall back to default
pub fn resolve_configuration_file(
    project_root: &Path,
    configuration_file: Option<PathBuf>,
) -> PathBuf {
    let config_file_path = {
        let mut relative_path = configuration_file.clone().unwrap_or_default();
        if relative_path.is_absolute() {
            error!(
                "Path to configuration file must be relative to the project root, found absolute path."
            );
            std::process::exit(1);
        }
        if relative_path.is_file() == false {
            relative_path.push(crate::DEFAULT_BUILD_CONFIG_FILE_NAME);
        }
        project_root.join(relative_path)
    };

    config_file_path
}

pub fn get_toml_arguments(
    project_root: &Path,
    cli_arguments: &CliArguments,
) -> Option<TomlArguments> {
    debug!("Parsing toml configuration file");
    let toml_arguments: Option<TomlArguments>;
    let config_file_path =
        resolve_configuration_file(project_root, cli_arguments.configuration_file.clone());
    match std::fs::read_to_string(&config_file_path) {
        Ok(toml_file_content) => {
            toml_arguments = match toml::from_str::<TomlArguments>(&toml_file_content) {
                Err(err) => {
                    error!(
                        r#"Failed to parse project config file.
                    {}"#,
                        err
                    );

                    std::process::exit(1);
                }
                Ok(toml_arguments) => Some(toml_arguments),
            };
        }
        Err(err) if { err.kind() == std::io::ErrorKind::NotFound } => {
            warn!("Configuration file {:?} not found", &config_file_path);
            info!(
                "Creating file {:?} with default parameters",
                &config_file_path
            );
            toml_arguments = None;
        }
        Err(err) => {
            error!("Unable to read configuration file: {}", err);
            std::process::exit(1);
        }
    }

    toml_arguments
}

pub fn generate_configuration_file(project_root: &Path, cli_arguments: &CliArguments) {
    let config_file_path =
        resolve_configuration_file(project_root, cli_arguments.configuration_file.clone());
    let toml_arguments = TomlArguments::default();
    let config_file_content = toml::to_string_pretty(&toml_arguments).unwrap();
    let write_res = std::fs::write(&config_file_path, config_file_content);
    if let Err(err) = write_res {
        error!("Failed writing to configuration file path: {}", err);
    }
}
