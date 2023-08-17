use std::sync::Arc;
use thiserror::Error;

pub type Result<T = (), E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug, Clone)]
#[repr(transparent)]
#[error(transparent)]
pub struct Error(Arc<ErrorImpl>);

#[derive(Error, Debug)]
pub(crate) enum ErrorImpl {
    #[error("External command failure.\n{0}")]
    Command(#[from] crate::commands::Error),

    #[error("Configuration error.\n{0}")]
    Config(#[from] crate::config::Error),

    #[error("Parser failure.\n{0}")]
    Parser(#[from] crate::parser::Error),

    #[error("I/O failure.\n{0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Uncategorized(#[from] anyhow::Error),
}

impl<E> From<E> for Error
where
    ErrorImpl: From<E>,
{
    fn from(value: E) -> Self {
        Error(Arc::new(value.into()))
    }
}
