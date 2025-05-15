use crate::command_run;
use crate::library::commands::command_runner::{call_shell, ExecuteCommandOptions};
use std::path::Path;

pub(crate) fn command_arg_maybe_fvm(pwd: Option<&Path>) -> Option<String> {
    should_use_fvm(pwd).then(|| "fvm".to_owned())
}

fn should_use_fvm(pwd: Option<&Path>) -> bool {
    if pwd.is_some() && !has_fvmrc(pwd.unwrap()) {
        false
    } else {
        let has_fvm_installation_output = has_fvm_installation();
        if !has_fvm_installation_output {
            log::info!("Has .fvmrc but no fvm binary installation, thus skip using fvm.");
        }
        has_fvm_installation_output
    }
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

#[allow(clippy::vec_init_then_push)]
fn has_fvm_installation() -> bool {
    command_run!(call_shell[None, Some(ExecuteCommandOptions { log_when_error: Some(false), ..Default::default() })], "fvm", "--version")
        .map_or(false, |res| res.status.success())
}
