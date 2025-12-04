use log::{debug, error, info, trace};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

mod compiler;
mod logging;
mod project_root;
mod user_arguments;

use compiler::{Compiler, CompilerType};

use self::user_arguments::{BuildAction, GlobalFlags};

const DEFAULT_BUILD_FILE_NAME: &str = "build.c";
const DEFAULT_BUILD_CONFIG_FILE_NAME: &str = "bold.toml";

#[cfg(target_family = "unix")]
const COMPILERS: [CompilerType; 2] = [CompilerType::Clang, CompilerType::GCC];

fn main() {
    logging::init_logger();
    let cli_arguments = user_arguments::get_cli_arguments();
    log::set_max_level(cli_arguments.global_flags.log_level.into());
    debug!(
        "Set log level to '{}'",
        cli_arguments.global_flags.log_level
    );
    debug!("Selecting action {:?}", &cli_arguments.action);
    match &cli_arguments.action {
        user_arguments::Action::Init(init_action) => user_arguments::generate_configuration_file(
            &PathBuf::new(),
            &cli_arguments.global_flags,
        ),
        user_arguments::Action::Build(build_action) => {
            let build_environment = build_environment(build_action, &cli_arguments.global_flags);
            debug!("{:?}", build_environment);
        }
        user_arguments::Action::Compile(compile_action) => todo!(),
    }
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

fn build_environment(build_action: &BuildAction, global_flags: &GlobalFlags) -> BuildEnvironment {
    let project_root = project_root::find_project_root(&build_action.build_file);
    let toml_arguments = user_arguments::get_toml_arguments(&project_root, &global_flags);
    let compiler = compiler::choose_c_compiler(build_action);

    BuildEnvironment::new(project_root, compiler)
}
