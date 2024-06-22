use anyhow::Result;
use include_dir::{Dir, DirEntry};
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::path_utils::path_to_string;

/// Recursively copies the file system structure starting at `current_reference_dir`
/// to `base_target_path` while filtering by `filter` and invoking `modifier` on each file.
pub(super) fn extract_dir_and_modify(
    current_reference_dir: &Dir,
    replace_path_content_config: &ReplaceContentConfig,
    base_target_path: &Path,
    modifier: &impl Fn(&Path, &[u8], Option<Vec<u8>>) -> Option<(PathBuf, Vec<u8>)>,
    filter: &impl Fn(&Path) -> bool,
) -> Result<()> {
    for entry in current_reference_dir.entries() {
        let target_sub_path = base_target_path.join(entry.path());

        if !filter(&target_sub_path) {
            continue;
        }

        match entry {
            DirEntry::Dir(new_reference_dir) => {
                if let Some((modified_path, _)) = modifier(&target_sub_path, &[], None) {
                    debug!("Create dir {modified_path:?}");
                    fs::create_dir_all(&modified_path)?;
                    extract_dir_and_modify(
                        new_reference_dir,
                        replace_path_content_config,
                        base_target_path,
                        modifier,
                        filter,
                    )?;
                }
            }
            DirEntry::File(file) => {
                let reference_content = file.contents();
                let existing_content = fs::read(&compute_effective_path(
                    &target_sub_path,
                    replace_path_content_config,
                ))
                .ok();
                if let Some((modified_path, modified_data)) =
                    modifier(&target_sub_path, reference_content, existing_content)
                {
                    debug!("Write to {modified_path:?}");
                    fs::write(&modified_path, modified_data)?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) struct ReplaceContentConfig<'a> {
    pub dart_package_name: &'a str,
    pub rust_crate_name: &'a str,
    pub rust_crate_dir: &'a str,
    pub enable_local_dependency: bool,
}

pub(crate) fn compute_effective_path(path: &Path, config: &ReplaceContentConfig) -> PathBuf {
    replace_string_content(&path_to_string(&path).unwrap(), config).into()
}

pub(crate) fn replace_file_content(content: &[u8], config: &ReplaceContentConfig) -> Vec<u8> {
    match String::from_utf8(content.to_owned()) {
        Ok(string_content) => replace_string_content(&string_content, config).into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

pub(crate) fn replace_string_content(content: &str, config: &ReplaceContentConfig) -> String {
    content
        .replace("REPLACE_ME_DART_PACKAGE_NAME", config.dart_package_name)
        .replace("REPLACE_ME_RUST_CRATE_NAME", config.rust_crate_name)
        .replace("REPLACE_ME_RUST_CRATE_DIR", config.rust_crate_dir)
        .replace("REPLACE_ME_FRB_VERSION", env!("CARGO_PKG_VERSION"))
        .replace(
            "REPLACE_ME_RUST_FRB_DEPENDENCY",
            &if config.enable_local_dependency {
                r#"{ path = "../../../frb_rust" }"#.to_owned()
            } else {
                format!(r#""={}""#, env!("CARGO_PKG_VERSION"))
            },
        )
}
