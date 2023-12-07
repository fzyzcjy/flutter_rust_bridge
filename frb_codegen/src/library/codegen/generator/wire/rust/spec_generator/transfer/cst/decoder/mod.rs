use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use serde::Serialize;

mod impl_new_with_nullptr;
mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireDartOutputSpecTransferCstDecoder {
    pub allocate_funcs: Acc<Vec<WireRustOutputCode>>,
    pub related_funcs: Acc<Vec<WireRustOutputCode>>,
    pub impl_wire2api: Acc<Vec<WireRustOutputCode>>,
    pub wire2api_class: Acc<Vec<WireRustOutputCode>>,
    pub impl_new_with_nullptr: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecTransferCstDecoder {
    WireDartOutputSpecTransferCstDecoder {
        allocate_funcs: cache
            .distinct_input_types
            .iter()
            .map(|ty| WireRustGenerator::new(ty.clone(), context).generate_allocate_funcs())
            .collect(),
        related_funcs: cache
            .distinct_types
            .iter()
            .map(|ty| WireRustGenerator::new(ty.clone(), context).generate_related_funcs())
            .collect(),
        impl_wire2api: generate_impl_wire2api(&cache.distinct_input_types, context),
        wire2api_class: Acc::new_io(
            cache
                .distinct_input_types
                .iter()
                .filter_map(|ty| {
                    WireRustGenerator::new(ty.clone(), context).generate_wire2api_class()
                })
                .map(|x| x.into())
                .collect(),
        ),
        impl_new_with_nullptr: Acc::new_io(generate_impl_new_with_nullptr(
            &cache.distinct_input_types,
            context,
        )),
    }
}
