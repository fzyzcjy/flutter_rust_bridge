use crate::codegen::dumper::Dumper;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::WireDartOutputSpecDart2Rust;
use crate::codegen::generator::wire::dart::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::dart::spec_generator::misc::WireDartOutputSpecMisc;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::WireDartOutputSpecRust2Dart;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use serde::Serialize;
use std::path::PathBuf;

pub(crate) mod base;
pub mod dart2rust;
mod dump;
pub(crate) mod misc;
pub(crate) mod output_code;
pub mod rust2dart;
mod transfer;
pub(super) mod wire_class;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpec {
    pub(super) misc: WireDartOutputSpecMisc,
    pub(super) rust2dart: WireDartOutputSpecRust2Dart,
    pub(super) dart2rust: WireDartOutputSpecDart2Rust,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
    rust_extern_funcs: &[ExternFunc],
    dumper: &Dumper,
) -> anyhow::Result<WireDartOutputSpec> {
    let cache = IrPackComputedCache::compute(context.ir_pack);

    dumper.dump(
        GeneratorInfo,
        "wire_dart.json",
        &generate_dump_info(&cache, context),
    )?;

    Ok(WireDartOutputSpec {
        misc: misc::generate(
            context,
            &cache,
            c_file_content,
            api_dart_actual_output_paths,
            rust_extern_funcs,
        )?,
        rust2dart: rust2dart::generate(context, &cache),
        dart2rust: dart2rust::generate(context, &cache),
    })
}
