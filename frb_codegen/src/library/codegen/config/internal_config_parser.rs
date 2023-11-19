use crate::codegen::config::internal_config::{
    GeneratorInternalConfig, GeneratorWireInternalConfig, InternalConfig, Namespace,
};
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::TargetOrCommonMap;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::parser::internal_config::{ParserInternalConfig, RustInputPathPack};
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::codegen::preparer::internal_config::PreparerInternalConfig;
use crate::codegen::Config;
use crate::utils::path_utils::{
    find_dart_package_dir, find_rust_crate_dir, glob_path, path_to_string,
};
use anyhow::{ensure, Context, Result};
use chrono::format::Fixed::TimezoneOffsetDoubleColon;
use convert_case::{Case, Casing};
use itertools::Itertools;
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

impl InternalConfig {
    pub(crate) fn parse(config: Config) -> Result<Self> {
        let base_dir = config
            .base_dir
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .unwrap_or(std::env::current_dir()?);
        debug!("InternalConfig.parse base_dir={base_dir:?}");

        let rust_input_path_pack = compute_rust_input_path_pack(&config.rust_input, &base_dir)?;
        let namespaces = rust_input_path_pack.rust_input_path.keys().collect_vec();

        let rust_output_path = compute_rust_output_path(&config, &base_dir, &rust_input_path_pack);

        let dart_output_dir: PathBuf = base_dir.join(config.dart_output);
        let dart_output_path_pack = compute_dart_output_path_pack(&dart_output_dir, &namespaces);
        let dart_class_name = compute_dart_class_name(&config.dart_class_name, &namespaces);

        let c_output_path = base_dir.join(&config.c_output);
        let duplicated_c_output_path = config
            .duplicated_c_output
            .unwrap_or_default()
            .into_iter()
            .map(|p| base_dir.join(&p))
            .collect();

        let rust_crate_dir: PathBuf = (config.rust_crate_dir.map(PathBuf::from)).unwrap_or(
            find_rust_crate_dir(rust_input_path_pack.one_rust_input_path())?,
        );
        let dart_root = (config.dart_root.map(PathBuf::from))
            .unwrap_or(find_dart_package_dir(&dart_output_dir)?);

        let wasm_enabled = config.wasm.unwrap_or(false);
        let use_bridge_in_method = config.use_bridge_in_method.unwrap_or(true);
        let dart_enums_style = config.dart_enums_style.unwrap_or(false);
        let dart3 = config.dart3.unwrap_or(true);

        // TODO multi file support
        let dart_output_path = dart_output_path_pack
            .dart_decl_output_path
            .values()
            .next()
            .unwrap();
        let dart_output_stem = get_file_stem(dart_output_path);
        let dart_api_instance_name =
            compute_dart_api_instance_name(use_bridge_in_method, dart_output_stem);

        Ok(InternalConfig {
            preparer: PreparerInternalConfig {
                dart_root: dart_root.clone(),
                deps_check: config.deps_check.unwrap_or(true),
            },
            parser: ParserInternalConfig {
                rust_input_path_pack,
                rust_crate_dir: rust_crate_dir.clone(),
            },
            generator: GeneratorInternalConfig {
                api_dart: GeneratorApiDartInternalConfig {
                    dart_api_class_name: dart_output_stem.to_case(Case::Pascal),
                    dart_api_instance_name,
                    dart_enums_style,
                    use_bridge_in_method,
                    dart3,
                    dart_decl_output_path: dart_output_path_pack.dart_decl_output_path,
                },
                wire: GeneratorWireInternalConfig {
                    dart: GeneratorWireDartInternalConfig {
                        dart_root: dart_root.clone(),
                        use_bridge_in_method,
                        wasm_enabled,
                        // TODO may not have such class after refactoring API, thus temp do this
                        dart_wire_class_name: dart_class_name.values().next().unwrap().to_owned(),
                        llvm_path: config
                            .llvm_path
                            .unwrap_or_else(fallback_llvm_path)
                            .into_iter()
                            .map(PathBuf::from)
                            .collect_vec(),
                        llvm_compiler_opts: config.llvm_compiler_opts.unwrap_or_else(String::new),
                        // TODO
                        // dart_impl_output_path: dart_output_path_pack.dart_impl_output_path,
                    },
                    rust: GeneratorWireRustInternalConfig {
                        rust_crate_dir: rust_crate_dir.clone(),
                        rust_wire_mod: TODO,
                        wasm_enabled,
                        rust_output_path: rust_output_path.clone(),
                    },
                    c: GeneratorWireCInternalConfig {
                        rust_crate_dir: rust_crate_dir.clone(),
                        rust_output_path: rust_output_path.clone(),
                        c_output_path: c_output_path.clone(),
                        extern_func_names: TODO,
                        extern_struct_names: TODO,
                        // TODO
                        // extra_headers: config.extra_headers.unwrap_or_else(String::new),
                    },
                },
            },
            polisher: PolisherInternalConfig {
                duplicated_c_output_path,
                dart_format_line_length: config.dart_format_line_length.unwrap_or(80),
                add_mod_to_lib: config.add_mod_to_lib.unwrap_or(true),
                build_runner: config.build_runner.unwrap_or(true),
                wasm_enabled,
                dart_root,
                rust_crate_dir,
                rust_output_path,
                c_output_path,
            },
        })
    }
}

impl RustInputPathPack {
    fn one_rust_input_path(&self) -> &Path {
        self.rust_input_path.iter().next().unwrap().1
    }
}

fn compute_rust_input_path_pack(
    raw_rust_input: &str,
    base_dir: &Path,
) -> Result<RustInputPathPack> {
    const BLACKLIST_FILE_NAMES: [&str; 1] = ["mod.rs"];

    let paths = glob_path(&base_dir.join(raw_rust_input))?
        .into_iter()
        .filter(|path| !BLACKLIST_FILE_NAMES.contains(&path.file_name().unwrap().to_str().unwrap()))
        .collect_vec();

    let pack = RustInputPathPack {
        rust_input_path: paths
            .into_iter()
            .map(|path| Ok((compute_namespace_from_rust_input_path(&path)?, path)))
            .collect::<Result<HashMap<_, _>>>()?,
    };

    ensure!(!pack.rust_input_path.is_empty());
    ensure!(
        !pack.rust_input_path.values().any(|p| path_to_string(p).unwrap().contains("lib.rs")),
        "Do not use `lib.rs` as a Rust input. Please put code to be generated in something like `api.rs`.",
    );

    Ok(pack)
}

fn compute_rust_output_path(
    config: &Config,
    base_dir: &PathBuf,
    rust_input_path_pack: &RustInputPathPack,
) -> TargetOrCommonMap<PathBuf> {
    let common =
        base_dir.join(&config.rust_output.map(PathBuf::from).unwrap_or_else(|| {
            fallback_rust_output_path(rust_input_path_pack.one_rust_input_path())
        }));

    TargetOrCommonMap {
        common: common.clone(),
        io: common.with_extension(".io.rs"),
        wasm: common.with_extenswasmn(".web.rs"),
    }
}

fn compute_namespace_from_rust_input_path(rust_input_path: &Path) -> Result<Namespace> {
    let stem = rust_input_path
        .file_stem()
        .context("cannot get file stem")?
        .to_str()
        .context("cannot convert to str")?;
    Ok(Namespace {
        name: stem.to_owned(),
    })
}

struct DartOutputPathPack {
    dart_decl_output_path: HashMap<Namespace, PathBuf>,
    dart_impl_output_path: PathBuf,
}

fn compute_dart_output_path_pack(
    dart_output_dir: &Path,
    namespaces: &[&Namespace],
) -> DartOutputPathPack {
    DartOutputPathPack {
        dart_decl_output_path: namespaces
            .iter()
            .map(|&namespace| {
                (
                    namespace.to_owned(),
                    dart_output_dir.join(compute_dart_decl_output_filename(namespace)),
                )
            })
            .collect(),
        dart_impl_output_path: dart_output_dir.join("bridge_generated.dart"),
    }
}

fn compute_dart_decl_output_filename(namespace: &Namespace) -> String {
    format!("{}.dart", namespace.name.to_case(Case::Snake))
}

fn compute_dart_class_name(
    dart_class_name: &Option<String>,
    namespaces: &[&Namespace],
) -> HashMap<Namespace, String> {
    const NAMESPACE_PLACEHOLDER: &str = "{namespace}";
    let dart_class_name = dart_class_name.as_deref().unwrap_or(NAMESPACE_PLACEHOLDER);
    namespaces
        .iter()
        .map(|&namespace| {
            (
                namespace.to_owned(),
                dart_class_name
                    .replace(NAMESPACE_PLACEHOLDER, &namespace.name.to_case(Case::Pascal)),
            )
        })
        .collect()
}

fn fallback_rust_output_path(rust_input_path: &Path) -> PathBuf {
    rust_input_path.with_file_name("bridge_generated.rs")
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

fn compute_dart_api_instance_name(use_bridge_in_method: bool, dart_output_stem: &str) -> String {
    if use_bridge_in_method {
        "bridge".to_owned()
    } else {
        dart_output_stem.to_case(Case::Camel)
    }
}

fn get_file_stem(p: &Path) -> &str {
    p.file_stem().unwrap().to_str().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config::InternalConfig;
    use crate::codegen::Config;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::path_utils::path_to_string;
    use crate::utils::test_utils::{get_test_fixture_dir, json_golden_test};
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

        let config = Config::from_files_auto()?;

        let internal_config = InternalConfig::parse(config)?;

        let actual_string = serde_json::to_string_pretty(&internal_config)?;
        let actual_json: Value = serde_json::from_str(&actual_string)?;

        json_golden_test(
            &actual_json,
            &PathBuf::from("expect_output.json"),
            &vec![(
                path_to_string(&test_fixture_dir)?,
                "{the-working-directory}".to_owned(),
            )],
        )?;

        Ok(())
    }
}
