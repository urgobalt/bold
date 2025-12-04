use log::{debug, error, info, trace};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

mod build;
mod cli;
mod compiler;
mod init;
mod logging;
mod project_configuration;
mod project_root;
mod compile;

use compiler::CompilerType;

use self::init::init_project;

const DEFAULT_BUILD_FILE_NAME: &str = "build.c";
const DEFAULT_BUILD_CONFIG_FILE_NAME: &str = "bold.toml";

#[cfg(target_family = "unix")]
const COMPILERS: [CompilerType; 2] = [CompilerType::Clang, CompilerType::GCC];

fn main() {
    let cli_arguments = cli::get_cli_arguments();
    logging::init_logger(cli_arguments.global_flags.log_level);
    debug!(
        "Set log level to '{}'",
        cli_arguments.global_flags.log_level
    );
    debug!("Selecting action '{}'", &cli_arguments.action);
    match cli_arguments.action {
        cli::Action::Init(init_action) => {
            init_project(init_action, &cli_arguments.global_flags);
        }
        cli::Action::Build(build_action) => {
            let build_environment =
                build::build_environment(&build_action, &cli_arguments.global_flags);
            debug!("{:?}", build_environment);
        }
        cli::Action::Compile(compile_action) => todo!(),
    }
}
