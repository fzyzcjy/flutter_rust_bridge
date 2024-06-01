use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::HirFlatCrate;
use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::parser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::mir::reader::read_rust_crate;

pub(crate) mod hir;
pub(crate) mod internal_config;
pub(crate) mod mir;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<MirPack> {
    parse_inner(config, dumper, progress_bar_pack, |_, _| {})
}

fn parse_inner(
    config: &ParserInternalConfig,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
    on_hir: impl FnOnce(&HirCrate, &HirFlatCrate) -> (),
) -> anyhow::Result<MirPack> {
    let pb = progress_bar_pack.parse_read.start();
    let file = read_rust_crate(&config.rust_crate_dir, dumper)?;
    drop(pb);

    let pb = progress_bar_pack.parse_hir.start();
    let hir_hierarchical = hir::hierarchical::parse(&config.hir, &file)?;
    let hir_flat = hir::flat::parse(&hir_hierarchical.root_module)?;
    on_hir(&hir_hierarchical, &hir_flat);
    drop(pb);

    let pb = progress_bar_pack.parse_mir.start();
    let mir_pack = mir::parse(&config.mir, &hir_flat)?;
    drop(pb);

    Ok(mir_pack)
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config_parser::compute_force_codec_mode_pack;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
    use crate::codegen::ir::mir::namespace::Namespace;
    use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
    use crate::codegen::parser::internal_config::ParserInternalConfig;
    use crate::codegen::parser::mir::internal_config::{
        ParserMirInternalConfig, RustInputNamespacePack,
    };
    use crate::codegen::parser::mir::reader::read_rust_crate;
    use crate::codegen::parser::{hir, parse_inner, MirPack};
    use crate::utils::logs::configure_opinionated_test_logging;
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
            Some(Box::new(|rust_crate_dir| {
                RustInputNamespacePack::new(vec![
                    Namespace::new_self_crate("api_one".to_owned()),
                    Namespace::new_self_crate("api_two".to_owned()),
                ])
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
            .unwrap_or(RustInputNamespacePack::new(vec![
                Namespace::new_self_crate("api".to_owned()),
            ]));

        let config = ParserInternalConfig {
            hir: ParserHirInternalConfig {
                rust_input_namespace_pack: rust_input_namespace_pack.clone(),
            },
            mir: ParserMirInternalConfig {
                rust_input_namespace_pack: rust_input_namespace_pack.clone(),
                force_codec_mode_pack: compute_force_codec_mode_pack(true),
                default_stream_sink_codec: CodecMode::Dco,
                default_rust_opaque_codec: RustOpaqueCodecMode::Nom,
            },
            rust_crate_dir: rust_crate_dir.clone(),
        };

        let pack = parse_inner(
            &config,
            &Dumper(&Default::default()),
            &GeneratorProgressBarPack::new(),
            |hir_hierarchical, _hir_flat| {
                json_golden_test(
                    &serde_json::to_value(hir_hierarchical).unwrap(),
                    &rust_crate_dir.join("expect_hir_hierarchical.json"),
                    &create_path_sanitizers(&test_fixture_dir),
                )
                .unwrap();
            },
        )?;

        Ok((pack, rust_crate_dir))
    }
}
