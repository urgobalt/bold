use log::{debug, error, info, trace};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

mod compiler;
mod logging;
mod project_root;
mod user_arguments;

use compiler::Compiler;

const DEFAULT_BUILD_FILE_NAME: &str = "build.c";
const DEFAULT_BUILD_CONFIG_FILE_NAME: &str = "bold.toml";

fn main() {
    logging::init_logger();
    let build_environment = get_build_environment();
    debug!("{:?}", build_environment);
}

#[derive(Debug)]
struct BuildEnvironment {
    project_root: PathBuf,
    build_file: PathBuf,
    compiler: Compiler,
}

impl BuildEnvironment {
    fn new(project_root: PathBuf, compiler: Compiler) -> Self {
        Self {
            build_file: project_root.join(DEFAULT_BUILD_FILE_NAME),
            project_root: project_root,
            compiler,
        }
    }
}

fn get_build_environment() -> BuildEnvironment {
    let project_root = project_root::find_project_root();
    let user_arguments = user_arguments::get_user_arguments(&project_root);
    let compiler = compiler::find_c_compiler();

    BuildEnvironment::new(project_root, compiler)
}
