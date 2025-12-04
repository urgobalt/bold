use std::path::PathBuf;

use clap::Args;

use crate::cli::GlobalFlags;
use crate::compiler::{self, Compiler, CompilerFlags, CompilerType};
use crate::{project_configuration, project_root};

#[derive(Debug, Clone, Args)]
pub struct BuildAction {
    /// Name of build file
    #[arg(long, env = "BOLD_BUILD_FILE", default_value_t = crate::DEFAULT_BUILD_FILE_NAME.into())]
    pub build_file: String,
    /// Filter for te compiler resolver
    #[arg(long)]
    pub compiler_filter: Vec<CompilerType>,
    /// Manually select a compiler to use for building
    #[arg(long, env = "BOLD_COMPILER")]
    pub compiler: Option<PathBuf>,
    /// Manually select a compiler type for your compiler
    #[arg(long, env = "BOLD_COMPILER_TYPE", requires = "compiler")]
    pub compiler_type: Option<CompilerType>,
}

impl CompilerFlags for BuildAction {
    fn compiler(&self) -> &Option<PathBuf> {
        &self.compiler
    }

    fn compiler_type(&self) -> &Option<CompilerType> {
        &self.compiler_type
    }

    fn compiler_filter(&self) -> &Vec<CompilerType> {
        &self.compiler_filter
    }
}

#[derive(Debug)]
pub struct BuildEnvironment {
    project_root: PathBuf,
    build_file: PathBuf,
    compiler: Compiler,
}

impl BuildEnvironment {
    fn new(project_root: PathBuf, compiler: Compiler) -> Self {
        Self {
            build_file: project_root.join(crate::DEFAULT_BUILD_FILE_NAME),
            project_root: project_root,
            compiler,
        }
    }
}

pub fn build_environment(
    build_action: &BuildAction,
    global_flags: &GlobalFlags,
) -> BuildEnvironment {
    let project_root = project_root::find_project_root(&build_action.build_file);
    let toml_arguments = project_configuration::get_toml_arguments(&project_root, &global_flags);
    let compiler = compiler::choose_c_compiler(build_action);

    BuildEnvironment::new(project_root, compiler)
}
