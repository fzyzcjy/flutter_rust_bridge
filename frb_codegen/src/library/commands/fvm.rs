use crate::command_run;
use crate::library::commands::command_runner::call_shell;
use std::path::Path;

pub(crate) fn command_arg_maybe_fvm(pwd: &Path) -> Option<String> {
    if should_use_fvm(pwd) {
        Some("fvm".to_owned())
    } else {
        None
    }
}

fn should_use_fvm(pwd: &Path) -> bool {
    has_fvmrc(pwd) && has_fvm_installation()
}

fn has_fvmrc(pwd: &Path) -> bool {
    let mut directory = pwd;
    loop {
        if directory.join(".fvmrc").exists() {
            return true;
        }
        if let Some(parent) = directory.parent() {
            directory = parent;
        } else {
            return false;
        }
    }
}

fn has_fvm_installation() -> bool {
    command_run!(call_shell[None, None], "fvm", "--version")
        .map_or(false, |res| res.status.success())
}
