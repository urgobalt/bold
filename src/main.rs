use log::{debug, error, info, trace};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

mod compiler;
mod logging;
mod project_root;
mod user_arguments;

use compiler::{Compiler, CompilerType};

const DEFAULT_BUILD_FILE_NAME: &str = "build.c";
const DEFAULT_BUILD_CONFIG_FILE_NAME: &str = "bold.toml";

#[cfg(target_family = "unix")]
const COMPILERS: [CompilerType; 2] = [CompilerType::Clang, CompilerType::GCC];

fn main() {
    logging::init_logger();
    let build_environment = build_environment();
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

fn build_environment() -> BuildEnvironment {
    let cli_arguments = user_arguments::get_cli_arguments();
    let project_root = project_root::find_project_root(&cli_arguments.build_filename);
    log::set_max_level(cli_arguments.log_level.into());
    if cli_arguments.generate_config_file {
        user_arguments::generate_configuration_file(&project_root, &cli_arguments);
    }

    let toml_arguments = user_arguments::get_toml_arguments(&project_root, &cli_arguments);
    let compilers = COMPILERS
        .iter()
        .filter_map(|compiler| {
            for compiler_filter in &cli_arguments.compiler_filter {
                if compiler_filter == compiler {
                    return Some(compiler.clone());
                }
            }
            None
        })
        .collect::<Vec<CompilerType>>();
    let compiler = compiler::find_c_compiler(compilers);

    BuildEnvironment::new(project_root, compiler)
}
