use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::Template;
use log::info;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn flutter_create(name: &str, org: &Option<String>, template: Template) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(pwd));
    full_args.extend(vec!["flutter", "create", name]);
    if let Some(o) = org {
        full_args.extend(["--org", o]);
    }
    match template {
        Template::App => full_args.extend(["--template", "app"]),
        Template::Plugin => full_args.extend([
            "--template",
            "plugin_ffi",
            "--platforms",
            "android,ios,linux,macos,windows",
        ]),
    }

    info!("Execute `{}` (this may take a while)", full_args.join(" "));
    check_exit_code(&command_run!(call_shell[None, None], *full_args)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_add(items: &[&str], pwd: Option<&Path>) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(pwd));
    full_args.extend(vec!["flutter", "pub", "add"]);
    full_args.extend(items);

    info!(
        "Execute `{}` inside {pwd:?} (this may take a while)",
        full_args.join(" ")
    );
    check_exit_code(&command_run!(call_shell[pwd, None], *full_args)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_get(path: &Path) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(Some(path)));
    full_args.extend(vec!["flutter", "pub", "get"]);

    info!(
        "Execute `{}` inside {path:?} (this may take a while)",
        full_args.join(" ")
    );
    check_exit_code(&command_run!(call_shell[Some(path), None], *full_args)?)
}
