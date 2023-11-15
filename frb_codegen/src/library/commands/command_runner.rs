use std::path::PathBuf;
use std::process::Output;

pub(crate) fn execute_command<'a>(
    bin: &str,
    args: impl IntoIterator<Item = &'a PathBuf>,
    current_dir: Option<&str>,
) -> CommandResult<Output> {
    todo!()
}
