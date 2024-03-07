use crate::codegen::dumper::Dumper;
use crate::codegen::ConfigDumpContent;
use crate::command_args;
use crate::library::commands::command_runner::execute_command;
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use regex::Regex;
use std::borrow::Cow;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Default)]
pub(crate) struct CachedCargoExpand {
    cache: HashMap<PathBuf, String>,
}

impl CachedCargoExpand {
    pub(crate) fn execute(
        &mut self,
        rust_crate_dir: &Path,
        module: Option<String>,
        rust_file_path: &Path,
        dumper: &Dumper,
    ) -> Result<String> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        debug!("CachedCargoExpand execute manifest_dir={manifest_dir} rust_crate_dir={rust_crate_dir:?}");

        if !manifest_dir.is_empty()
            && normalize_windows_unc_path(&path_to_string(rust_crate_dir)?)
                == normalize_windows_unc_path(&manifest_dir)
        {
            // We do not care about this warning message
            // frb-coverage:ignore-start
            warn!(
                "Skip cargo-expand on {rust_crate_dir:?}, \
             because cargo is already running and would block cargo-expand. \
             This might cause errors if your api contains macros."
            );
            // frb-coverage:ignore-end
            return Ok(fs::read_to_string(rust_file_path)?);
        }

        let expanded = match self.cache.entry(rust_crate_dir.to_owned()) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(run_cargo_expand_with_frb_aware(rust_crate_dir, dumper)?),
        };

        extract_module(expanded, module)
    }
}

fn extract_module(raw_expanded: &str, module: Option<String>) -> Result<String> {
    if let Some(module) = module {
        let mut spaces = 0;
        let mut expanded = raw_expanded;

        for module in module.split("::") {
            if expanded.contains(&format!("mod {module} {{}}")) {
                return Ok("".to_owned());
            }

            let indent = " ".repeat(spaces);
            let searched = regex::Regex::new(&format!(
                "(?m)^{indent}(?:pub(?:\\([^\\)]+\\))?\\s+)?mod {module} \\{{$"
            ))
            .unwrap();
            let start = match searched.find(expanded) {
                Some(m) => m.end() + 1,
                None => bail!("Module not found: {}", module),
            };
            let end = expanded[start..]
                .find(&format!("\n{}}}", indent))
                .map(|n| n + start)
                .unwrap_or(expanded.len());
            spaces += 4;
            expanded = &expanded[start..end];
        }
        return Ok(expanded.to_owned());
    }
    Ok(raw_expanded.to_owned())
}

fn run_cargo_expand_with_frb_aware(rust_crate_dir: &Path, dumper: &Dumper) -> Result<String> {
    Ok(unwrap_frb_attrs_in_doc(&run_cargo_expand(
        rust_crate_dir,
        "--cfg frb_expand",
        dumper,
        true,
    )?)
    .into_owned())
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

fn run_cargo_expand(
    rust_crate_dir: &Path,
    extra_rustflags: &str,
    dumper: &Dumper,
    allow_auto_install: bool,
) -> Result<String> {
    // let _pb = simple_progress("Run cargo-expand".to_owned(), 1);
    debug!("Running cargo expand in '{rust_crate_dir:?}'");

    let args = command_args!("expand", "--lib", "--theme=none", "--ugly");

    let output = execute_command(
        "cargo",
        &args,
        Some(rust_crate_dir),
        Some(
            [(
                "RUSTFLAGS".to_owned(),
                env::var("RUSTFLAGS").map(|x| x + " ").unwrap_or_default() + extra_rustflags,
            )]
            .into(),
        ),
    )
    .with_context(|| format!("Could not expand rust code at path {rust_crate_dir:?}"))?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if stdout.is_empty() {
        if stderr.contains("no such command: `expand`") && allow_auto_install {
            info!("Cargo expand is not installed. Automatically install and re-run.");
            install_cargo_expand()?;
            return run_cargo_expand(rust_crate_dir, extra_rustflags, dumper, false);
        }
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("cargo expand returned empty output");
        // frb-coverage:ignore-end
    }

    let ans = stdout.lines().skip(1).join("\n");
    dumper.dump_str(ConfigDumpContent::Source, "cargo_expand.rs", &ans)?;
    Ok(ans)
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

#[cfg(test)]
mod tests {
    use super::extract_module;

    #[test]
    pub fn test_extract_module_simple() {
        let src = "mod module_1 {
    // code 1
}
mod module_2 {
    // code 2
}";
        let extracted = extract_module(src, Some(String::from("module_1"))).unwrap();
        assert_eq!(String::from("    // code 1"), extracted);
        let extracted = extract_module(src, Some(String::from("module_2"))).unwrap();
        assert_eq!(String::from("    // code 2"), extracted);
    }

    #[test]
    pub fn test_extract_module_submod() {
        let src = "mod module {
    mod submodule {
        // sub code
    }
}";
        let extracted = extract_module(src, Some(String::from("module::submodule"))).unwrap();
        assert_eq!(String::from("        // sub code"), extracted);
    }

    #[test]
    pub fn test_extract_module_empty_submod() {
        let src = "pub mod api {
    // some code
}
mod another {}";
        let extracted = extract_module(src, Some(String::from("another"))).unwrap();
        assert_eq!(String::from(""), extracted);
    }

    #[test]
    pub fn test_extract_module_with_prefix() {
        let src = "pub mod parent {
    mod another {
        // some code
    }
}";
        let extracted = extract_module(src, Some(String::from("another")));
        assert!(extracted.is_err());
    }

    #[test]
    pub fn test_extract_module_with_same_name() {
        let src = "pub mod parent {
    mod another {
        // some code
    }
}
pub(self) mod another {
    // 12345
}                                                                                                                                      ";
        let extracted = extract_module(src, Some(String::from("another"))).unwrap();
        assert_eq!(String::from("    // 12345"), extracted);
    }
}
