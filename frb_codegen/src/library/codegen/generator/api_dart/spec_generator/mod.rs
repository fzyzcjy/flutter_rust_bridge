use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;
use itertools::Itertools;

pub(crate) mod base;
pub(crate) mod class;
pub(crate) mod function;
pub(crate) mod info;
pub(crate) mod misc;

pub(crate) struct ApiDartOutputSpec {
    funcs: Vec<String>,
    classes: Vec<String>,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorApiDartInternalConfig,
) -> Result<ApiDartOutputSpec> {
    let distinct_types = ir_pack.distinct_types(true, true);
    let context = ApiDartGeneratorContext { ir_pack, config };

    let funcs = ir_pack
        .funcs
        .iter()
        .map(|f| function::generate_func(f, context, config.dart_enums_style))
        .map(|func| {
            format!(
                "{}{}\n\n{}",
                func.func_comments, func.func_signature, func.companion_field_signature,
            )
        })
        .collect_vec();

    let classes = distinct_types
        .iter()
        .filter_map(|ty| ApiDartGenerator::new(ty.clone(), context).generate_class())
        .collect_vec();

    Ok(ApiDartOutputSpec { funcs, classes })
}
