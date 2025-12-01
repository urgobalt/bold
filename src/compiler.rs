use log::{debug, error, info, trace};
use std::path::PathBuf;

enum CompilerType {
    Clang,
    GCC,
}

pub struct Compiler {
    compiler_type: CompilerType,
    compiler_path: PathBuf,
}

impl From<&'static str> for CompilerType {
    #[cfg(target_family = "unix")]
    fn from(value: &'static str) -> Self {
        match value {
            "clang" => Self::Clang,
            "gcc" => Self::GCC,
            value => unimplemented!("Compiler {:?} not implemented", value),
        }
    }
}

#[cfg(target_family = "unix")]
const COMPILERS: [&str; 2] = ["clang", "gcc"];

#[cfg(target_family = "unix")]
pub fn find_c_compiler() -> Compiler {
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
        for compiler_name in COMPILERS {
            let candidate = path_dir.join(compiler_name);
            if candidate.is_file() {
                info!("Found C compiler in path: {:?}", candidate);
                compiler = Some((candidate, CompilerType::from(compiler_name)));
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

use compiler_hints::compiler_installation_hint;
mod compiler_hints {
    use super::*;
    pub fn compiler_installation_hint() {
        let mut compiler_messages = Vec::new();
        for compiler in COMPILERS {
            let help_message = match CompilerType::from(compiler) {
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
        info!(
            r#"Please install a valid C compiler and make sure it is added to PATH.

    {}"#,
            compiler_messages.join("\n")
        );
    }
}
