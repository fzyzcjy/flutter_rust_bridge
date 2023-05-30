use crate::utils::dart_repository::dart_repo::DartDependencyMode;
use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("rustfmt failed: {0}")]
    Rustfmt(String),
    #[error("dart fmt failed: {0}")]
    Dartfmt(String),
    #[error(
        "ffigen could not find LLVM.
    Please supply --llvm-path to flutter_rust_bridge_codegen, e.g.:

        flutter_rust_bridge_codegen .. --llvm-path <path_to_llvm>"
    )]
    FfigenLlvm,
    #[error("{0} is not a command, or not executable.")]
    MissingExe(String),
    #[error("{0}")]
    StringError(String),
    #[error("please add {name} to your {manager}. (version {requirement})")]
    MissingDep {
        name: String,
        manager: DartDependencyMode,
        requirement: String,
    },
    #[error("please update version of {name} in your {manager}. (version {requirement})")]
    InvalidDep {
        name: String,
        manager: DartDependencyMode,
        requirement: String,
    },
}

impl Error {
    pub fn string<T: Into<String>>(msg: T) -> Self {
        Self::StringError(msg.into())
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        if let Some(e) = e.downcast_ref::<Self>() {
            return e.clone();
        }
        Error::StringError(e.to_string())
    }
}
