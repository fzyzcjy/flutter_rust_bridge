use crate::utils::path_utils::find_dart_package_dir;
use log::debug;
use std::env;
use std::path::PathBuf;

pub fn build(dart_root: Option<PathBuf>, args: Vec<String>) -> anyhow::Result<()> {
    let dart_root = dart_root.map(Ok).unwrap_or_else(|| fallback_dart_root())?;
    debug!("build dart_root={dart_root:?} args={args:?}");
    todo!()
}

fn fallback_dart_root() -> anyhow::Result<PathBuf> {
    find_dart_package_dir(&env::current_dir()?)
}
