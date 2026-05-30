use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::{FvmInstallMode, Template};
use log::info;
use std::path::Path;

pub(crate) struct FlutterPlatforms {
    pub(crate) value: String,
    pub(crate) include_ohos: bool,
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_create(
    name: &str,
    org: &Option<String>,
    template: Template,
    platforms: Option<String>,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(None, fvm_install_mode));
    full_args.extend(vec![
        "flutter".to_owned(),
        "create".to_owned(),
        name.to_owned(),
    ]);
    let platforms = resolve_flutter_platforms(template, platforms, fvm_install_mode)?;
    if let Some(o) = org {
        full_args.extend(["--org".to_owned(), o.to_owned()]);
    }
    match template {
        Template::App => full_args.extend([
            "--template".to_owned(),
            "app".to_owned(),
            "--platforms".to_owned(),
            platforms.value,
        ]),
        Template::Plugin => full_args.extend([
            "--template".to_owned(),
            "plugin_ffi".to_owned(),
            "--platforms".to_owned(),
            platforms.value,
        ]),
    }

    info!("Execute `{}` (this may take a while)", full_args.join(" "));
    check_exit_code(&command_run!(call_shell[None, None], *full_args)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_add(
    items: &[&str],
    pwd: Option<&Path>,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(pwd, fvm_install_mode));
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
pub fn flutter_pub_get(path: &Path, fvm_install_mode: FvmInstallMode) -> anyhow::Result<()> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(Some(path), fvm_install_mode));
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

pub(crate) fn resolve_flutter_platforms(
    template: Template,
    platforms: Option<String>,
    _fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<FlutterPlatforms> {
    if let Some(value) = platforms {
        return Ok(FlutterPlatforms {
            include_ohos: platform_list_contains_ohos(&value),
            value,
        });
    }

    let include_ohos = is_ohos_flutter().unwrap_or(false);
    Ok(FlutterPlatforms {
        value: default_flutter_platforms(template, include_ohos),
        include_ohos,
    })
}

#[allow(clippy::vec_init_then_push)]
pub fn is_ohos_flutter() -> anyhow::Result<bool> {
    let mut full_args = vec![];
    full_args.extend(command_arg_maybe_fvm(None, FvmInstallMode::Skip));
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
    let is_ohos = flutter_create_help_supports_platform(&res, "ohos");
    if is_ohos {
        info!("current flutter support ohos platform.")
    }
    Ok(is_ohos)
}

fn default_flutter_platforms(template: Template, include_ohos: bool) -> String {
    format!(
        "android,ios,linux,macos,windows{}{}",
        if include_ohos { ",ohos" } else { "" },
        if matches!(template, Template::Plugin) {
            ""
        } else {
            ",web"
        }
    )
}

fn platform_list_contains_ohos(platforms: &str) -> bool {
    platforms
        .split(',')
        .any(|platform| platform.trim().eq_ignore_ascii_case("ohos"))
}

fn flutter_create_help_supports_platform(help: &str, platform: &str) -> bool {
    let mut in_platforms_option = false;

    for line in help.lines() {
        let trimmed = line.trim_start();
        let starts_new_option =
            trimmed.starts_with("--") || trimmed.starts_with('-') && trimmed.contains("--");
        if in_platforms_option && starts_new_option && !trimmed.contains("--platforms") {
            return false;
        }

        if trimmed.contains("--platforms") {
            in_platforms_option = true;
        }

        if in_platforms_option && text_contains_platform_token(line, platform) {
            return true;
        }
    }

    false
}

fn text_contains_platform_token(text: &str, platform: &str) -> bool {
    text.split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .any(|token| token.eq_ignore_ascii_case(platform))
}

#[cfg(test)]
mod tests {
    use super::{
        default_flutter_platforms, flutter_create_help_supports_platform,
        platform_list_contains_ohos,
    };
    use crate::misc::Template;

    #[test]
    fn test_platform_list_contains_ohos_with_whitespace() {
        assert!(platform_list_contains_ohos("android, ohos"));
    }

    #[test]
    fn test_platform_list_contains_ohos_is_case_insensitive() {
        assert!(platform_list_contains_ohos("android,OHOS"));
    }

    #[test]
    fn test_platform_list_contains_ohos_rejects_other_platforms() {
        assert!(!platform_list_contains_ohos(
            "android,ios,linux,macos,windows"
        ));
    }

    #[test]
    fn test_default_flutter_platforms_for_app() {
        assert_eq!(
            default_flutter_platforms(Template::App, false),
            "android,ios,linux,macos,windows,web"
        );
        assert_eq!(
            default_flutter_platforms(Template::App, true),
            "android,ios,linux,macos,windows,ohos,web"
        );
    }

    #[test]
    fn test_default_flutter_platforms_for_plugin() {
        assert_eq!(
            default_flutter_platforms(Template::Plugin, false),
            "android,ios,linux,macos,windows"
        );
        assert_eq!(
            default_flutter_platforms(Template::Plugin, true),
            "android,ios,linux,macos,windows,ohos"
        );
    }

    #[test]
    fn test_flutter_create_help_supports_platform_on_platforms_line() {
        let help = "
Usage: flutter create <output directory>

    --platforms    The platforms supported by this project.
                   [android, ios, linux, macos, ohos, web, windows]
    --org          The organization responsible for your new Flutter project.
";

        assert!(flutter_create_help_supports_platform(help, "ohos"));
    }

    #[test]
    fn test_flutter_create_help_supports_platform_rejects_other_help_text() {
        let help = "
Usage: flutter create <output directory>

    --description  Create an app for the ohos store.
    --platforms    The platforms supported by this project.
                   [android, ios, linux, macos, web, windows]
    --org          The organization responsible for your new Flutter project.
";

        assert!(!flutter_create_help_supports_platform(help, "ohos"));
    }

    #[test]
    fn test_flutter_create_help_supports_platform_stops_at_next_option() {
        let help = "
Usage: flutter create <output directory>

    --platforms    The platforms supported by this project.
                   [android, ios, linux, macos, web, windows]
    --org          The organization responsible for your ohos project.
";

        assert!(!flutter_create_help_supports_platform(help, "ohos"));
    }
}
