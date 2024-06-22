use crate::codegen::config::config::MetaConfig;
use crate::codegen::config::internal_config::InternalConfig;
use crate::codegen::config::internal_config_parser::rust_path_parser::RustInputInfo;
use crate::codegen::dumper::internal_config::DumperInternalConfig;
use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::codegen::preparer::internal_config::PreparerInternalConfig;
use crate::codegen::{Config, ConfigDumpContent};
use crate::utils::path_utils::{canonicalize_with_error_message, find_dart_package_dir};
use anyhow::Result;
use itertools::Itertools;
use log::debug;
use std::path::PathBuf;
use strum::IntoEnumIterator;

mod controller_parser;
mod dart_path_parser;
mod generator_parser;
mod rust_path_migrator;
mod rust_path_parser;

impl InternalConfig {
    pub(crate) fn parse(config: &Config, meta_config: &MetaConfig) -> Result<Self> {
        let base_dir = (config.base_dir.as_ref())
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .unwrap_or(std::env::current_dir()?);
        debug!("InternalConfig.parse base_dir={base_dir:?}");

        let rust_input = (config.rust_input.clone())
            .expect("Please provide `rust_input` (via config file or command line)");
        let dart_output = (config.dart_output.clone())
            .expect("Please provide `dart_output` (via config file or command line)");

        let migrated_rust_input =
            rust_path_migrator::migrate_rust_input_config(&config.rust_root, &rust_input)?;
        let RustInputInfo {
            rust_crate_dir,
            third_party_crate_names,
            rust_input_namespace_pack,
            rust_output_path,
        } = rust_path_parser::compute_rust_path_info(
            &migrated_rust_input,
            &base_dir,
            &config.rust_output,
        )?;

        let dart_output_dir = canonicalize_with_error_message(&base_dir.join(dart_output))?;
        let dart_output_path_pack =
            dart_path_parser::compute_dart_output_path_pack(&dart_output_dir)?;

        let dart_output_class_name_pack = compute_dart_output_class_name_pack(config);

        let c_output_path = config.c_output.as_ref().map(|x| base_dir.join(x));
        let duplicated_c_output_path = config
            .duplicated_c_output
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|p| base_dir.join(p))
            .collect();

        let dart_root = canonicalize_with_error_message(
            &(config.dart_root.clone().map(PathBuf::from))
                .unwrap_or(find_dart_package_dir(&dart_output_dir)?),
        )?;

        let web_enabled = config.web.unwrap_or(true);

        let dump_directory = rust_crate_dir.join("target").join("frb_dump");

        let full_dep = config.full_dep.unwrap_or(false);
        let default_stream_sink_codec = generate_default_stream_sink_codec(full_dep);
        let default_rust_opaque_codec = generate_default_rust_opaque_codec(full_dep);
        let enable_local_dependency = config.local.unwrap_or_default();
        let stop_on_error = config.stop_on_error.unwrap_or_default();

        let controller = controller_parser::parse(meta_config, &rust_crate_dir, &rust_output_path)?;

        let generator = generator_parser::parse(generator_parser::Args {
            config,
            dart_root: &dart_root,
            rust_crate_dir: &rust_crate_dir,
            dart_output_path_pack: &dart_output_path_pack,
            dart_output_class_name_pack: &dart_output_class_name_pack,
            rust_output_path: &rust_output_path,
            default_stream_sink_codec,
            default_rust_opaque_codec,
            c_output_path: &c_output_path,
            web_enabled,
            full_dep,
        })?;

        Ok(InternalConfig {
            controller,
            preparer: PreparerInternalConfig {
                dart_root: dart_root.clone(),
                deps_check: config.deps_check.unwrap_or(true),
                needs_ffigen: full_dep,
            },
            parser: ParserInternalConfig {
                hir: ParserHirInternalConfig {
                    rust_crate_dir: rust_crate_dir.clone(),
                    rust_input_namespace_pack: rust_input_namespace_pack.clone(),
                    third_party_crate_names,
                },
                mir: ParserMirInternalConfig {
                    rust_input_namespace_pack: rust_input_namespace_pack.clone(),
                    force_codec_mode_pack: compute_force_codec_mode_pack(full_dep),
                    default_stream_sink_codec,
                    default_rust_opaque_codec,
                    stop_on_error,
                    enable_lifetime: config.enable_lifetime.unwrap_or_default(),
                    type_64bit_int: config.type_64bit_int.unwrap_or_default(),
                    default_dart_async: config.default_dart_async.unwrap_or(true),
                },
            },
            generator,
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
                enable_auto_upgrade: !enable_local_dependency,
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

fn compute_dart_output_class_name_pack(config: &Config) -> DartOutputClassNamePack {
    const FALLBACK_DART_ENTRYPOINT_CLASS_NAME: &str = "RustLib";

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

pub(crate) fn compute_force_codec_mode_pack(full_dep: bool) -> Option<CodecModePack> {
    (!full_dep).then_some(CodecModePack {
        dart2rust: CodecMode::Pde,
        rust2dart: CodecMode::Pde,
    })
}

fn generate_default_stream_sink_codec(full_dep: bool) -> CodecMode {
    if full_dep {
        CodecMode::Dco
    } else {
        CodecMode::Sse
    }
}

fn generate_default_rust_opaque_codec(full_dep: bool) -> RustOpaqueCodecMode {
    if full_dep {
        RustOpaqueCodecMode::Nom
    } else {
        RustOpaqueCodecMode::Moi
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
    fn test_parse_rust_output_faulty() -> anyhow::Result<()> {
        let result = body("library/codegen/config/internal_config_parser/faulty_rust_output");

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert!(error
            .to_string()
            .contains("Rust output path needs to include the file name."));
        Ok(())
    }
}
