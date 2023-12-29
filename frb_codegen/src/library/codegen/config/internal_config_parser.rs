use crate::codegen::config::config::MetaConfig;
use crate::codegen::config::internal_config::{
    ControllerInternalConfig, GeneratorInternalConfig, GeneratorWireInternalConfig, InternalConfig,
    RustInputPathPack,
};
use crate::codegen::dumper::internal_config::DumperInternalConfig;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::target::TargetOrCommonMap;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::{
    DartOutputClassNamePack, GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig,
    GeneratorWireDartInternalConfig,
};
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::codegen::preparer::internal_config::PreparerInternalConfig;
use crate::codegen::{Config, ConfigDumpContent};
use crate::library::commands::cargo_metadata::execute_cargo_metadata;
use crate::utils::path_utils::{
    canonicalize_with_error_message, find_dart_package_dir, find_rust_crate_dir, glob_path,
    path_to_string,
};
use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use log::debug;
use pathdiff::diff_paths;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

impl InternalConfig {
    pub(crate) fn parse(config: &Config, meta_config: &MetaConfig) -> Result<Self> {
        let base_dir = config
            .base_dir
            .as_ref()
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .unwrap_or(std::env::current_dir()?);
        debug!("InternalConfig.parse base_dir={base_dir:?}");

        let rust_input_path_pack = compute_rust_input_path_pack(&config.rust_input, &base_dir)?;

        let dart_output_dir = canonicalize_with_error_message(&base_dir.join(&config.dart_output))?;
        let dart_output_path_pack = compute_dart_output_path_pack(&dart_output_dir);

        let dart_output_class_name_pack = compute_dart_output_class_name_pack(config);

        let c_output_path = config.c_output.as_ref().map(|x| base_dir.join(x));
        let duplicated_c_output_path = config
            .duplicated_c_output
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|p| base_dir.join(p))
            .collect();

        let rust_crate_dir = canonicalize_with_error_message(
            &(config.rust_root.clone().map(PathBuf::from)).unwrap_or(find_rust_crate_dir(
                rust_input_path_pack.one_rust_input_path(),
            )?),
        )?;
        let rust_output_path = compute_rust_output_path(config, &base_dir, &rust_crate_dir);

        let dart_root = canonicalize_with_error_message(
            &(config.dart_root.clone().map(PathBuf::from))
                .unwrap_or(find_dart_package_dir(&dart_output_dir)?),
        )?;

        let default_external_library_loader =
            compute_default_external_library_loader(&rust_crate_dir, &dart_root, config);

        let web_enabled = config.web.unwrap_or(true);
        let dart_enums_style = config.dart_enums_style.unwrap_or(true);
        let dart3 = config.dart3.unwrap_or(true);

        let dump_directory = rust_crate_dir.join("target").join("frb_dump");

        let controller_watching_paths = vec![
            // The whole crate needs to be watched, because e.g. when a struct definition changes
            // in a non-input file, it may still cause the generated code to change.
            rust_crate_dir.join("src"),
        ];
        let controller_exclude_paths = rust_output_path.clone().into_vec();

        Ok(InternalConfig {
            controller: ControllerInternalConfig {
                watch: meta_config.watch,
                watching_paths: controller_watching_paths,
                exclude_paths: controller_exclude_paths,
                max_count: None,
            },
            preparer: PreparerInternalConfig {
                dart_root: dart_root.clone(),
                deps_check: config.deps_check.unwrap_or(true),
            },
            parser: ParserInternalConfig {
                rust_input_path_pack: rust_input_path_pack.clone(),
                rust_crate_dir: rust_crate_dir.clone(),
            },
            generator: GeneratorInternalConfig {
                api_dart: GeneratorApiDartInternalConfig {
                    dart_enums_style,
                    dart3,
                    dart_decl_base_output_path: dart_output_path_pack.dart_decl_base_output_path,
                    dart_entrypoint_class_name: dart_output_class_name_pack
                        .entrypoint_class_name
                        .clone(),
                },
                wire: GeneratorWireInternalConfig {
                    dart: GeneratorWireDartInternalConfig {
                        dart_root: dart_root.clone(),
                        web_enabled,
                        llvm_path: config
                            .llvm_path
                            .clone()
                            .unwrap_or_else(fallback_llvm_path)
                            .into_iter()
                            .map(PathBuf::from)
                            .collect_vec(),
                        llvm_compiler_opts: config.llvm_compiler_opts.clone().unwrap_or_default(),
                        extra_headers: config.extra_headers.clone().unwrap_or_default(),
                        dart_impl_output_path: dart_output_path_pack.dart_impl_output_path,
                        dart_output_class_name_pack,
                        default_external_library_loader,
                    },
                    rust: GeneratorWireRustInternalConfig {
                        rust_input_path_pack,
                        rust_crate_dir: rust_crate_dir.clone(),
                        web_enabled,
                        rust_output_path: rust_output_path.clone(),
                    },
                    c: GeneratorWireCInternalConfig {
                        rust_crate_dir: rust_crate_dir.clone(),
                        rust_output_path: rust_output_path.clone(),
                        c_output_path: c_output_path.clone(),
                    },
                },
            },
            polisher: PolisherInternalConfig {
                duplicated_c_output_path,
                dart_format_line_length: config.dart_format_line_length.unwrap_or(80),
                add_mod_to_lib: config.add_mod_to_lib.unwrap_or(true),
                build_runner: config.build_runner.unwrap_or(true),
                web_enabled,
                dart_root,
                rust_crate_dir,
                rust_output_path,
                c_output_path,
            },
            dumper: DumperInternalConfig {
                dump_contents: parse_dump_contents(config),
                dump_directory,
            },
        })
    }
}

fn parse_dump_contents(config: &Config) -> Vec<ConfigDumpContent> {
    if config.dump_all.unwrap_or(false) {
        return ConfigDumpContent::iter().collect_vec();
    }
    config.dump.clone().unwrap_or_default()
}

fn compute_default_external_library_loader(
    rust_crate_dir: &Path,
    dart_root: &Path,
    config: &Config,
) -> GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig {
    GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig {
        stem: compute_default_external_library_stem(rust_crate_dir)
            .unwrap_or(FALLBACK_DEFAULT_EXTERNAL_LIBRARY_STEM.to_owned()),
        io_directory: compute_default_external_library_relative_directory(
            rust_crate_dir,
            dart_root,
        )
        .unwrap_or(FALLBACK_DEFAULT_EXTERNAL_LIBRARY_RELATIVE_DIRECTORY.to_owned()),
        web_prefix: config
            .default_external_library_loader_web_prefix
            .as_deref()
            .unwrap_or("pkg/")
            .into(),
    }
}

fn compute_default_external_library_stem(rust_crate_dir: &Path) -> Result<String> {
    let metadata = execute_cargo_metadata(&rust_crate_dir.join("Cargo.toml"))?;
    let package = metadata
        .root_package()
        .context("cannot find root package")?;
    let target = (package.targets.iter())
        .find(|target| target.kind.iter().any(|kind| kind.contains("lib")))
        .context("cannot find target")?;
    Ok(target.name.clone())
}

fn compute_default_external_library_relative_directory(
    rust_crate_dir: &Path,
    dart_root: &Path,
) -> Result<String> {
    let diff = diff_paths(rust_crate_dir, dart_root).context("cannot diff path")?;
    Ok(path_to_string(&diff.join("target").join("release/"))?.replace('\\', "/"))
}

const FALLBACK_DEFAULT_EXTERNAL_LIBRARY_STEM: &str = "UNKNOWN";
const FALLBACK_DEFAULT_EXTERNAL_LIBRARY_RELATIVE_DIRECTORY: &str = "UNKNOWN";

impl RustInputPathPack {
    fn one_rust_input_path(&self) -> &Path {
        self.rust_input_paths.first().unwrap()
    }
}

fn compute_rust_input_path_pack(
    raw_rust_input: &str,
    base_dir: &Path,
) -> Result<RustInputPathPack> {
    const BLACKLIST_FILE_NAMES: [&str; 1] = ["mod.rs"];

    let glob_pattern = base_dir.join(raw_rust_input);
    let rust_input_paths = glob_path(&glob_pattern)?
        .into_iter()
        .filter(|path| !BLACKLIST_FILE_NAMES.contains(&path.file_name().unwrap().to_str().unwrap()))
        .collect_vec();

    let pack = RustInputPathPack { rust_input_paths };

    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    ensure!(
        !pack.rust_input_paths.is_empty(),
        "Find zero rust input paths. (glob_pattern={glob_pattern:?})"
    );
    ensure!(
        !pack.rust_input_paths.iter().any(|p| path_to_string(p).unwrap().contains("lib.rs")),
        "Do not use `lib.rs` as a Rust input. Please put code to be generated in something like `api.rs`.",
    );
    // frb-coverage:ignore-end

    Ok(pack)
}

fn compute_rust_output_path(
    config: &Config,
    base_dir: &Path,
    rust_crate_dir: &Path,
) -> TargetOrCommonMap<PathBuf> {
    let path_common = base_dir.join(
        (config.rust_output.clone().map(PathBuf::from))
            .unwrap_or_else(|| fallback_rust_output_path(rust_crate_dir)),
    );
    compute_path_map(&path_common)
}

struct DartOutputPathPack {
    dart_decl_base_output_path: PathBuf,
    dart_impl_output_path: TargetOrCommonMap<PathBuf>,
}

fn compute_dart_output_path_pack(dart_output_dir: &Path) -> DartOutputPathPack {
    DartOutputPathPack {
        dart_decl_base_output_path: dart_output_dir.to_owned(),
        dart_impl_output_path: compute_path_map(&dart_output_dir.join("frb_generated.dart")),
    }
}

fn compute_path_map(path_common: &Path) -> TargetOrCommonMap<PathBuf> {
    let extension = path_common.extension()
        .unwrap_or_else(|| panic!(
            "Cannot use the path configuration\n {:?}.\n\
            A path for input/output needs to include the file name (a glob, like *.rs, can be used).",
            path_common
        )).to_str().unwrap();

    TargetOrCommonMap {
        common: path_common.to_owned(),
        io: path_common.with_extension(format!("io.{extension}")),
        web: path_common.with_extension(format!("web.{extension}")),
    }
}

fn fallback_rust_output_path(rust_crate_dir: &Path) -> PathBuf {
    rust_crate_dir.join("src").join("frb_generated.rs")
}

fn fallback_llvm_path() -> Vec<String> {
    vec![
        "/opt/homebrew/opt/llvm".to_owned(), // Homebrew root
        "/usr/local/opt/llvm".to_owned(),    // Homebrew x86-64 root
        // Possible Linux LLVM roots
        "/usr/lib/llvm-9".to_owned(),
        "/usr/lib/llvm-10".to_owned(),
        "/usr/lib/llvm-11".to_owned(),
        "/usr/lib/llvm-12".to_owned(),
        "/usr/lib/llvm-13".to_owned(),
        "/usr/lib/llvm-14".to_owned(),
        "/usr/lib/".to_owned(),
        "/usr/lib64/".to_owned(),
        "C:/Program Files/llvm".to_owned(), // Default on Windows
        "C:/msys64/mingw64".to_owned(), // https://packages.msys2.org/package/mingw-w64-x86_64-clang
    ]
}

const FALLBACK_DART_ENTRYPOINT_CLASS_NAME: &str = "RustLib";

fn compute_dart_output_class_name_pack(config: &Config) -> DartOutputClassNamePack {
    let entrypoint_class_name = (config.dart_entrypoint_class_name.clone())
        .unwrap_or(FALLBACK_DART_ENTRYPOINT_CLASS_NAME.to_owned());
    let with_postfix = |postfix: &str| format!("{entrypoint_class_name}{postfix}");

    DartOutputClassNamePack {
        entrypoint_class_name: entrypoint_class_name.clone(),
        api_class_name: with_postfix("Api"),
        api_impl_class_name: with_postfix("ApiImpl"),
        api_impl_platform_class_name: with_postfix("ApiImplPlatform"),
        wire_class_name: with_postfix("Wire"),
        wasm_module_name: with_postfix("WasmModule"),
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::config::MetaConfig;
    use crate::codegen::config::internal_config::InternalConfig;
    use crate::codegen::Config;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::{
        create_path_sanitizers, get_test_fixture_dir, json_golden_test,
    };
    use log::info;
    use serde_json::Value;
    use serial_test::serial;
    use std::env;
    use std::path::PathBuf;

    #[test]
    #[serial]
    fn test_parse_single_rust_input() -> anyhow::Result<()> {
        body("library/codegen/config/internal_config_parser/single_rust_input")
    }

    #[test]
    #[serial]
    fn test_parse_wildcard_rust_input() -> anyhow::Result<()> {
        body("library/codegen/config/internal_config_parser/wildcard_rust_input")
    }

    fn body(fixture_name: &str) -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        env::set_current_dir(&test_fixture_dir)?;
        info!("test_fixture_dir={test_fixture_dir:?}");

        let config = Config::from_files_auto()?;

        let internal_config = InternalConfig::parse(&config, &MetaConfig { watch: false })?;

        let actual_string = serde_json::to_string_pretty(&internal_config)?;
        let actual_json: Value = serde_json::from_str(&actual_string)?;

        json_golden_test(
            &actual_json,
            &PathBuf::from("expect_output.json"),
            &create_path_sanitizers(&test_fixture_dir),
        )?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_compute_path_map() -> anyhow::Result<()> {
        let result = super::compute_path_map(&PathBuf::from("src/api/api.rs"));
        assert_eq!(result.common, PathBuf::from("src/api/api.rs"));
        assert_eq!(result.io, PathBuf::from("src/api/api.io.rs"));
        assert_eq!(result.web, PathBuf::from("src/api/api.web.rs"));
        Ok(())
    }
    #[test]
    #[serial]
    fn test_compute_path_map_with_glob() -> anyhow::Result<()> {
        let result = super::compute_path_map(&PathBuf::from("src/api/*.rs"));
        assert_eq!(result.common, PathBuf::from("src/api/*.rs"));
        assert_eq!(result.io, PathBuf::from("src/api/*.io.rs"));
        assert_eq!(result.web, PathBuf::from("src/api/*.web.rs"));
        Ok(())
    }
    #[test]
    #[serial]
    fn test_compute_path_map_faulty() -> anyhow::Result<()> {
        // Set a custom panic hook that does not print the panic information
        let _ = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_panic_info| {
            // Custom panic hook: Do nothing
        }));

        let result = std::panic::catch_unwind(|| {
            super::compute_path_map(&PathBuf::from("src/api"));
        });

        // Restore the default panic hook
        std::panic::set_hook(Box::new(|panic_info| {
            eprintln!("{}", panic_info);
        }));
        assert!(result.is_err());
        Ok(())
    }
}
