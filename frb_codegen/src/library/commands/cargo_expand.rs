use std::{env, fs};
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use log::{info, warn};
use anyhow::{bail, Result};
use itertools::Itertools;

pub(crate) fn cargo_expand(rust_crate_dir: &Path, module: Option<String>, rust_file_path: &Path) -> Result<String> {
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")?.into();

    if !manifest_dir.is_empty() && rust_crate_dir == manifest_dir {
        warn!(
            "Skip cargo-expand on {rust_crate_dir}, \
             because cargo is already running and would block cargo-expand. \
             This might cause errors if your api contains macros."
        );
        return Ok(fs::read_to_string(file)?);
    }

    let mut cache = CARGO_EXPAND_CACHE.lock().unwrap();
    let expanded = cache
        .entry(String::from(rust_crate_dir))
        .or_insert_with(|| run_cargo_expand(rust_crate_dir));

    extract_module(expanded, module)
}

lazy_static! {
    static ref CARGO_EXPAND_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn extract_module(raw_expanded: &str, module: Option<String>) -> Result<String> {
    if let Some(module) = module {
        let (_, extracted) = module
            .split("::")
            .fold((0, raw_expanded), |(spaces, expanded), module| {
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

fn run_cargo_expand(rust_crate_dir: &Path) -> String {
    info!("Running cargo expand in '{rust_crate_dir}'");
    let args = vec![
        PathBuf::from("expand"),
        PathBuf::from("--theme=none"),
        PathBuf::from("--ugly"),
    ];

    let output = execute_command("cargo", &args, Some(rust_crate_dir))
        .with_context(|| format!("Could not expand rust code at path {:?}: {}\n", rust_crate_dir, e))?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if stdout.is_empty() {
        if stderr.contains("no such command: `expand`") {
            bail!("cargo expand is not installed. Please run  `cargo install cargo-expand`");
        }
        bail!("cargo expand returned empty output");
    }

    let mut stdout_lines = stdout.lines();
    stdout_lines.next();
    stdout_lines.join("\n").replace("/// frb_marker: ", "")
}

#[cfg(test)]
mod tests {
    use super::extract_module;

    #[test]
    pub fn extract_mod() {
        let src = "mod module_1 {
    // code 1
}
mod module_2 {
    // code 2
}";
        let extracted = extract_module(src, Some(String::from("module_1")));
        assert_eq!(String::from("    // code 1"), extracted);
        let extracted = extract_module(src, Some(String::from("module_2")));
        assert_eq!(String::from("    // code 2"), extracted);
    }

    #[test]
    pub fn extract_submod() {
        let src = "mod module {
    mod submodule {
        // sub code
    }
}";
        let extracted = extract_module(src, Some(String::from("module::submodule")));
        assert_eq!(String::from("        // sub code"), extracted);
    }
}
