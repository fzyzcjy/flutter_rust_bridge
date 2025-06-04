use crate::command_run;
use crate::library::commands::command_runner::{call_shell, check_exit_code};
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::Context;
use itertools::Itertools;
use log::debug;
use pathdiff::diff_paths;
use std::path::{Path, PathBuf};

pub fn format_rust(paths: &[PathBuf], base_path: &Path) -> anyhow::Result<()> {
    let paths = prepare_paths(paths, base_path, &[])?;
    debug!("execute format_rust paths={paths:?}");

    check_exit_code(&command_run!(
        call_shell[Some(base_path), None],
        "rustfmt",
        // otherwise cannot understand `async move`
        "--edition",
        "2018",
        *paths
    )?)
}

pub(super) fn prepare_paths(
    paths: &[PathBuf],
    base_path: &Path,
    extra_extensions: &[&str],
) -> anyhow::Result<Vec<PathBuf>> {
    let base_path_str = path_to_string(base_path)?;
    let normalized_base_path = normalize_windows_unc_path(&base_path_str);

    Ok(paths
        .iter()
        .map(|path| {
            let mut path: PathBuf = normalize_windows_unc_path(&path_to_string(path)?)
                .to_owned()
                .into();
            path = diff_paths(path, normalized_base_path).context("diff path")?;
            if path_to_string(&path)?.is_empty() {
                path = ".".into();
            }
            Ok(path)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|path| {
            vec![path.clone()].into_iter().chain(
                extra_extensions
                    .iter()
                    .map(move |ext| with_extension(path.clone(), ext))
                    .filter(|path| path.exists()),
            )
        })
        .collect_vec())
}

fn with_extension(mut path: PathBuf, ext: &str) -> PathBuf {
    path.set_extension(ext);
    path
}
