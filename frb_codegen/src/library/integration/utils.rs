use crate::utils::path_utils::path_to_string;
use anyhow::Result;
use include_dir::{Dir, DirEntry};
use log::debug;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Recursively copies the file system structure starting at `current_reference_dir`
/// to `base_target_path` while filtering by `filter` and invoking `modifier` on each file.
/// Any target path content
pub(super) fn overlay_dir(
    current_reference_dir: &Dir,
    replacements: &HashMap<&str, String>,
    base_target_path: &Path,
    modifier: &impl Fn(&Path, &[u8], Option<Vec<u8>>) -> Option<(PathBuf, Vec<u8>)>,
    filter: &impl Fn(&Path) -> bool,
) -> Result<()> {
    for entry in current_reference_dir.entries() {
        let target_sub_path = base_target_path.join(entry.path());

        if !filter(&target_sub_path) {
            continue;
        }

        let target_sub_path = compute_effective_path(&target_sub_path, replacements);
        match entry {
            DirEntry::Dir(new_reference_dir) => {
                if let Some((modified_path, _)) = modifier(&target_sub_path, &[], None) {
                    debug!("Create dir {modified_path:?}");
                    fs::create_dir_all(&modified_path)?;
                    overlay_dir(
                        new_reference_dir,
                        replacements,
                        base_target_path,
                        modifier,
                        filter,
                    )?;
                }
            }
            DirEntry::File(file) => {
                let reference_content = file.contents();
                let existing_content = fs::read(&target_sub_path).ok();
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

pub(crate) fn compute_effective_path(path: &Path, replacements: &HashMap<&str, String>) -> PathBuf {
    replace_string_content(&path_to_string(path).unwrap(), replacements).into()
}

pub(crate) fn replace_file_content(
    content: &[u8],
    replacements: &HashMap<&str, String>,
) -> Vec<u8> {
    match String::from_utf8(content.to_owned()) {
        Ok(string_content) => replace_string_content(&string_content, replacements).into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

pub(crate) fn replace_string_content(
    content: &str,
    replacements: &HashMap<&str, String>,
) -> String {
    let mut result = content.to_string();

    for (key, value) in replacements {
        result = result.replace(key, value);
    }

    result
}
