use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::ConfigDumpContent;
use crate::codegen::ConfigDumpContent::Hir;
use ConfigDumpContent::Mir;

pub(crate) mod early_generator;
pub(crate) mod hir;
pub(crate) mod internal_config;
pub(crate) mod mir;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<MirPack> {
    parse_inner(config, dumper, progress_bar_pack, |_| Ok(()))
}

fn parse_inner(
    config: &ParserInternalConfig,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
    on_hir_flat: impl FnOnce(&HirFlatPack) -> anyhow::Result<()>,
) -> anyhow::Result<MirPack> {
    let dumper_hir = dumper.with_content(Hir);
    let dumper_hir_tree = dumper_hir.with_add_name_prefix("hir_tree/");
    let dumper_hir_naive_flat = dumper_hir.with_add_name_prefix("hir_naive_flat/");
    let dumper_hir_flat = dumper_hir.with_add_name_prefix("hir_flat/");
    let dumper_early_generator = dumper_hir.with_add_name_prefix("early_generator/");
    let dumper_mir = dumper.with_content(Mir);

    let pb = progress_bar_pack.parse_hir_raw.start();
    let hir_raw = hir::raw::parse(&config.hir, dumper)?;
    drop(pb);

    let pb = progress_bar_pack.parse_hir_primary.start();
    let hir_tree = hir::tree::parse(&config.hir, hir_raw, &dumper_hir_tree)?;
    let hir_naive_flat = hir::naive_flat::parse(&config.hir, hir_tree, &dumper_hir_naive_flat)?;
    let hir_flat = hir::flat::parse(&config.hir, hir_naive_flat, &dumper_hir_flat)?;
    on_hir_flat(&hir_flat)?;
    let ir_early_generator =
        early_generator::execute(hir_flat, &config.mir, &dumper_early_generator)?;
    drop(pb);

    let pb = progress_bar_pack.parse_mir.start();
    let mir_pack = mir::parse(
        &config.mir,
        &ir_early_generator,
        &dumper_mir,
        mir::ParseMode::Normal,
    )?;
    drop(pb);

    Ok(mir_pack)
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config_parser::compute_force_codec_mode_pack;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
    use crate::codegen::parser::internal_config::ParserInternalConfig;
    use crate::codegen::parser::mir::internal_config::{
        ParserMirInternalConfig, RustInputNamespacePack,
    };
    use crate::codegen::parser::{parse_inner, MirPack};
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::namespace::Namespace;
    use crate::utils::test_utils::{
        create_path_sanitizers, get_test_fixture_dir, json_golden_test,
    };
    use log::info;
    use serial_test::serial;
    use std::path::{Path, PathBuf};

    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/simple", None)
    }

    #[test]
    #[serial]
    fn test_methods() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/methods", None)
    }

    #[test]
    #[serial]
    fn test_multi_input_file() -> anyhow::Result<()> {
        body(
            "library/codegen/parser/mod/multi_input_file",
            Some(Box::new(|_rust_crate_dir| RustInputNamespacePack {
                rust_input_namespace_prefixes: vec![
                    Namespace::new_self_crate("api_one".to_owned()),
                    Namespace::new_self_crate("api_two".to_owned()),
                ],
                rust_output_path_namespace: Namespace::new_self_crate("frb_generated".to_owned()),
            })),
        )
    }

    #[test]
    #[serial]
    fn test_use_type_in_another_file() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/use_type_in_another_file", None)
    }

    #[test]
    #[serial]
    fn test_qualified_names() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/qualified_names", None)
    }

    #[test]
    #[serial]
    fn test_non_qualified_names() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/non_qualified_names", None)
    }

    #[test]
    #[serial]
    fn test_generics() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/generics", None)
    }

    #[test]
    #[serial]
    fn test_unused_struct_enum() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/unused_struct_enum", None)
    }

    #[allow(clippy::type_complexity)]
    fn body(
        fixture_name: &str,
        rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
    ) -> anyhow::Result<()> {
        let (actual_ir, rust_crate_dir) = execute_parse(fixture_name, rust_input_namespace_pack)?;
        json_golden_test(
            &serde_json::to_value(actual_ir)?,
            &rust_crate_dir.join("expect_mir.json"),
            &[],
        )?;

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn execute_parse(
        fixture_name: &str,
        rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
    ) -> anyhow::Result<(MirPack, PathBuf)> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        let rust_crate_dir = test_fixture_dir.clone();
        info!("test_fixture_dir={test_fixture_dir:?}");

        let rust_input_namespace_pack = rust_input_namespace_pack
            .map(|f| f(&rust_crate_dir))
            .unwrap_or(RustInputNamespacePack {
                rust_input_namespace_prefixes: vec![Namespace::new_self_crate("api".to_owned())],
                rust_output_path_namespace: Namespace::new_self_crate("frb_generated".to_owned()),
            });

        let config = ParserInternalConfig {
            hir: ParserHirInternalConfig {
                rust_input_namespace_pack: rust_input_namespace_pack.clone(),
                rust_crate_dir: rust_crate_dir.clone(),
                third_party_crate_names: vec![],
            },
            mir: ParserMirInternalConfig {
                rust_input_namespace_pack: rust_input_namespace_pack.clone(),
                force_codec_mode_pack: compute_force_codec_mode_pack(true),
                default_stream_sink_codec: CodecMode::Dco,
                default_rust_opaque_codec: RustOpaqueCodecMode::Nom,
                stop_on_error: true,
                enable_lifetime: false,
                type_64bit_int: false,
                default_dart_async: true,
            },
        };

        let pack = parse_inner(
            &config,
            &Dumper::new(&Default::default()),
            &GeneratorProgressBarPack::new(),
            |hir_flat| {
                json_golden_test(
                    &serde_json::to_value(hir_flat).unwrap(),
                    &rust_crate_dir.join("expect_hir_flat.json"),
                    &create_path_sanitizers(&test_fixture_dir),
                )
            },
        )?;

        Ok((pack, rust_crate_dir))
    }
}
