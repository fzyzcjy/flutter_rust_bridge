use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::Template;
use log::info;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn flutter_create(
    name: &str,
    org: &Option<String>,
    template: Template,
    platforms: Option<String>,
    skip_fvm_install: bool,
) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(None, skip_fvm_install));
    full_args.extend(vec![
        "flutter".to_owned(),
        "create".to_owned(),
        name.to_owned(),
    ]);
    let platforms = platforms.unwrap_or(format!(
        "android,ios,linux,macos,windows{}{}",
        if is_ohos_flutter().unwrap_or(false) {
            ",ohos"
        } else {
            ""
        },
        if matches!(template, Template::Plugin) {
            ",web"
        } else {
            ""
        }
    ));
    if let Some(o) = org {
        full_args.extend(["--org".to_owned(), o.to_owned()]);
    }
    match template {
        Template::App => full_args.extend([
            "--template".to_owned(),
            "app".to_owned(),
            "--platforms".to_owned(),
            platforms,
        ]),
        Template::Plugin => full_args.extend([
            "--template".to_owned(),
            "plugin_ffi".to_owned(),
            "--platforms".to_owned(),
            platforms,
        ]),
    }

    info!("Execute `{}` (this may take a while)", full_args.join(" "));
    check_exit_code(&command_run!(call_shell[None, None], *full_args)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_add(
    items: &[&str],
    pwd: Option<&Path>,
    skip_fvm_install: bool,
) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(pwd, skip_fvm_install));
    full_args.extend(vec![
        "flutter".to_owned(),
        "pub".to_owned(),
        "add".to_owned(),
    ]);
    full_args.extend(items.iter().map(|x| x.to_string()));

    info!(
        "Execute `{}` inside {pwd:?} (this may take a while)",
        full_args.join(" ")
    );
    check_exit_code(&command_run!(call_shell[pwd, None], *full_args)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_get(path: &Path, skip_fvm_install: bool) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(Some(path), skip_fvm_install));
    full_args.extend(vec![
        "flutter".to_owned(),
        "pub".to_owned(),
        "get".to_owned(),
    ]);

    info!(
        "Execute `{}` inside {path:?} (this may take a while)",
        full_args.join(" ")
    );
    check_exit_code(&command_run!(call_shell[Some(path), None], *full_args)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn is_ohos_flutter() -> anyhow::Result<bool> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(None, true));
    full_args.extend(vec![
        "flutter".to_owned(),
        "create".to_owned(),
        "--help".to_owned(),
    ]);
    info!("Execute `{}` (this may take a while)", full_args.join(" "));
    let out = command_run!(call_shell[None, None], *full_args)?;
    if !out.status.success() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        let msg = String::from_utf8_lossy(&out.stderr);
        anyhow::bail!("Command execution failed: {msg}");
    }
    let res = String::from_utf8_lossy(&out.stdout);
    let is_ohos = res.to_lowercase().contains("ohos");
    if is_ohos {
        info!("current flutter support ohos platform.")
    }
    Ok(is_ohos)
}
