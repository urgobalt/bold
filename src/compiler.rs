use clap::ValueEnum;
use colored::Colorize;
use log::{debug, error, info, trace};
use std::path::PathBuf;
use strum::Display;

use crate::COMPILERS;
use crate::user_arguments::BuildAction;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, PartialOrd, Display)]
#[strum(serialize_all = "lowercase")]
pub enum CompilerType {
    Clang,
    GCC,
}

impl CompilerType {
    pub fn infer_compiler_type(value: &str) -> Option<Self> {
        match value {
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Compiler {
    compiler_type: CompilerType,
    compiler_path: PathBuf,
}

pub fn choose_c_compiler(build_action: &BuildAction) -> Compiler {
    if let Some(compiler_path) = build_action.compiler.clone() {
        if compiler_path.is_file() {
            debug!("Choosing user specified compiler {:?}", &compiler_path);
            if let Some(compiler_type) = build_action.compiler_type {
                return Compiler {
                    compiler_type,
                    compiler_path,
                };
            } else if let Some(compiler_name) = compiler_path.file_stem().unwrap().to_str()
                && let Some(compiler_type) = CompilerType::infer_compiler_type(compiler_name)
            {
                return Compiler {
                    compiler_type,
                    compiler_path,
                };
            }
            error!(
                "Unable to infer compiler type of manually specified compiler {:?}",
                &compiler_path
            );
        } else {
            error!("Compiler {:?} is not a file", &compiler_path);
        }
        std::process::exit(1);
    }

    let compilers;
    if build_action.compiler_filter.is_empty() {
        compilers = Vec::from(COMPILERS);
    } else {
        compilers = COMPILERS
            .iter()
            .filter_map(|compiler| {
                for compiler_filter in &build_action.compiler_filter {
                    if compiler_filter == compiler {
                        return Some(compiler.clone());
                    }
                }
                None
            })
            .collect::<Vec<CompilerType>>();
    }
    find_c_compiler(compilers)
}

#[cfg(target_family = "unix")]
pub fn find_c_compiler(compilers: Vec<CompilerType>) -> Compiler {
    let path_var = match std::env::var_os("PATH") {
        Some(path_var) => path_var,
        None => {
            error!(
                r#"
            Unable to get PATH variable.
            Required for compilation and executing commands.
            "#
            );
            std::process::exit(1);
        }
    };

    debug!("Searching for C compiler in PATH");
    let mut compiler: Option<(PathBuf, CompilerType)> = None;
    'compiler_path: for path_dir in std::env::split_paths(&path_var) {
        trace!("Searching in directory: {:?}", path_dir);
        for compiler_name in &compilers {
            let candidate = path_dir.join(compiler_name.to_string());
            if candidate.is_file() {
                info!("Found C compiler in path: {:?}", candidate);
                compiler = Some((candidate, compiler_name.clone()));
                break 'compiler_path;
            }
        }
    }

    if let Some(compiler) = compiler {
        return Compiler {
            compiler_path: compiler.0,
            compiler_type: compiler.1,
        };
    }

    error!("No suitable C compiler found on the system.");
    compiler_installation_hint();
    std::process::exit(1);
}

pub fn compiler_installation_hint() {
    let mut compiler_messages = Vec::new();
    for compiler in crate::COMPILERS {
        let help_message = match compiler {
            CompilerType::Clang => format!(
                "'{}' - get started at '{}'",
                compiler, "https://clang.llvm.org/get_started.html"
            ),
            CompilerType::GCC => format!(
                "'{}' - get started at '{}'",
                compiler, "https://gcc.gnu.org/install/"
            ),
        };
        compiler_messages.push(help_message);
    }
    println!(
        r#"
{}

{}"#,
        "Please install a valid C compiler and make sure it is added to PATH.".yellow(),
        compiler_messages.join("\n").yellow()
    );
}
