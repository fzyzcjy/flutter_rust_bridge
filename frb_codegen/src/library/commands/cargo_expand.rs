use crate::codegen::dumper::Dumper;
use crate::codegen::ConfigDumpContent;
use crate::command_args;
use crate::library::commands::command_runner::execute_command;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{debug, info};
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pub(crate) fn run_cargo_expand(rust_crate_dir: &Path, dumper: &Dumper) -> Result<syn::File> {
    let text = run_with_frb_aware(rust_crate_dir)?;
    dumper.dump_str(ConfigDumpContent::Source, "cargo_expand.rs", &text)?;
    Ok(syn::parse_file(&text)?)
}

fn run_with_frb_aware(rust_crate_dir: &Path) -> Result<String> {
    Ok(unwrap_frb_attrs_in_doc(&run_raw(rust_crate_dir, "--cfg frb_expand", true)?).into_owned())
}

/// Turns `#[doc = "frb_marker: .."]` back into `#[frb(..)]`, usually produced
/// as a side-effect of cargo-expand.
// NOTE: The amount of pounds must match exactly with the implementation in frb_macros
fn unwrap_frb_attrs_in_doc(code: &str) -> Cow<str> {
    lazy_static! {
        static ref PATTERN: Regex =
            Regex::new(r####"#\[doc =[\s\n]*r###"frb_marker: ([\s\S]*?)"###]"####).unwrap();
    }
    PATTERN.replace_all(code, "$1")
}

#[allow(clippy::vec_init_then_push)]
fn run_raw(
    rust_crate_dir: &Path,
    extra_rustflags: &str,
    allow_auto_install: bool,
) -> Result<String> {
    // let _pb = simple_progress("Run cargo-expand".to_owned(), 1);
    debug!("Running cargo expand in '{rust_crate_dir:?}'");

    let args = command_args!("expand", "--lib", "--theme=none", "--ugly");

    let mut extra_env: HashMap<_, _> = [(
        "RUSTFLAGS".to_owned(),
        env::var("RUSTFLAGS").map(|x| x + " ").unwrap_or_default() + extra_rustflags,
    ),
        (
            Skipper
            )
    ]
    .into();
    if let Some(cargo_target_dir) = compute_cargo_target_dir() {
        extra_env.insert("CARGO_TARGET_DIR".to_owned(), cargo_target_dir);
    }

    let output = execute_command("cargo", &args, Some(rust_crate_dir), Some(extra_env))
        .with_context(|| format!("Could not expand rust code at path {rust_crate_dir:?}"))?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if stdout.is_empty() {
        if stderr.contains("no such command: `expand`") && allow_auto_install {
            info!("Cargo expand is not installed. Automatically install and re-run.");
            install_cargo_expand()?;
            return run_raw(rust_crate_dir, extra_rustflags, false);
        }
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("cargo expand returned empty output");
        // frb-coverage:ignore-end
    }

    Ok(stdout.lines().skip(1).join("\n"))
}

fn compute_cargo_target_dir() -> Option<String> {
    // Please refer to https://github.com/mozilla/cbindgen/blob/3cbb637bbf16c7378ce4d6cb4b73e5d2d2bd33fa/src/bindgen/cargo/cargo_expand.rs#L81-L87
    // to see how this handles the case when called in build.rs
    if let Ok(ref path) = env::var("OUT_DIR") {
        let ans = PathBuf::from(path).join("frb_expanded");
        log::debug!("compute_cargo_target_dir choose ans={ans:?} since see OUT_DIR");
        Some(ans.to_str().unwrap().to_string())
    } else {
        None
    }
}

fn install_cargo_expand() -> Result<()> {
    execute_command(
        "cargo",
        &vec!["install".into(), "cargo-expand".into()],
        None,
        None,
    )?;
    Ok(())
}
