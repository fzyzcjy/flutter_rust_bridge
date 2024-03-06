mod frb_dart_source_code;
mod frb_rust_source_code;

use crate::internal::frb_dart_source_code::generate_frb_dart_source_code;
use crate::internal::frb_rust_source_code::generate_frb_rust_source_code;
use log::info;
use std::env;
use std::path::PathBuf;

pub fn generate() -> anyhow::Result<()> {
    let repo_base_dir = compute_repo_base_dir()?;
    info!("Determine repo_base_dir={repo_base_dir:?}");

    generate_frb_rust_source_code(&repo_base_dir)?;
    generate_frb_dart_source_code(&repo_base_dir)?;

    Ok(())
}

fn compute_repo_base_dir() -> anyhow::Result<PathBuf> {
    Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .parent()
        .unwrap()
        .to_owned())
}
