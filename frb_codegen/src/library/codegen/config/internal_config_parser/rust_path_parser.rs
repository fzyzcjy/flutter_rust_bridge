use crate::codegen::config::internal_config_parser::dart_path_parser::compute_path_map;
use crate::codegen::generator::misc::target::TargetOrCommonMap;
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::codegen::parser::internal_config::RustInputNamespacePack;
use crate::codegen::Config;
use crate::utils::path_utils::{canonicalize_with_error_message, find_rust_crate_dir, glob_path};
use anyhow::{ensure, Context};
use std::path::{Path, PathBuf};

pub(super) struct RustCrateDirAndInputPack {
    pub rust_crate_dir: PathBuf,
    pub rust_input_namespace_pack: RustInputNamespacePack,
}

pub(super) fn compute_rust_crate_dir_and_input_pack(
    config: &Config,
    base_dir: &PathBuf,
) -> anyhow::Result<RustCrateDirAndInputPack> {
    Ok(RustCrateDirAndInputPack {
        rust_crate_dir: compute_rust_crate_dir(config)?,
        rust_input_namespace_pack: compute_rust_input_namespace_pack(&config.rust_input, base_dir)?,
    })
}

fn compute_rust_input_namespace_pack(
    raw_rust_input: &str,
    base_dir: &Path,
) -> anyhow::Result<RustInputNamespacePack> {
    const BLACKLIST_FILE_NAMES: [&str; 1] = ["mod.rs"];

    let glob_pattern = base_dir.join(raw_rust_input);

    let mut pack = RustInputNamespacePack {
        rust_input_namespaces: vec![],
    };
    for path in glob_path(&glob_pattern)?.into_iter() {
        if BLACKLIST_FILE_NAMES.contains(&path.file_name().unwrap().to_str().unwrap()) {
            pack.rust_suppressed_input_namespaces.push(path);
        } else {
            pack.rust_input_namespaces.push(path);
        }
    }

    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    ensure!(
        !pack.rust_input_namespaces.is_empty(),
        "Find zero rust input paths. (glob_pattern={glob_pattern:?})"
    );
    // ensure!(
    //     !pack.rust_input_namespaces.iter().any(|p| path_to_string(p).unwrap().contains("lib.rs")),
    //     "Do not use `lib.rs` as a Rust input. Please put code to be generated in something like `api.rs`.",
    // );
    // frb-coverage:ignore-end

    Ok(pack)
}

fn compute_rust_crate_dir(config: &Config) -> anyhow::Result<PathBuf> {
    canonicalize_with_error_message(&(config.rust_root.clone().map(PathBuf::from)).unwrap_or(
        find_rust_crate_dir(rust_input_namespace_pack.one_rust_input_path())?,
    ))
}

pub(super) fn compute_rust_output_path(
    config: &Config,
    base_dir: &Path,
    rust_crate_dir: &Path,
) -> anyhow::Result<TargetOrCommonMap<PathBuf>> {
    let path_common = base_dir.join(
        (config.rust_output.clone().map(PathBuf::from))
            .unwrap_or_else(|| fallback_rust_output_path(rust_crate_dir)),
    );
    compute_path_map(&path_common).context("rust_output: is wrong: ")
}

fn fallback_rust_output_path(rust_crate_dir: &Path) -> PathBuf {
    rust_crate_dir.join("src").join("frb_generated.rs")
}

impl RustInputNamespacePack {
    fn one_rust_input_path(&self) -> &Path {
        self.rust_input_namespaces.first().unwrap()
    }
}
