use super::IntegrateConfig;
use crate::integration::utils::{overlay_dir, replace_file_content};
use crate::misc::{IntegrationBackend, Template};
use anyhow::Result;
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use log::warn;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

fn execute_overlay_dir(
    current_reference_dir: &Dir,
    replacements: &HashMap<&'static str, &str>,
    dart_root: &Path,
    config: &IntegrateConfig,
    comment_out_files: Option<&[String]>,
    include_ohos: bool,
) -> Result<()> {
    overlay_dir(
        current_reference_dir,
        replacements,
        dart_root,
        &|target_path, reference_content, existing_content| {
            modify_file(
                target_path.into(),
                reference_content,
                existing_content,
                replacements,
                config.enable_local_dependency,
                comment_out_files,
            )
        },
        &|path| {
            filter_file(
                path,
                config.enable_write_lib,
                config.enable_integration_test,
                include_ohos,
            )
        },
    )
}

pub(super) fn execute_overlay_templates(
    replacements: &HashMap<&'static str, &str>,
    dart_root: &Path,
    config: &IntegrateConfig,
    include_ohos: bool,
    dart_package_name: &str,
) -> Result<()> {
    execute_overlay_dir(
        &TemplateDirs::SHARED_SHARED,
        replacements,
        dart_root,
        config,
        None,
        include_ohos,
    )?;

    let (shared_template_dir, comment_out_files) = match &config.template {
        Template::App => (&TemplateDirs::SHARED_APP, vec!["main.dart".to_string()]),
        Template::Plugin => (
            &TemplateDirs::SHARED_PLUGIN,
            vec![format!("{dart_package_name}.dart")],
        ),
    };
    execute_overlay_dir(
        shared_template_dir,
        replacements,
        dart_root,
        config,
        Some(&comment_out_files),
        include_ohos,
    )?;

    if let Some(dir) = backend_shared_template_dir(config.integration_backend) {
        execute_overlay_dir(dir, replacements, dart_root, config, None, include_ohos)?;
    }

    if let Some(dir) = backend_template_dir(config.integration_backend, config.template) {
        execute_overlay_dir(dir, replacements, dart_root, config, None, include_ohos)?;
    }

    Ok(())
}

fn backend_shared_template_dir(
    integration_backend: IntegrationBackend,
) -> Option<&'static Dir<'static>> {
    match integration_backend {
        IntegrationBackend::Cargokit => Some(&TemplateDirs::CARGOKIT_SHARED),
        IntegrationBackend::NativeAssets => Some(&TemplateDirs::NATIVE_ASSETS_SHARED),
    }
}

fn backend_template_dir(
    integration_backend: IntegrationBackend,
    template: Template,
) -> Option<&'static Dir<'static>> {
    match (integration_backend, template) {
        (IntegrationBackend::Cargokit, Template::App) => Some(&TemplateDirs::CARGOKIT_APP),
        (IntegrationBackend::Cargokit, Template::Plugin) => Some(&TemplateDirs::CARGOKIT_PLUGIN),
        (IntegrationBackend::NativeAssets, Template::App) => None,
        (IntegrationBackend::NativeAssets, Template::Plugin) => {
            Some(&TemplateDirs::NATIVE_ASSETS_PLUGIN)
        }
    }
}

pub(super) fn compute_replacements<'a>(
    config: &'a IntegrateConfig,
    dart_package_name: &'a str,
    rust_crate_name: &'a str,
    include_ohos: bool,
) -> HashMap<&'static str, &'a str> {
    let mut replacements = HashMap::new();
    replacements.insert("REPLACE_ME_DART_PACKAGE_NAME", dart_package_name);
    replacements.insert("REPLACE_ME_RUST_CRATE_NAME", rust_crate_name);
    replacements.insert("REPLACE_ME_RUST_CRATE_DIR", config.rust_crate_dir.as_str());
    let (cargokit_hook_dir, cargokit_crate_prefix) = match config.template {
        Template::App => ("rust_builder/hook", "../"),
        Template::Plugin => ("hook", ""),
    };
    replacements.insert("REPLACE_ME_CARGOKIT_HOOK_DIR", cargokit_hook_dir);
    replacements.insert("REPLACE_ME_CARGOKIT_CRATE_PREFIX", cargokit_crate_prefix);
    replacements.insert("REPLACE_ME_FRB_VERSION", env!("CARGO_PKG_VERSION"));

    let rust_frb_dependency = if config.enable_local_dependency {
        r#"{ path = "../../../frb_rust" }"#
    } else {
        concat!(r#""="#, env!("CARGO_PKG_VERSION"), r#"""#)
    };
    replacements.insert("REPLACE_ME_RUST_FRB_DEPENDENCY", rust_frb_dependency);

    replacements.insert("Cargo.toml.template", "Cargo.toml");
    replacements.insert("Cargo.lock.template", "Cargo.lock");
    replacements.insert(
        "REPLACE_ME_OHOS_PLUGIN_PLATFORM_TEXT",
        if include_ohos {
            "\n      ohos:\n        ffiPlugin: true"
        } else {
            ""
        },
    );

    replacements
}

fn modify_file(
    target_path: PathBuf,
    reference_content: &[u8],
    existing_content: Option<Vec<u8>>,
    replacements: &HashMap<&str, &str>,
    enable_local_dependency: bool,
    comment_out_files: Option<&[String]>,
) -> Option<(PathBuf, Vec<u8>)> {
    let src = replace_file_content(reference_content, replacements);

    if let Some(existing_content) = existing_content {
        if let (Some(file_name), Some(files)) = (
            target_path.file_name().and_then(|e| e.to_str()),
            comment_out_files,
        ) {
            if files.contains(&file_name.to_owned()) {
                return comment_out_existing_file_and_write_template(
                    existing_content,
                    target_path,
                    &src,
                );
            }
        }
        // We do not care about this warning
        // frb-coverage:ignore-start
        warn!(
            "Skip writing to {target_path:?} because file already exists. \
            It is suggested to remove that file before running this command to apply the full template."
        );
        return None;
        // frb-coverage:ignore-end
    }

    if target_path.iter().contains(&OsStr::new("cargokit")) {
        if let Some(comments) = compute_cargokit_comments(&target_path) {
            return Some((target_path, [comments.as_bytes(), &src].concat()));
        }
    }

    if target_path
        .iter()
        .contains(&OsStr::new("flutter_rust_bridge.yaml"))
    {
        let mut ans = String::from_utf8(src).unwrap();
        if enable_local_dependency {
            ans += "\nlocal: true\n";
        }
        return Some((target_path, ans.as_bytes().to_owned()));
    }

    Some((target_path, src))
}

fn comment_out_existing_file_and_write_template(
    existing_content: Vec<u8>,
    path: PathBuf,
    src: &[u8],
) -> Option<(PathBuf, Vec<u8>)> {
    let existing_content = String::from_utf8(existing_content);
    let commented_existing_content = existing_content
        .map(|x| {
            format!(
                "// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.\n\n{}\n\n",
                x.split('\n').map(|line| format!("// {line}")).join("\n")
            )
        })
        .unwrap_or_default();
    Some((path, [commented_existing_content.as_bytes(), src].concat()))
}

fn filter_file(
    path: &Path,
    enable_write_lib: bool,
    enable_integration_test: bool,
    include_ohos: bool,
) -> bool {
    if path.iter().contains(&OsStr::new("ohos")) && !include_ohos {
        return false;
    }

    if path.iter().contains(&OsStr::new("cargokit")) {
        return ![".git", ".github", "docs", "test"].contains(&file_name(path));
    }

    if !enable_write_lib {
        if path.iter().contains(&OsStr::new("rust_builder")) {
            return true;
        }
        if path.iter().contains(&OsStr::new("android"))
            || path.iter().contains(&OsStr::new("ios"))
            || path.iter().contains(&OsStr::new("windows"))
            || path.iter().contains(&OsStr::new("macos"))
            || path.iter().contains(&OsStr::new("linux"))
            || path.iter().contains(&OsStr::new("ohos"))
            || path.iter().contains(&OsStr::new("lib"))
            || path
                .iter()
                .contains(&OsStr::new("REPLACE_ME_RUST_CRATE_DIR"))
            || path
                .iter()
                .contains(&OsStr::new("flutter_rust_bridge.yaml"))
        {
            return false;
        }
    }

    if !enable_integration_test
        && (path.iter().contains(&OsStr::new("integration_test"))
            || path.iter().contains(&OsStr::new("test_driver")))
    {
        return false;
    }

    true
}

fn compute_cargokit_comments(path: &Path) -> Option<String> {
    if [".gitignore"].contains(&file_name(path)) {
        return None;
    }

    let comment_leading = match file_extension(path) {
        "dart" | "md" | "gradle" | "" => "///",
        "yaml" | "toml" => "#",
        // Do not add prelude for `sh`, since it can contain things like `#!/bin/bash`
        // which must be at first line
        "lock" | "cmake" | "sh" | "ps1" | "cmd" => return None,
        // frb-coverage:ignore-start
        ext => unreachable!("unexpected file extension for path={:?} ext={}", path, ext),
        // frb-coverage:ignore-end
    };

    Some(
        (CARGOKIT_PRELUDE.iter())
            .map(|line| format!("{comment_leading} {line}"))
            .join("\n")
            + "\n\n",
    )
}

fn file_name(p: &Path) -> &str {
    p.file_name().unwrap().to_str().unwrap()
}

fn file_extension(p: &Path) -> &str {
    p.extension().unwrap_or_default().to_str().unwrap()
}

const CARGOKIT_PRELUDE: &[&str] = &[
    "This is copied from Cargokit (which is the official way to use it currently)", //
    "Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin",
];

struct TemplateDirs;

impl TemplateDirs {
    const SHARED_SHARED: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/shared/shared");
    const SHARED_APP: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/shared/app");
    const SHARED_PLUGIN: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/shared/plugin");
    const CARGOKIT_APP: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/cargokit/app");
    const CARGOKIT_SHARED: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/cargokit/shared");
    const CARGOKIT_PLUGIN: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/cargokit/plugin");
    const NATIVE_ASSETS_SHARED: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/native_assets/shared");
    const NATIVE_ASSETS_PLUGIN: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/native_assets/plugin");
}

#[cfg(test)]
mod tests {
    use super::{compute_replacements, execute_overlay_templates, filter_file, IntegrateConfig};
    use crate::misc::{FvmInstallMode, IntegrationBackend, Template};
    use std::{fs, path::Path};

    #[test]
    fn test_cargokit_apple_templates_support_swiftpm_and_cocoapods() {
        const DART_PACKAGE_NAME: &str = "example_plugin";
        const RUST_CRATE_NAME: &str = "example_crate";

        for template in [Template::App, Template::Plugin] {
            let output = tempfile::tempdir().unwrap();
            let config = IntegrateConfig {
                enable_write_lib: true,
                enable_integration_test: false,
                enable_dart_fix: false,
                enable_dart_format: false,
                enable_local_dependency: false,
                rust_crate_name: Some(RUST_CRATE_NAME.to_owned()),
                rust_crate_dir: "rust".to_owned(),
                template,
                integration_backend: IntegrationBackend::Cargokit,
                platforms: Some("ios,macos".to_owned()),
                fvm_install_mode: FvmInstallMode::Skip,
            };
            let replacements =
                compute_replacements(&config, DART_PACKAGE_NAME, RUST_CRATE_NAME, false);
            execute_overlay_templates(
                &replacements,
                output.path(),
                &config,
                false,
                DART_PACKAGE_NAME,
            )
            .unwrap();

            let (hook_path, platform_root, swift_package_name, expected_crate_path) = match template
            {
                Template::App => (
                    output.path().join("rust_builder/hook/build.dart"),
                    output.path().join("rust_builder"),
                    RUST_CRATE_NAME,
                    "../rust",
                ),
                Template::Plugin => (
                    output.path().join("hook/build.dart"),
                    output.path().to_owned(),
                    DART_PACKAGE_NAME,
                    "rust",
                ),
            };
            let hook = fs::read_to_string(hook_path).unwrap();
            assert!(!hook.contains("REPLACE_ME_"));
            assert!(hook.contains(&format!("const _cratePath = '{expected_crate_path}'")));
            assert!(hook.contains("FlutterRustBridgeNativeAssetsBuilder"));
            assert!(hook.contains("FlutterGeneratedPluginSwiftPackage"));
            assert!(hook.contains(".flutter-plugins-dependencies"));
            assert!(hook.contains("['Cargo.toml', 'Cargo.lock', 'build.rs']"));
            assert!(
                hook.contains("'IPHONEOS_DEPLOYMENT_TARGET': '${codeConfig.iOS.targetVersion}.0'")
            );

            for platform in ["ios", "macos"] {
                let package_dir = platform_root.join(platform).join(swift_package_name);
                let manifest = fs::read_to_string(package_dir.join("Package.swift")).unwrap();
                assert!(!manifest.contains("REPLACE_ME_"));
                assert!(!manifest.contains("BuildToolPlugin"));
                assert!(!manifest.contains("unsafeFlags"));
                assert!(manifest.contains(&format!("let productName = \"{swift_package_name}\"")));
                assert!(manifest.contains(".split(separator: \"_\")"));
                assert!(manifest.contains(".joined(separator: \"-\")"));
                assert!(manifest.contains(&format!("name: \"{RUST_CRATE_NAME}\"")));
                assert!(manifest.contains("name: productName"));
                assert!(manifest.contains(&format!("targets: [\"{RUST_CRATE_NAME}\"]")));
                assert!(package_dir.with_extension("podspec").is_file());
                assert!(package_dir
                    .join("Sources")
                    .join(RUST_CRATE_NAME)
                    .join("dummy.swift")
                    .is_file());
            }
        }
    }

    #[test]
    fn test_filter_file_excludes_ohos_when_not_enabled() {
        assert!(!filter_file(
            Path::new("rust_builder/ohos/src/main/module.json5"),
            true,
            true,
            false,
        ));
        assert!(!filter_file(
            Path::new("ohos/src/main/module.json5"),
            true,
            true,
            false,
        ));
    }

    #[test]
    fn test_filter_file_includes_ohos_when_enabled() {
        assert!(filter_file(
            Path::new("rust_builder/ohos/src/main/module.json5"),
            true,
            true,
            true,
        ));
        assert!(filter_file(
            Path::new("ohos/src/main/module.json5"),
            true,
            true,
            true,
        ));
    }

    #[test]
    fn test_filter_file_no_write_lib_excludes_enabled_ohos_platform_shell() {
        assert!(!filter_file(
            Path::new("ohos/src/main/module.json5"),
            false,
            true,
            true,
        ));
    }

    #[test]
    fn test_filter_file_excludes_cargokit_metadata() {
        assert!(!filter_file(
            Path::new("rust_builder/cargokit/.git"),
            true,
            true,
            true,
        ));
        assert!(!filter_file(
            Path::new("cargokit/.github"),
            true,
            true,
            true,
        ));
        assert!(!filter_file(Path::new("cargokit/docs"), true, true, true,));
        assert!(filter_file(
            Path::new("cargokit/build_tool/pubspec.yaml"),
            true,
            true,
            true,
        ));
    }
}
