use crate::codegen::dumper::Dumper;
use crate::codegen::ConfigDumpContent;
use crate::command_args;
use crate::library::commands::command_runner::{
    check_exit_code, execute_command, ExecuteCommandOptions,
};
use crate::utils::crate_name::CrateName;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{debug, info};
use regex::{Captures, Regex};
use std::borrow::Cow;
use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const CARGO_EXPAND_FALLBACK_VERSION: &str = "1.0.118";

pub(super) fn run(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    dumper: &Dumper,
    features: Option<&[String]>,
) -> Result<syn::File> {
    let text = run_with_frb_aware(rust_crate_dir, interest_crate_name, features)?;
    (dumper.with_content(ConfigDumpContent::Source)).dump_str("cargo_expand.rs", &text)?;
    Ok(syn::parse_file(&text)?)
}

fn run_with_frb_aware(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    features: Option<&[String]>,
) -> Result<String> {
    Ok(decode_macro_frb_encoded_comments(&run_raw(
        rust_crate_dir,
        interest_crate_name,
        "--cfg frb_expand",
        true,
        features,
    )?)
    .into_owned())
}

/// Decode `#[doc = "frb_encoded(...)"]`, usually produced as a side-effect of cargo-expand.
fn decode_macro_frb_encoded_comments(code: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref PATTERN: Regex =
            Regex::new(r##"#\[doc =[\s\n]*"frb_encoded\(([\s\S]*?)\)"\]"##).unwrap();
    }

    PATTERN.replace_all(code, |captures: &Captures| {
        let hex_str = captures.get(1).unwrap().as_str();
        let decoded_str = String::from_utf8(hex::decode(hex_str).unwrap()).unwrap();
        decoded_str.to_string()
    })
}

#[allow(clippy::vec_init_then_push)]
fn run_raw(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    extra_rustflags: &str,
    allow_auto_install: bool,
    features: Option<&[String]>,
) -> Result<String> {
    // let _pb = simple_progress("Run cargo-expand".to_owned(), 1);
    debug!("Running cargo expand in '{rust_crate_dir:?}'");

    let args_choosing_crate = if let Some(interest_crate_name) = interest_crate_name {
        vec!["-p", interest_crate_name.raw()]
    } else {
        vec![]
    };

    let args_features: Vec<PathBuf> = features
        .unwrap_or_default()
        .iter()
        .flat_map(|feature| vec!["--features", feature])
        .map(PathBuf::from_str)
        .try_collect()?;

    let args = command_args!(
        "expand",
        "--lib",
        "--theme=none",
        "--ugly",
        *args_choosing_crate,
        *args_features
    );

    let extra_env = [(
        "RUSTFLAGS".to_owned(),
        env::var("RUSTFLAGS").map(|x| x + " ").unwrap_or_default() + extra_rustflags,
    )]
    .into();

    let output = execute_command(
        "cargo",
        &args,
        Some(rust_crate_dir),
        Some(ExecuteCommandOptions {
            envs: Some(extra_env),
            ..Default::default()
        }),
    )
    .with_context(|| format!("Could not expand rust code at path {rust_crate_dir:?}"))?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if stdout.is_empty() {
        if stderr.contains("no such command: `expand`") && allow_auto_install {
            info!("Cargo expand is not installed. Automatically install and re-run.");
            install_cargo_expand()?;
            return run_raw(
                rust_crate_dir,
                interest_crate_name,
                extra_rustflags,
                false,
                features,
            );
        }
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("cargo expand returned empty output");
        // frb-coverage:ignore-end
    }

    Ok(stdout.lines().skip(1).join("\n"))
}

fn install_cargo_expand() -> Result<()> {
    let latest = execute_command("cargo", &cargo_expand_install_args(None), None, None)?;
    if latest.status.success() {
        return Ok(());
    }

    let latest_stderr = String::from_utf8_lossy(&latest.stderr);
    if let Some(version) = cargo_expand_fallback_version(&latest_stderr) {
        info!(
            "Latest cargo-expand is incompatible with the active Rust toolchain. Falling back to cargo-expand {version}."
        );
        let fallback = execute_command(
            "cargo",
            &cargo_expand_install_args(Some(version)),
            None,
            None,
        )?;
        check_exit_code(&fallback)?;
        return Ok(());
    }

    check_exit_code(&latest)?;
    Ok(())
}

fn cargo_expand_fallback_version(stderr: &str) -> Option<&'static str> {
    stderr
        .contains("requires rustc")
        .then_some(CARGO_EXPAND_FALLBACK_VERSION)
}

fn cargo_expand_install_args(version: Option<&str>) -> Vec<PathBuf> {
    let mut args = command_args!("install", "cargo-expand");
    if let Some(version) = version {
        args.extend(command_args!("--version", version, "--locked"));
    }
    args
}

#[cfg(test)]
mod tests {
    use super::{cargo_expand_fallback_version, cargo_expand_install_args};

    #[test]
    fn test_cargo_expand_fallback_version_when_latest_requires_newer_rustc() {
        let stderr =
            "error: cannot install package `cargo-expand 1.0.121`, it requires rustc 1.88 or newer";
        assert_eq!(cargo_expand_fallback_version(stderr), Some("1.0.118"));
    }

    #[test]
    fn test_cargo_expand_fallback_version_when_error_is_unrelated() {
        assert_eq!(cargo_expand_fallback_version("network timeout"), None);
    }

    #[test]
    fn test_cargo_expand_install_args_for_fallback_uses_locked() {
        let args = cargo_expand_install_args(Some("1.0.118"));
        let args = args
            .into_iter()
            .map(|item| item.into_os_string().into_string().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            args,
            vec!["install", "cargo-expand", "--version", "1.0.118", "--locked"]
        );
    }
}

// fn extract_module(raw_expanded: &str, module: Option<String>) -> Result<String> {
//     if let Some(module) = module {
//         let mut spaces = 0;
//         let mut expanded = raw_expanded;
//
//         for module in module.split("::") {
//             if expanded.contains(&format!("mod {module} {{}}")) {
//                 return Ok("".to_owned());
//             }
//
//             let indent = " ".repeat(spaces);
//             let searched = regex::Regex::new(&format!(
//                 "(?m)^{indent}(?:pub(?:\\([^\\)]+\\))?\\s+)?mod {module} \\{{$"
//             ))
//             .unwrap();
//             let start = match searched.find(expanded) {
//                 Some(m) => m.end() + 1,
//                 // #1830
//                 None => return Ok("".to_owned()),
//             };
//             let end = expanded[start..]
//                 .find(&format!("\n{}}}", indent))
//                 .map(|n| n + start)
//                 .unwrap_or(expanded.len());
//             spaces += 4;
//             expanded = &expanded[start..end];
//         }
//         return Ok(expanded.to_owned());
//     }
//     Ok(raw_expanded.to_owned())
// }
//
// #[cfg(test)]
// mod tests {
//     use super::extract_module;
//
//     #[test]
//     pub fn test_extract_module_simple() {
//         let src = "mod module_1 {
//     // code 1
// }
// mod module_2 {
//     // code 2
// }";
//         let extracted = extract_module(src, Some(String::from("module_1"))).unwrap();
//         assert_eq!(String::from("    // code 1"), extracted);
//         let extracted = extract_module(src, Some(String::from("module_2"))).unwrap();
//         assert_eq!(String::from("    // code 2"), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_submod() {
//         let src = "mod module {
//     mod submodule {
//         // sub code
//     }
// }";
//         let extracted = extract_module(src, Some(String::from("module::submodule"))).unwrap();
//         assert_eq!(String::from("        // sub code"), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_empty_submod() {
//         let src = "pub mod api {
//     // some code
// }
// mod another {}";
//         let extracted = extract_module(src, Some(String::from("another"))).unwrap();
//         assert_eq!(String::from(""), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_with_prefix() {
//         let src = "pub mod parent {
//     mod another {
//         // some code
//     }
// }";
//         let extracted = extract_module(src, Some(String::from("another"))).unwrap();
//         assert_eq!(String::from(""), extracted);
//     }
//
//     #[test]
//     pub fn test_extract_module_with_same_name() {
//         let src = "pub mod parent {
//     mod another {
//         // some code
//     }
// }
// pub(self) mod another {
//     // 12345
// }                                                                                                                                      ";
//         let extracted = extract_module(src, Some(String::from("another"))).unwrap();
//         assert_eq!(String::from("    // 12345"), extracted);
//     }
// }
