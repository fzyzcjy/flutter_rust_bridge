use crate::codegen::config::internal_config_parser::dart_path_parser::compute_path_map;
use crate::codegen::generator::misc::target::TargetOrCommonMap;
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::codegen::Config;
use crate::utils::path_utils::{canonicalize_with_error_message, find_rust_crate_dir};
use anyhow::Context;
use std::path::{Path, PathBuf};

pub(super) fn compute_rust_crate_dir(config: &Config) -> anyhow::Result<PathBuf> {
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
