use crate::codegen::dumper::Dumper;
use crate::codegen::ConfigDumpContent;
use crate::library::commands::command_runner::execute_command;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use log::{debug, info, warn};
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

        if !manifest_dir.is_empty() && rust_crate_dir == PathBuf::from(manifest_dir) {
            warn!(
                "Skip cargo-expand on {rust_crate_dir:?}, \
             because cargo is already running and would block cargo-expand. \
             This might cause errors if your api contains macros."
            );
            return Ok(fs::read_to_string(rust_file_path)?);
        }

        let expanded = match self.cache.entry(rust_crate_dir.to_owned()) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(run_cargo_expand(rust_crate_dir, dumper, true)?),
        };

        extract_module(expanded, module)
    }
}

fn extract_module(raw_expanded: &str, module: Option<String>) -> Result<String> {
    if let Some(module) = module {
        let (_, extracted) =
            module
                .split("::")
                .fold((0, raw_expanded), |(spaces, expanded), module| {
                    // empty module scenario
                    if expanded.contains(&format!("mod {module} {{}}")) {
                        return (spaces, "");
                    }

                    // non-empty
                    let searched = format!("mod {module} {{\n");
                    let start = expanded
                        .find(&searched)
                        .map(|n| n + searched.len())
                        .unwrap_or_default();
                    if start == 0 {
                        return (spaces, expanded);
                    }
                    let end = expanded[start..]
                        .find(&format!("\n{}}}", " ".repeat(spaces)))
                        .map(|n| n + start)
                        .unwrap_or(expanded.len());
                    (spaces + 4, &expanded[start..end])
                });
        return Ok(extracted.to_owned());
    }
    Ok(raw_expanded.to_owned())
}

fn run_cargo_expand(
    rust_crate_dir: &Path,
    dumper: &Dumper,
    _allow_auto_install: bool,
) -> Result<String> {
    debug!("Running cargo expand in '{rust_crate_dir:?}'");
    let args = vec![
        PathBuf::from("expand"),
        PathBuf::from("--lib"),
        PathBuf::from("--theme=none"),
        PathBuf::from("--ugly"),
    ];

    let output = execute_command("cargo", &args, Some(rust_crate_dir))
        .with_context(|| format!("Could not expand rust code at path {rust_crate_dir:?}"))?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if stdout.is_empty() {
        if stderr.contains("no such command: `expand`") {
            info!("Cargo expand is not installed. Automatically install and re-run.");
            install_cargo_expand()?;
            return run_cargo_expand(rust_crate_dir, dumper, false);
        }
        bail!("cargo expand returned empty output");
    }

    let mut stdout_lines = stdout.lines();
    stdout_lines.next();
    let ans = stdout_lines.join("\n").replace("/// frb_marker: ", "");

    dumper.dump_str(ConfigDumpContent::Source, "cargo_expand.rs", &ans)?;

    Ok(ans)
}

fn install_cargo_expand() -> Result<()> {
    execute_command(
        "cargo",
        &vec!["install".into(), "cargo-expand".into()],
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
}
