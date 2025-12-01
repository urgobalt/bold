use std::path::{Path, PathBuf};

use crate::logging;
use clap::Parser;
use log::{error, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct UserArguments {
    cli_arguments: CliArguments,
    toml_arguments: TomlArguments,
}

#[derive(Debug, Parser)]
struct CliArguments {
    /// Change the logging level of the program
    #[arg(long)]
    log_level: logging::Level,
    /// Relative path from project root to configuration file
    #[arg(short = 'C', long)]
    configuration_file: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TomlArguments {}

impl Default for TomlArguments {
    fn default() -> Self {
        Self {}
    }
}

pub fn get_user_arguments(project_root: &Path) -> UserArguments {
    let cli_arguments = CliArguments::parse();
    let config_file_path = {
        let mut relative_path = cli_arguments.configuration_file.clone().unwrap_or_default();
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
    let toml_arguments: TomlArguments;
    match std::fs::read_to_string(&config_file_path) {
        Ok(toml_file_content) => {
            toml_arguments = match toml::from_str::<TomlArguments>(&toml_file_content) {
                Err(e) => {
                    error!("Failed to parse project config file at root.");
                    warn!(
                        "Creating file {:?} with default parameters.",
                        &config_file_path
                    );

                    let default_toml_arguments = TomlArguments::default();
                    let config_file_content =
                        toml::to_string_pretty(&default_toml_arguments).unwrap();
                    let write_res = std::fs::write(&config_file_path, config_file_content);
                    if let Err(err) = write_res {
                        error!("Failed writing to configuration file path: {}", err);
                        std::process::exit(1);
                    }

                    default_toml_arguments
                }
                Ok(toml_arguments) => toml_arguments,
            };
        }
        Err(err) => {
            error!("Unable to read configuration file: {}", err);
            std::process::exit(1);
        }
    }

    UserArguments {
        cli_arguments,
        toml_arguments,
    }
}
