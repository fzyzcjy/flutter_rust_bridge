use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::path_texts::{PathText, PathTexts};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::utils::basic_code::general_code::GeneralCode;

pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireRustOutput {
    pub output_texts: PathTexts,
    pub extern_funcs: Vec<ExternFunc>,
    pub content_hash: i32,
    pub extern_struct_names: Vec<String>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    dumper: &Dumper,
) -> anyhow::Result<GeneratorWireRustOutput> {
    let spec = spec_generator::generate(context, dumper)?;
    (dumper.with_content(ConfigDumpContent::GeneratorSpec)).dump("wire_rust.json", &spec)?;

    let text = text_generator::generate(&spec, context.config)?;
    (dumper.with_content(ConfigDumpContent::GeneratorText)).dump("wire_rust.rs", &text.text)?;

    Ok(GeneratorWireRustOutput {
        output_texts: PathTexts(vec![PathText {
            path: context.config.rust_output_path.clone(),
            text: GeneralCode::new_rust(text.text),
        }]),
        extern_funcs: text.extern_funcs,
        content_hash: spec.misc.content_hash,
        extern_struct_names: spec.extern_struct_names,
    })
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::config::MetaConfig;
    use crate::codegen::config::internal_config::InternalConfig;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::generator::wire::rust::generate;
    use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::Config;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::get_test_fixture_dir;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn test_native_only_dto_i64_type_64bit_int() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir =
            get_test_fixture_dir("library/codegen/generator/wire/rust/mod/native_only_dto_i64");
        env::set_current_dir(&test_fixture_dir)?;

        let config = Config::from_files_auto()?;
        let internal_config = InternalConfig::parse(&config, &MetaConfig { watch: false })?;
        let mir_pack = crate::codegen::parser::parse(
            &internal_config.parser,
            &Dumper::new(&Default::default()),
            &GeneratorProgressBarPack::new(),
        )?;
        let actual = generate(
            WireRustGeneratorContext {
                mir_pack: &mir_pack,
                config: &internal_config.generator.wire.rust,
                wire_dart_config: &internal_config.generator.wire.dart,
                api_dart_config: &internal_config.generator.api_dart,
            },
            &Dumper::new(&Default::default()),
        )?;

        let raw_text = actual.output_texts.0[0].text.all_code();
        assert!(raw_text.contains("maybe_value: *mut i64"), "{raw_text}");
        assert!(
            !raw_text.contains("impl CstDecode<Option<i64>> for *mut Option<i64>"),
            "{raw_text}"
        );
        assert!(
            raw_text.contains("new_leak_vec_ptr(Default::default(), len)"),
            "{raw_text}"
        );
        assert!(
            !raw_text.contains("<i64>::new_with_null_ptr()"),
            "{raw_text}"
        );

        Ok(())
    }
}
