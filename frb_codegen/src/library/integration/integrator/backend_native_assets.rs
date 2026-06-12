use crate::utils::dart_repository::get_dart_package_name;
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

pub(super) fn remove_native_assets_plugin_scaffold(dart_root: &Path) -> Result<()> {
    let dart_package_name = get_dart_package_name(dart_root)?;

    remove_flutter_plugin_stanza(dart_root)?;
    remove_example_ffi_plugin_registration(dart_root, &dart_package_name)?;

    for relative_path in ["android", "ios", "linux", "macos", "windows", "ohos"] {
        let path = dart_root.join(relative_path);
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
    }

    Ok(())
}

fn remove_example_ffi_plugin_registration(dart_root: &Path, dart_package_name: &str) -> Result<()> {
    for relative_path in [
        "example/linux/flutter/generated_plugins.cmake",
        "example/windows/flutter/generated_plugins.cmake",
    ] {
        let path = dart_root.join(relative_path);
        if !path.exists() {
            continue;
        }

        let text_raw = std::fs::read_to_string(&path)?;
        let plugin_line = format!("  {dart_package_name}\n");
        let text_output = text_raw.replace(&plugin_line, "");
        std::fs::write(&path, text_output)?;
    }

    Ok(())
}

fn remove_flutter_plugin_stanza(dart_root: &Path) -> Result<()> {
    let path = dart_root.join("pubspec.yaml");
    let text_raw = std::fs::read_to_string(&path)?;
    let plugin_stanza = Regex::new(r"(?m)^  plugin:\n(?:    .*\n)+")?;
    let orphan_hook_dependency = Regex::new(r"(?m)^  hooks: .*\n")?;
    let text_without_plugin = plugin_stanza.replace(&text_raw, "").to_string();
    let text_output = orphan_hook_dependency
        .replace(&text_without_plugin, "")
        .to_string();
    std::fs::write(&path, text_output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::remove_native_assets_plugin_scaffold;
    use std::fs;

    #[test]
    fn test_remove_native_assets_plugin_scaffold_removes_flutter_plugin_stanza() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dart_root = temp_dir.path();
        fs::write(
            dart_root.join("pubspec.yaml"),
            r#"name: sample
dependencies:
  flutter:
    sdk: flutter
  hooks: ^1.0.3

flutter:
  plugin:
    platforms:
      android:
        ffiPlugin: true
      ios:
        ffiPlugin: true

  uses-material-design: true
"#,
        )
        .unwrap();
        fs::create_dir(dart_root.join("android")).unwrap();
        fs::create_dir(dart_root.join("ios")).unwrap();
        fs::create_dir_all(dart_root.join("example/linux/flutter")).unwrap();
        fs::write(
            dart_root.join("example/linux/flutter/generated_plugins.cmake"),
            r#"list(APPEND FLUTTER_FFI_PLUGIN_LIST
  sample
  other_plugin
)
"#,
        )
        .unwrap();

        remove_native_assets_plugin_scaffold(dart_root).unwrap();

        let text = fs::read_to_string(dart_root.join("pubspec.yaml")).unwrap();
        let generated_plugins =
            fs::read_to_string(dart_root.join("example/linux/flutter/generated_plugins.cmake"))
                .unwrap();
        assert!(!text.contains("hooks:"));
        assert!(!text.contains("plugin:"));
        assert!(text.contains("flutter:"));
        assert!(text.contains("uses-material-design"));
        assert!(!generated_plugins.contains("  sample\n"));
        assert!(generated_plugins.contains("  other_plugin\n"));
        assert!(!dart_root.join("android").exists());
        assert!(!dart_root.join("ios").exists());
    }
}
