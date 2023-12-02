use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::library::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use serde::Serialize;

mod misc;
pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireRustOutputSpecRust2Dart {
    pub impl_into_dart: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireRustOutputSpecRust2Dart {
    WireRustOutputSpecRust2Dart {
        impl_into_dart: cache
            .distinct_output_types
            .iter()
            .filter_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_impl_into_dart())
            .map(|x| Acc::new_common(x.into()))
            .collect(),
    }
}
