use log::{debug, error, info, trace};
use std::io::Write;
use std::path::PathBuf;
mod logging;
fn main() {
    logging::init_logger();
    let build_environment = get_build_environment();
}

const BUILD_FILE_NAME: &str = "build.c";

struct BuildEnvironment {
    project_root: PathBuf,
    build_file: PathBuf,
}

impl BuildEnvironment {
    fn new(project_root: PathBuf) -> Self {
        Self {
            build_file: project_root.join(BUILD_FILE_NAME),
            project_root: project_root,
        }
    }
}

fn get_build_environment() -> BuildEnvironment {
    let build_directory = find_project_root();

    BuildEnvironment::new(build_directory)
}

fn find_project_root() -> PathBuf {
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
