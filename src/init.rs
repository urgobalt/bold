use std::path::{Path, PathBuf};

use clap::Args;
use log::{error, info};

use crate::cli::GlobalFlags;
use crate::project_root;

#[derive(Debug, Clone, Args)]
pub struct InitAction {
    /// Path to the directory where the project should be initialized
    pub project_root: Option<PathBuf>,
}

pub fn init_project(init_action: InitAction, global_flags: &GlobalFlags) {
    let project_root = init_action
        .project_root
        .unwrap_or(std::env::current_dir().unwrap_or_else(|err| {
            error!("Failed fetching current directory: {}", err);
            std::process::exit(1);
        }));
    if project_root.is_file() || project_root.extension() != None {
        error!("Project directory cannot be a file");
        std::process::exit(1);
    }
    if project_root.exists() {
        // TODO: Make this smarter than just exiting when the directory contains items
        match std::fs::read_dir(&project_root) {
            Ok(read_dir) => {
                if read_dir.count() != 0 {
                    error!("Directory {:?} not empty", &project_root);
                    std::process::exit(1);
                }
            }
            Err(err) => {
                error!("Unable to read specified project directory: {}", err);
                std::process::exit(1);
            }
        }
    } else {
        if let Err(err) = std::fs::create_dir_all(&project_root) {
            error!("Failed creating directory {:?}: {}", &project_root, err);
            std::process::exit(1);
        }
    }
    generate_configuration_file(project_root.as_path(), global_flags);
}

pub fn generate_configuration_file(project_root: &Path, global_flags: &GlobalFlags) {
    let config_file_path = crate::project_configuration::resolve_configuration_file(
        project_root,
        &global_flags.configuration_file,
    );
    let toml_arguments = crate::project_configuration::TomlArguments {
        // TODO: Make this handle errors gracefully
        name: config_file_path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        compiler: None,
    };
    let config_file_content = toml::to_string_pretty(&toml_arguments).unwrap();
    let write_res = std::fs::write(&config_file_path, config_file_content);
    if let Err(err) = write_res {
        error!("Failed writing to configuration file path: {}", err);
    }
    info!(
        "Successfully wrote default configuration file to {:?}",
        &config_file_path
    )
}
