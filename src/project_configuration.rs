use std::path::{Path, PathBuf};

use crate::cli::GlobalFlags;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlArguments {
    pub name: String,
    pub compiler: Option<String>,
}

/// Resolve configuraiton file and fall back to default
pub fn resolve_configuration_file(
    project_root: &Path,
    configuration_file: &Option<PathBuf>,
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
    global_flags: &GlobalFlags,
) -> Option<TomlArguments> {
    debug!("Parsing toml configuration file");
    let toml_arguments: Option<TomlArguments>;
    let config_file_path =
        resolve_configuration_file(project_root, &global_flags.configuration_file);
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
