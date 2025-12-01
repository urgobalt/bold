use log::{ error, info, debug, trace };
use std::path::PathBuf;
use crate::BUILD_FILE_NAME;

pub fn find_project_root() -> PathBuf {
    let current_dir = match std::env::current_dir() {
        Ok(val) => val,
        Err(err) => {
            error!(
                r#"
                Unable to get the current directory.
                Failed with: {}
                "#,
                err
            );
            std::process::exit(1);
        }
    };
    debug!("Resolved current directory to {:?}", current_dir);

    debug!("Finding build directory");
    let Some(build_directory) = current_dir.ancestors().find(|path| {
        trace!("Checking directory: {:?}", path);
        path.join(BUILD_FILE_NAME).exists()
    }) else {
        error!("No build file found.");
        info!(
            "Try adding a file named '{}' in the project root.",
            BUILD_FILE_NAME
        );
        std::process::exit(1);
    };

    info!("Resolved project root to {:?}", build_directory);
    build_directory.to_path_buf()
}
