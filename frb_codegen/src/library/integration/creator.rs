use crate::integration::integrator;
use crate::integration::integrator::IntegrateConfig;
use crate::library::commands::flutter::flutter_create;
use crate::misc::{FvmInstallMode, IntegrationBackend, Template};
use anyhow::{bail, ensure};
use log::{debug, info};
use regex::Regex;
use std::path::Path;
use std::{env, fs};

pub struct CreateConfig {
    pub name: String,
    pub org: Option<String>,
    pub enable_local_dependency: bool,
    pub rust_crate_name: Option<String>,
    pub rust_crate_dir: String,
    pub template: Template,
    pub integration_backend: IntegrationBackend,
    pub platforms: Option<String>,
    pub fvm_install_mode: FvmInstallMode,
}

/// Create a new Flutter + Rust project.
pub fn create(config: CreateConfig) -> anyhow::Result<()> {
    let dart_root = env::current_dir()?.join(&config.name);
    debug!(
        "create name={} org={:?} dart_root={dart_root:?}",
        config.name, config.org
    );

    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    ensure!(
        !dart_root.exists(),
        "The target folder {:?} already exists. Please use the `integrate` command in this case",
        dart_root,
    );
    // frb-coverage:ignore-end

    let platforms = config.platforms.clone();
    flutter_create(
        &config.name,
        &config.org,
        config.template,
        config.integration_backend,
        platforms.clone(),
        config.fvm_install_mode,
    )?;

    env::set_current_dir(&dart_root)?;

    match &config.template {
        Template::App => remove_unnecessary_app_files(&dart_root)?,
        Template::Plugin => {
            remove_unnecessary_plugin_files(&dart_root, config.integration_backend)?
        }
    }

    info!("Step: Inject flutter_rust_bridge related code");
    integrator::integrate(IntegrateConfig {
        enable_write_lib: true,
        enable_integration_test: true,
        enable_dart_fix: true,
        enable_dart_format: true,
        enable_local_dependency: config.enable_local_dependency,
        rust_crate_name: config.rust_crate_name,
        rust_crate_dir: config.rust_crate_dir,
        template: config.template,
        integration_backend: config.integration_backend,
        platforms,
        fvm_install_mode: config.fvm_install_mode,
    })
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn remove_unnecessary_app_files(dart_root: &Path) -> anyhow::Result<()> {
    // frb-coverage:ignore-end
    let lib_dir = dart_root.join("lib");
    remove_files_in_dir(&lib_dir)?;
    let test_dir = dart_root.join("test");
    remove_files_in_dir(&test_dir)?;

    Ok(())
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn remove_unnecessary_plugin_files(
    dart_root: &Path,
    integration_backend: IntegrationBackend,
) -> anyhow::Result<()> {
    // frb-coverage:ignore-end
    let lib_dir = dart_root.join("lib");
    remove_files_in_dir(&lib_dir)?;
    let src_dir = dart_root.join("src");
    if src_dir.exists() {
        remove_files_in_dir(&src_dir)?;
        fs::remove_dir(src_dir)?;
    }
    let ffi_gen_file = dart_root.join("ffigen.yaml");
    if ffi_gen_file.exists() {
        fs::remove_file(ffi_gen_file)?;
    }
    let hook_dir = dart_root.join("hook");
    if hook_dir.exists() {
        remove_files_in_dir(&hook_dir)?;
    }
    let test_dir = dart_root.join("test");
    if test_dir.exists() {
        remove_files_in_dir(&test_dir)?;
        fs::remove_dir(test_dir)?;
    }
    remove_package_ffi_pubspec_dependencies(dart_root)?;

    let example_lib_dir = dart_root.join("example").join("lib");
    remove_files_in_dir(&example_lib_dir)?;
    let example_web_dir = dart_root.join("example").join("web");
    if example_web_dir.exists() {
        fs::remove_dir_all(example_web_dir)?;
    }
    if integration_backend == IntegrationBackend::NativeAssets {
        let readme_file = dart_root.join("README.md");
        if readme_file.exists() {
            fs::remove_file(readme_file)?;
        }
    }

    let android_dir = dart_root.join("android");
    if android_dir.exists() {
        let android_src_dir = &android_dir.join("src");
        let android_src_main_dir = &android_src_dir.join("main");
        remove_files_in_dir(android_src_main_dir)?;
        fs::remove_dir(android_src_main_dir)?;
        fs::remove_dir(android_src_dir)?;
        remove_files_in_dir(&android_dir)?;
    }

    let ios_dir = dart_root.join("ios");
    if ios_dir.exists() {
        let ios_classes_dir = &ios_dir.join("Classes");
        remove_files_in_dir(ios_classes_dir)?;
        fs::remove_dir(ios_classes_dir)?;
        remove_files_in_dir(&ios_dir)?;
    }

    let linux_dir = dart_root.join("linux");
    if linux_dir.exists() {
        remove_files_in_dir(&linux_dir)?;
    }

    let macos_dir = dart_root.join("macos");
    if macos_dir.exists() {
        let macos_classes_dir = &macos_dir.join("Classes");
        remove_files_in_dir(macos_classes_dir)?;
        fs::remove_dir(macos_classes_dir)?;
        remove_files_in_dir(&macos_dir)?;
    }

    let windows_dir = dart_root.join("windows");
    if windows_dir.exists() {
        remove_files_in_dir(&windows_dir)?;
    }

    let ohos_dir = dart_root.join("ohos");
    if ohos_dir.exists() {
        fs::remove_dir_all(&ohos_dir)?;
    }
    Ok(())
}

fn remove_package_ffi_pubspec_dependencies(dart_root: &Path) -> anyhow::Result<()> {
    let path = dart_root.join("pubspec.yaml");
    let text_raw = fs::read_to_string(&path)?;
    let package_ffi_dependency = Regex::new(
        r"(?m)^  (code_assets|hooks|logging|native_toolchain_c|ffi|ffigen|test): .*\r?\n",
    )?;
    let text_output = package_ffi_dependency.replace_all(&text_raw, "");
    fs::write(path, text_output.as_bytes())?;
    Ok(())
}

fn remove_files_in_dir(dir: &Path) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(&path)?;
        } else if path.is_dir() {
            bail!("Directory '{:?}' was expected to contain only files but directory '{:?}' was encountered", dir.display(), path.display());
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::{remove_files_in_dir, remove_unnecessary_plugin_files};
    use crate::misc::IntegrationBackend;
    use std::fs;
    use std::path::Path;

    fn write_file(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, "x").unwrap();
    }

    #[test]
    fn test_remove_files_in_dir_rejects_nested_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root = temp_dir.path().join("root");
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("top.txt"), "x").unwrap();
        fs::write(root.join("nested").join("child.txt"), "x").unwrap();

        let err = remove_files_in_dir(&root).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("expected to contain only files"));
    }

    #[test]
    fn test_remove_unnecessary_plugin_files_cleans_platform_specific_layout() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dart_root = temp_dir.path();

        write_file(&dart_root.join("lib").join("a.dart"));
        write_file(&dart_root.join("src").join("bridge.h"));
        write_file(&dart_root.join("ffigen.yaml"));
        write_file(&dart_root.join("hook").join("build.dart"));
        write_file(&dart_root.join("test").join("sample_test.dart"));
        write_file(&dart_root.join("example").join("lib").join("example.dart"));
        write_file(&dart_root.join("example").join("web").join("index.html"));
        write_file(&dart_root.join("README.md"));
        fs::write(
            dart_root.join("pubspec.yaml"),
            r#"name: sample
dependencies:
  code_assets: ^1.0.0
  hooks: ^1.0.0
  logging: ^1.3.0
  native_toolchain_c: ^0.17.4
  keep_me: ^1.0.0

dev_dependencies:
  ffi: ^2.1.4
  ffigen: ^20.1.1
  flutter_lints: ^6.0.0
  test: ^1.28.0
"#,
        )
        .unwrap();

        write_file(
            &dart_root
                .join("android")
                .join("src")
                .join("main")
                .join("plugin.kt"),
        );
        write_file(&dart_root.join("android").join("AndroidManifest.xml"));

        write_file(&dart_root.join("ios").join("Classes").join("Plugin.swift"));
        write_file(&dart_root.join("ios").join("podspec.yaml"));

        write_file(&dart_root.join("linux").join("plugin.cc"));
        write_file(&dart_root.join("macos").join("Classes").join("Plugin.swift"));
        write_file(&dart_root.join("macos").join("podspec.yaml"));
        write_file(&dart_root.join("windows").join("plugin.cpp"));

        let ohos_lib = dart_root
            .join("ohos")
            .join("src")
            .join("main")
            .join("cpp")
            .join("types")
            .join("libmy_crate");
        write_file(&ohos_lib.join("bridge.c"));
        write_file(&dart_root.join("ohos").join("top.txt"));

        remove_unnecessary_plugin_files(dart_root, IntegrationBackend::NativeAssets).unwrap();

        assert!(!dart_root.join("ffigen.yaml").exists());
        assert!(!dart_root.join("src").exists());
        assert!(!dart_root.join("hook").join("build.dart").exists());
        assert!(!dart_root.join("test").exists());
        assert!(!dart_root.join("example").join("web").exists());
        assert!(!dart_root.join("README.md").exists());
        let pubspec = fs::read_to_string(dart_root.join("pubspec.yaml")).unwrap();
        assert!(!pubspec.contains("code_assets:"));
        assert!(!pubspec.contains("hooks:"));
        assert!(!pubspec.contains("native_toolchain_c:"));
        assert!(!pubspec.contains("ffi:"));
        assert!(!pubspec.contains("ffigen:"));
        assert!(!pubspec.contains("test:"));
        assert!(pubspec.contains("keep_me:"));
        assert!(pubspec.contains("flutter_lints:"));
        assert!(!dart_root.join("android").join("src").exists());
        assert!(!dart_root.join("ios").join("Classes").exists());
        assert!(!dart_root.join("macos").join("Classes").exists());
        assert!(!dart_root.join("ohos").exists());
        assert!(dart_root.join("android").exists());
        assert!(dart_root.join("ios").exists());
        assert!(dart_root.join("linux").exists());
        assert!(dart_root.join("macos").exists());
        assert!(dart_root.join("windows").exists());
    }
}
