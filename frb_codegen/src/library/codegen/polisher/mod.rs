use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::polisher::add_mod_to_lib::try_add_mod_to_lib;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::commands::format_rust::format_rust;
use crate::library::commands::dart_build_runner::dart_build_runner;
use crate::library::commands::dart_fix::dart_fix;
use crate::library::commands::dart_format::dart_format;
use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use anyhow::Context;
use cargo_metadata::VersionReq;
use itertools::Itertools;
use lazy_static::lazy_static;
use log::warn;
use std::fs;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref ANY_REQUIREMENT: VersionReq = VersionReq::parse(">= 1.0.0").unwrap();
    static ref BUILD_RUNNER_REQUIREMENT: VersionReq = VersionReq::parse(">= 1.7.0").unwrap();
}

pub(crate) mod add_mod_to_lib;
mod auto_upgrade;
pub(crate) mod internal_config;

pub(super) fn polish(
    config: &PolisherInternalConfig,
    needs_freezed: bool,
    needs_json_serializable: bool,
    output_paths: &[PathBuf],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    execute_try_add_mod_to_lib(config);
    execute_duplicate_c_output(config)?;
    ensure_dependencies(config, needs_freezed, needs_json_serializable)?;

    warn_if_fail(
        execute_build_runner(
            needs_freezed,
            needs_json_serializable,
            config,
            progress_bar_pack,
        ),
        "execute_build_runner",
    );
    if config.dart_fix {
        warn_if_fail(
            execute_dart_fix(config, progress_bar_pack),
            "execute_dart_fix",
        );
    }

    // Even if formatting generated code fails, it is not a big problem, and our codegen should not fail.
    if config.dart_format {
        warn_if_fail(
            execute_dart_format(config, progress_bar_pack),
            "execute_dart_format",
        );
    }
    if config.rust_format {
        warn_if_fail(
            execute_rust_format(output_paths, &config.rust_crate_dir, progress_bar_pack),
            "execute_rust_format",
        );
    }

    warn_if_fail(
        auto_upgrade::execute(progress_bar_pack, config),
        "auto_upgrade",
    );

    Ok(())
}

fn ensure_dependencies(
    config: &PolisherInternalConfig,
    needs_freezed: bool,
    needs_json_serializable: bool,
) -> anyhow::Result<()> {
    if needs_freezed {
        let repo = DartRepository::from_path(&config.dart_root)?;
        repo.has_specified_and_installed("freezed", DartDependencyMode::Dev, &ANY_REQUIREMENT)?;
        repo.has_specified_and_installed(
            "freezed_annotation",
            DartDependencyMode::Main,
            &ANY_REQUIREMENT,
        )?;
        repo.has_specified_and_installed(
            "build_runner",
            DartDependencyMode::Dev,
            build_runner_requirement(config.build_runner),
        )?;
    }

    if needs_json_serializable {
        let repo = DartRepository::from_path(&config.dart_root)?;
        repo.has_specified_and_installed(
            "json_annotation",
            DartDependencyMode::Main,
            &ANY_REQUIREMENT,
        )?;
        repo.has_specified_and_installed(
            "json_serializable",
            DartDependencyMode::Dev,
            &ANY_REQUIREMENT,
        )?;
    }

    Ok(())
}

fn build_runner_requirement(build_runner: bool) -> &'static VersionReq {
    if build_runner {
        &BUILD_RUNNER_REQUIREMENT
    } else {
        &ANY_REQUIREMENT
    }
}

fn warn_if_fail(r: anyhow::Result<()>, debug_name: &str) -> bool {
    match r {
        Ok(_) => true,
        Err(_) => {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            warn!("Fail to {debug_name}, but continue to run.\nError details: {r:?}");
            false
            // frb-coverage:ignore-end
        }
    }
}

fn execute_build_runner(
    needs_freezed: bool,
    needs_json_serializable: bool,
    config: &PolisherInternalConfig,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    if !should_execute_build_runner(needs_freezed, config.build_runner) {
        return Ok(());
    }

    let _pb = progress_bar_pack.polish_dart_build_runner.start();
    dart_build_runner(
        &config.dart_root,
        &config.dart_output,
        needs_json_serializable,
        config.fvm_install_mode,
    )
}

fn should_execute_build_runner(needs_freezed: bool, build_runner: bool) -> bool {
    needs_freezed && build_runner
}

fn execute_dart_fix(
    config: &PolisherInternalConfig,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_dart_fix.start();
    dart_fix(&config.dart_output, config.fvm_install_mode)
}

fn execute_dart_format(
    config: &PolisherInternalConfig,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_dart_formatter.start();
    dart_format(
        &config.dart_output,
        config.dart_format_line_length,
        config.fvm_install_mode,
    )
}

fn execute_rust_format(
    output_paths: &[PathBuf],
    base_path: &Path,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_rust_formatter.start();
    format_rust(&filter_paths_by_extension(output_paths, "rs"), base_path)
}

fn filter_paths_by_extension(paths: &[PathBuf], extension: &str) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|path| path.extension().unwrap().to_str().unwrap() == extension)
        .cloned()
        .collect_vec()
}

fn execute_try_add_mod_to_lib(config: &PolisherInternalConfig) {
    if config.add_mod_to_lib {
        try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }
}

fn execute_duplicate_c_output(config: &PolisherInternalConfig) -> anyhow::Result<()> {
    for path in config.duplicated_c_output_path.iter() {
        fs::copy(
            (config.c_output_path.as_ref())
                .context("When having duplicated_c_output_path, should also have c_output_path")?,
            path,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        build_runner_requirement, execute_build_runner, execute_duplicate_c_output,
        filter_paths_by_extension, should_execute_build_runner, warn_if_fail,
        BUILD_RUNNER_REQUIREMENT,
    };
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::polisher::internal_config::PolisherInternalConfig;
    use crate::misc::FvmInstallMode;
    use cargo_metadata::Version;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn config(temp_dir: &tempfile::TempDir) -> PolisherInternalConfig {
        PolisherInternalConfig {
            duplicated_c_output_path: vec![],
            dart_format_line_length: 80,
            dart_format: false,
            dart_fix: false,
            rust_format: false,
            add_mod_to_lib: false,
            build_runner: false,
            web_enabled: false,
            dart_output: temp_dir.path().join("dart/output.dart"),
            dart_root: temp_dir.path().join("dart"),
            rust_crate_dir: temp_dir.path().join("rust"),
            rust_output_path: temp_dir.path().join("rust/src/bridge.rs"),
            c_output_path: None,
            enable_auto_upgrade: false,
            fvm_install_mode: FvmInstallMode::Skip,
        }
    }

    /// Requires the first build_runner release that supports output filters.
    #[test]
    fn test_build_runner_requirement_supports_output_filters() {
        assert!(!BUILD_RUNNER_REQUIREMENT.matches(&Version::parse("1.6.9").unwrap()));
        assert!(BUILD_RUNNER_REQUIREMENT.matches(&Version::parse("1.7.0").unwrap()));
    }

    /// Keeps the legacy dependency range when automatic build_runner invocation is disabled.
    #[test]
    fn test_build_runner_requirement_when_invocation_disabled() {
        assert!(build_runner_requirement(false).matches(&Version::parse("1.6.9").unwrap()));
        assert!(!build_runner_requirement(true).matches(&Version::parse("1.6.9").unwrap()));
    }

    /// Retains only output paths whose extension matches the requested formatter input.
    #[test]
    fn test_filter_paths_by_extension_keeps_matching_paths() {
        let paths = vec![
            PathBuf::from("bridge.rs"),
            PathBuf::from("bridge.dart"),
            PathBuf::from("nested/module.rs"),
        ];

        assert_eq!(
            filter_paths_by_extension(&paths, "rs"),
            vec![
                PathBuf::from("bridge.rs"),
                PathBuf::from("nested/module.rs")
            ],
        );
    }

    /// Skips build_runner when generated code does not require freezed.
    #[test]
    fn test_execute_build_runner_skips_without_freezed() {
        let temp_dir = tempdir().unwrap();
        let progress = GeneratorProgressBarPack::new();

        execute_build_runner(false, true, &config(&temp_dir), &progress).unwrap();
    }

    /// Skips build_runner when its invocation is disabled in the configuration.
    #[test]
    fn test_execute_build_runner_skips_when_disabled() {
        let temp_dir = tempdir().unwrap();
        let progress = GeneratorProgressBarPack::new();

        execute_build_runner(true, false, &config(&temp_dir), &progress).unwrap();
    }

    /// Runs build_runner only when freezed output and invocation are both enabled.
    #[test]
    fn test_should_execute_build_runner_matrix() {
        assert!(!should_execute_build_runner(false, false));
        assert!(!should_execute_build_runner(false, true));
        assert!(!should_execute_build_runner(true, false));
        assert!(should_execute_build_runner(true, true));
    }

    /// Returns whether an optional polish operation completed successfully.
    #[test]
    fn test_warn_if_fail_returns_operation_status() {
        assert!(warn_if_fail(Ok(()), "success"));
        assert!(!warn_if_fail(Err(anyhow::anyhow!("failure")), "failure"));
    }

    /// Copies exact C output bytes to every configured duplicate destination.
    #[test]
    fn test_execute_duplicate_c_output_copies_bytes() {
        let temp_dir = tempdir().unwrap();
        let source = temp_dir.path().join("bridge.h");
        let duplicate = temp_dir.path().join("include/bridge.h");
        fs::create_dir(duplicate.parent().unwrap()).unwrap();
        fs::write(&source, [0, 1, 2, 255]).unwrap();
        let mut config = config(&temp_dir);
        config.c_output_path = Some(source);
        config.duplicated_c_output_path = vec![duplicate.clone()];

        execute_duplicate_c_output(&config).unwrap();

        assert_eq!(fs::read(duplicate).unwrap(), vec![0, 1, 2, 255]);
    }
}
