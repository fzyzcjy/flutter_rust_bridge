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
    replacements: &HashMap<&str, &str>,
    base_target_path: &Path,
    modifier: &impl Fn(&Path, &[u8], Option<Vec<u8>>) -> Option<(PathBuf, Vec<u8>)>,
    filter: &impl Fn(&Path) -> bool,
) -> Result<()> {
    for entry in current_reference_dir.entries() {
        if !filter(entry.path()) {
            continue;
        }

        let target_sub_path = base_target_path.join(entry.path());

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

pub(crate) fn compute_effective_path(path: &Path, replacements: &HashMap<&str, &str>) -> PathBuf {
    replace_string_content(&path_to_string(path).unwrap(), replacements).into()
}

pub(crate) fn replace_file_content(content: &[u8], replacements: &HashMap<&str, &str>) -> Vec<u8> {
    match String::from_utf8(content.to_owned()) {
        Ok(string_content) => replace_string_content(&string_content, replacements).into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

pub(crate) fn replace_string_content(content: &str, replacements: &HashMap<&str, &str>) -> String {
    let mut result = content.to_string();

    for (key, value) in replacements {
        result = result.replace(key, value);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{compute_effective_path, overlay_dir, replace_file_content};
    use include_dir::{include_dir, Dir};
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    const TEMPLATE_DIR: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/cargokit/app");

    /// Replaces placeholders in a path without changing its path structure.
    #[test]
    fn compute_effective_path_replaces_components() {
        let replacements = HashMap::from([
            ("REPLACE_ME_RUST_CRATE_NAME", "my_crate"),
            (".template", ""),
        ]);

        assert_eq!(
            compute_effective_path(
                Path::new("target/REPLACE_ME_RUST_CRATE_NAME/Cargo.toml.template"),
                &replacements,
            ),
            PathBuf::from("target/my_crate/Cargo.toml"),
        );
    }

    /// Leaves binary file contents unchanged when they are not valid UTF-8.
    #[test]
    fn replace_file_content_preserves_invalid_utf8() {
        let content = [b'a', 0xff, b'b'];
        let replacements = HashMap::from([("a", "z")]);

        assert_eq!(replace_file_content(&content, &replacements), content);
    }

    /// Overlays recursively, filters entries, and lets modifiers skip and rename files.
    #[test]
    fn overlay_dir_applies_filter_modifier_and_existing_content() {
        let temp_dir = tempfile::tempdir().unwrap();
        let target = temp_dir.path();
        let existing_path = target.join("rust_builder/macos/Classes/dummy_file.c");
        fs::create_dir_all(existing_path.parent().unwrap()).unwrap();
        fs::write(&existing_path, "existing").unwrap();
        let replacements = HashMap::from([("REPLACE_ME_RUST_CRATE_NAME", "my_crate")]);

        overlay_dir(
            &TEMPLATE_DIR,
            &replacements,
            target,
            &|path, reference_content, existing_content| {
                if path.ends_with("run_build_tool.cmd") {
                    return None;
                }
                if path.ends_with("macos/Classes/dummy_file.c") {
                    assert_eq!(existing_content.as_deref(), Some(&b"existing"[..]));
                    return Some((path.with_file_name("renamed.c"), b"modified".to_vec()));
                }
                Some((path.to_path_buf(), reference_content.to_vec()))
            },
            &|path| !path.ends_with("build_tool"),
        )
        .unwrap();

        assert!(target.join("rust_builder/macos").is_dir());
        assert_eq!(
            fs::read(target.join("rust_builder/macos/Classes/renamed.c")).unwrap(),
            b"modified",
        );
        assert_eq!(fs::read(existing_path).unwrap(), b"existing");
        assert!(!target.join("rust_builder/cargokit/build_tool").exists());
        assert!(!target
            .join("rust_builder/cargokit/run_build_tool.cmd")
            .exists());
        assert!(target.join("rust_builder/ios/my_crate.podspec").exists());
    }
}
