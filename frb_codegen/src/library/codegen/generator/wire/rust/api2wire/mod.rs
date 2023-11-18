use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::misc::section_header_comment;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::codegen::ir::pack::IrPack;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> Acc<Vec<String>> {
    let mut lines = Acc::<Vec<_>>::default();

    lines.push(section_header_comment("impl IntoDart"));
    lines.extend(
        cache
            .distinct_output_types
            .iter()
            .map(|ty| WireRustGenerator::new(ty, context).generate_impl_into_dart()),
    );

    lines
}
