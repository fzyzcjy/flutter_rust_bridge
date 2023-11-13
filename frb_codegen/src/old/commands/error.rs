use crate::utils::dart_repository::dart_repo::DartDependencyMode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("rustfmt failed: {0}")]
    Rustfmt(String),
    #[error("dart fmt failed: {0}")]
    Dartfmt(String),
    #[error(
"ffigen could not find LLVM. Please supply --llvm-path to flutter_rust_bridge_codegen, e.g.:

    flutter_rust_bridge_codegen .. --llvm-path <path_to_llvm>"
    )]
    FfigenLlvm,
    #[error("{0} is not a command, or not executable.")]
    MissingExe(String),
    #[error(transparent)]
    Cbindgen(#[from] cbindgen::Error),
    #[error("Please add {name} to your {manager}. (version {requirement})")]
    MissingDep {
        name: String,
        manager: DartDependencyMode,
        requirement: String,
    },
    #[error("Please update version of {name} in your {manager}. (version {requirement})")]
    InvalidDep {
        name: String,
        manager: DartDependencyMode,
        requirement: String,
    },
    #[error("I/O failure.\n{0}")]
    Io(#[from] std::io::Error),
    #[error("Formatting failure.\n{0}")]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Uncategorized(#[from] anyhow::Error),
}
