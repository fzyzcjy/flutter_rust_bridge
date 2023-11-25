use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Context;
use log::debug;
use std::path::PathBuf;
use std::{env, fs};

pub fn build(dart_root: Option<PathBuf>, args: Vec<String>) -> anyhow::Result<()> {
    let dart_root = parse_dart_root(dart_root)?;
    debug!("build dart_root={dart_root:?} args={args:?}");

    todo!()
}

fn parse_dart_root(dart_root: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    dart_root
        .map(|x| Ok(fs::canonicalize(x)?))
        .unwrap_or_else(|| {
            find_dart_package_dir(&env::current_dir()?)
                .context("Please provide --dart-root, or run command inside a Flutter/Dart package")
        })
}
