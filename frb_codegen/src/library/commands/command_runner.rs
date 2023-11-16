use crate::library::commands::error::Error;
use std::path::{Path, PathBuf};
use std::process::Output;

pub(crate) type CommandResult<T = (), E = Error> = Result<T, E>;

pub(crate) fn execute_command<'a>(
    _bin: &str,
    _args: impl IntoIterator<Item = &'a PathBuf>,
    _current_dir: Option<&Path>,
) -> CommandResult<Output> {
    todo!()
}
