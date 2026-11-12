use super::IntegrateConfig;
use crate::misc::Template;
use crate::utils::dart_repository::dart_repo::DartRepository;
use anyhow::{Context, Result};
use serde_yaml::{Mapping, Value};
use std::fs;
use std::path::Path;

const OHOS_RUST_TARGETS: [&str; 3] = [
    "aarch64-unknown-linux-ohos",
    "x86_64-unknown-linux-ohos",
    "armv7-unknown-linux-ohos",
];

const OHOS_HOOK_DEPENDENCY: &str = r#"
git:
  url: https://github.com/star4277/native_toolchain_rust_ohos.git
  path: frb_hooks_ohos
  ref: main
"#;

const OHOS_DEPENDENCY_OVERRIDES: &str = r#"
hooks:
  git:
    url: https://gitcode.com/CPF-Flutter/fluttertpc_dart_native.git
    path: pkgs/hooks
    ref: flutter_3.47.7_deps
code_assets:
  git:
    url: https://gitcode.com/CPF-Flutter/fluttertpc_dart_native.git
    path: pkgs/code_assets
    ref: flutter_3.47.7_deps
"#;

pub(super) fn ohos_rust_targets_template(enabled: bool) -> String {
    if !enabled {
        return String::new();
    }
    OHOS_RUST_TARGETS
        .iter()
        .map(|target| format!("  \"{target}\",\n"))
        .collect()
}

pub(super) fn configure_ohos_native_assets(
    dart_root: &Path,
    config: &IntegrateConfig,
) -> Result<()> {
    configure_package_pubspec(dart_root, true)?;
    if config.template == Template::Plugin {
        // Overrides in a dependency are ignored by an independently resolved example app.
        configure_package_pubspec(&dart_root.join("example"), false)?;
    }
    let toolchain_path = dart_root
        .join(&config.rust_crate_dir)
        .join("rust-toolchain.toml");
    if toolchain_path.exists() {
        add_ohos_rust_targets(&toolchain_path)?;
    }
    Ok(())
}

fn configure_package_pubspec(package_root: &Path, add_hook_dependency: bool) -> Result<()> {
    let workspace_root = DartRepository::from_path(package_root)?.workspace_root;
    let package_manifest = package_root.join("pubspec.yaml");
    let root_manifest = workspace_root.join("pubspec.yaml");
    update_pubspec(
        &package_manifest,
        add_hook_dependency,
        package_manifest == root_manifest,
    )?;
    if package_manifest != root_manifest {
        update_pubspec(&root_manifest, false, true)?;
    }
    Ok(())
}

fn update_pubspec(path: &Path, add_hook_dependency: bool, root_overrides: bool) -> Result<()> {
    let mut manifest: Value = serde_yaml::from_str(&fs::read_to_string(path)?)
        .with_context(|| format!("Cannot parse {path:?}"))?;
    let original = manifest.clone();
    if add_hook_dependency {
        yaml_section(&mut manifest, "dependencies")?.insert(
            Value::String("flutter_rust_bridge_hooks".to_owned()),
            serde_yaml::from_str(OHOS_HOOK_DEPENDENCY)?,
        );
        if let Some(dev_dependencies) = manifest
            .get_mut("dev_dependencies")
            .filter(|section| !section.is_null())
        {
            dev_dependencies
                .as_mapping_mut()
                .context("dev_dependencies must be a mapping")?
                .remove(Value::String("flutter_rust_bridge_hooks".to_owned()));
        }
    }
    if root_overrides {
        let overrides: Mapping = serde_yaml::from_str(OHOS_DEPENDENCY_OVERRIDES)?;
        let root = yaml_section(&mut manifest, "dependency_overrides")?;
        root.extend(overrides);
        let hook_package = Value::String("flutter_rust_bridge_hooks".to_owned());
        if root.contains_key(&hook_package) {
            root.insert(hook_package, serde_yaml::from_str(OHOS_HOOK_DEPENDENCY)?);
        }
    } else if let Some(overrides) = manifest
        .get_mut("dependency_overrides")
        .filter(|section| !section.is_null())
    {
        let overrides = overrides
            .as_mapping_mut()
            .context("dependency_overrides must be a mapping")?;
        // A workspace dependency may only be overridden once, at the consumer root.
        for package in ["hooks", "code_assets", "flutter_rust_bridge_hooks"] {
            overrides.remove(Value::String(package.to_owned()));
        }
    }
    if manifest != original {
        fs::write(path, serde_yaml::to_string(&manifest)?)?;
    }
    Ok(())
}

fn yaml_section<'a>(manifest: &'a mut Value, section: &str) -> Result<&'a mut Mapping> {
    let value = manifest
        .as_mapping_mut()
        .context("pubspec.yaml must be a mapping")?
        .entry(Value::String(section.to_owned()))
        .or_insert_with(|| Value::Mapping(Mapping::new()));
    if value.is_null() {
        *value = Value::Mapping(Mapping::new());
    }
    value
        .as_mapping_mut()
        .with_context(|| format!("{section} must be a mapping"))
}

fn add_ohos_rust_targets(path: &Path) -> Result<()> {
    let mut manifest: toml::Value = toml::from_str(&fs::read_to_string(path)?)
        .with_context(|| format!("Cannot parse {path:?}"))?;
    let original = manifest.clone();
    let targets = manifest
        .get_mut("toolchain")
        .and_then(toml::Value::as_table_mut)
        .context("rust-toolchain.toml must contain a [toolchain] table")?
        .entry("targets".to_owned())
        .or_insert_with(|| toml::Value::Array(Vec::new()))
        .as_array_mut()
        .context("toolchain.targets must be an array")?;
    anyhow::ensure!(
        targets.iter().all(|target| target.as_str().is_some()),
        "toolchain.targets must contain only strings"
    );
    for target in OHOS_RUST_TARGETS {
        let target = toml::Value::String(target.to_owned());
        if !targets.contains(&target) {
            targets.push(target);
        }
    }
    if manifest != original {
        fs::write(path, toml::to_string_pretty(&manifest)?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::{FvmInstallMode, IntegrationBackend};

    fn config(template: Template) -> IntegrateConfig {
        IntegrateConfig {
            enable_write_lib: true,
            enable_integration_test: false,
            enable_dart_fix: false,
            enable_dart_format: false,
            enable_local_dependency: true,
            rust_crate_name: None,
            rust_crate_dir: "native/rust".to_owned(),
            template,
            integration_backend: IntegrationBackend::NativeAssets,
            platforms: Some("android".to_owned()),
            fvm_install_mode: FvmInstallMode::Skip,
        }
    }

    fn read_yaml(path: &Path) -> Value {
        serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn configures_hook_dependencies_targets_and_preserves_custom_settings() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        fs::write(
            root.join("pubspec.yaml"),
            "name: demo\nenvironment:\n  sdk: ^3.13.0\ndependencies:\n  flutter_rust_bridge:\n    path: ../../frb_dart\n  collection: ^1.0.0\ndev_dependencies:\n  flutter_rust_bridge_hooks: any\ndependency_overrides:\n  hooks: ^2.0.2\n  other_package:\n    path: ../other_package\n",
        )
        .unwrap();
        let rust_root = root.join("native/rust");
        fs::create_dir_all(&rust_root).unwrap();
        fs::write(
            rust_root.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"1.93.1\"\nprofile = \"minimal\"\ncomponents = [\"rustfmt\"]\ntargets = [\"aarch64-unknown-linux-ohos\", \"x86_64-unknown-linux-gnu\"]\n",
        )
        .unwrap();
        configure_ohos_native_assets(root, &config(Template::App)).unwrap();

        let pubspec = read_yaml(&root.join("pubspec.yaml"));
        assert_eq!(pubspec["environment"]["sdk"], "^3.13.0");
        assert_eq!(pubspec["dependencies"]["collection"], "^1.0.0");
        assert_eq!(
            pubspec["dependencies"]["flutter_rust_bridge"]["path"],
            "../../frb_dart"
        );
        assert_eq!(
            pubspec["dependencies"]["flutter_rust_bridge_hooks"],
            serde_yaml::from_str::<Value>(OHOS_HOOK_DEPENDENCY).unwrap()
        );
        assert!(pubspec["dev_dependencies"]
            .get("flutter_rust_bridge_hooks")
            .is_none());
        let expected_overrides: Mapping = serde_yaml::from_str(OHOS_DEPENDENCY_OVERRIDES).unwrap();
        for (package, dependency) in expected_overrides {
            assert_eq!(pubspec["dependency_overrides"][package], dependency);
        }
        assert_eq!(
            pubspec["dependency_overrides"]["other_package"]["path"],
            "../other_package"
        );

        let toolchain: toml::Value =
            toml::from_str(&fs::read_to_string(rust_root.join("rust-toolchain.toml")).unwrap())
                .unwrap();
        assert_eq!(toolchain["toolchain"]["channel"].as_str(), Some("1.93.1"));
        assert_eq!(toolchain["toolchain"]["profile"].as_str(), Some("minimal"));
        assert_eq!(
            toolchain["toolchain"]["components"][0].as_str(),
            Some("rustfmt")
        );
        let targets = toolchain["toolchain"]["targets"].as_array().unwrap();
        assert_eq!(targets.len(), 4);
        assert!(targets
            .iter()
            .any(|target| target.as_str() == Some("x86_64-unknown-linux-gnu")));
        for target in OHOS_RUST_TARGETS {
            assert!(targets.iter().any(|value| value.as_str() == Some(target)));
        }

        let before_pubspec = fs::read(root.join("pubspec.yaml")).unwrap();
        let before_toolchain = fs::read(rust_root.join("rust-toolchain.toml")).unwrap();
        configure_ohos_native_assets(root, &config(Template::App)).unwrap();
        assert_eq!(fs::read(root.join("pubspec.yaml")).unwrap(), before_pubspec);
        assert_eq!(
            fs::read(rust_root.join("rust-toolchain.toml")).unwrap(),
            before_toolchain
        );
    }

    #[test]
    fn workspace_overrides_are_moved_to_root_and_example_has_its_own_overrides() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let package = root.join("package");
        let example = package.join("example");
        fs::create_dir_all(&example).unwrap();
        fs::write(
            root.join("pubspec.yaml"),
            "name: workspace\nworkspace: [package]\n",
        )
        .unwrap();
        fs::write(
            package.join("pubspec.yaml"),
            "name: package\nresolution: workspace\ndependency_overrides:\n  hooks: any\n  code_assets: any\n  other_package: any\n",
        )
        .unwrap();
        fs::write(
            example.join("pubspec.yaml"),
            "name: example\ndependencies:\n  package:\n    path: ../\n",
        )
        .unwrap();
        configure_ohos_native_assets(&package, &config(Template::Plugin)).unwrap();

        let root_pubspec = read_yaml(&root.join("pubspec.yaml"));
        let package_pubspec = read_yaml(&package.join("pubspec.yaml"));
        let example_pubspec = read_yaml(&example.join("pubspec.yaml"));
        for name in ["hooks", "code_assets"] {
            assert!(package_pubspec["dependency_overrides"].get(name).is_none());
            assert_eq!(
                root_pubspec["dependency_overrides"][name],
                example_pubspec["dependency_overrides"][name]
            );
            assert!(root_pubspec["dependency_overrides"][name]["git"].is_mapping());
        }
        assert_eq!(
            package_pubspec["dependency_overrides"]["other_package"],
            "any"
        );
        assert!(package_pubspec["dependencies"]["flutter_rust_bridge_hooks"]["git"].is_mapping());
        assert!(root_pubspec.get("dependencies").is_none());
        assert!(example_pubspec["dependencies"]
            .get("flutter_rust_bridge_hooks")
            .is_none());
    }

    #[test]
    fn creates_targets_when_existing_toolchain_has_none() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("rust-toolchain.toml");
        fs::write(&path, "[toolchain]\nchannel = \"1.94.0\"\n").unwrap();
        add_ohos_rust_targets(&path).unwrap();
        let toolchain: toml::Value = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(toolchain["toolchain"]["channel"].as_str(), Some("1.94.0"));
        assert_eq!(
            toolchain["toolchain"]["targets"].as_array().unwrap().len(),
            3
        );
    }

    #[test]
    fn accepts_empty_sections_and_replaces_an_existing_hook_override() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pubspec.yaml");
        fs::write(
            &path,
            "name: demo\ndependencies:\ndev_dependencies:\ndependency_overrides:\n",
        )
        .unwrap();
        update_pubspec(&path, true, true).unwrap();
        let mut pubspec = read_yaml(&path);
        pubspec["dependency_overrides"]["flutter_rust_bridge_hooks"] =
            serde_yaml::from_str("path: ../../frb_hooks").unwrap();
        fs::write(&path, serde_yaml::to_string(&pubspec).unwrap()).unwrap();
        update_pubspec(&path, true, true).unwrap();
        let pubspec = read_yaml(&path);
        assert_eq!(
            pubspec["dependency_overrides"]["flutter_rust_bridge_hooks"],
            pubspec["dependencies"]["flutter_rust_bridge_hooks"]
        );
    }

    #[test]
    fn malformed_manifests_are_reported_without_replacing_them() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pubspec.yaml");
        for original in ["dependencies: [broken", "dependencies: invalid"] {
            fs::write(&path, original).unwrap();
            assert!(update_pubspec(&path, true, true).is_err());
            assert_eq!(fs::read_to_string(&path).unwrap(), original);
        }
        let path = temp.path().join("rust-toolchain.toml");
        for original in ["[toolchain", "[toolchain]\ntargets = \"invalid\"\n"] {
            fs::write(&path, original).unwrap();
            assert!(add_ohos_rust_targets(&path).is_err());
            assert_eq!(fs::read_to_string(&path).unwrap(), original);
        }
    }
}
