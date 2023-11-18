use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::misc::section_header_comment;
use crate::codegen::generator::wire::rust::wire_rust_code::WireRustCode;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::generator::wire::rust::api2wire::ty::WireRustGeneratorApi2wireTrait;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> Acc<WireRustCode> {
    let mut ans = Acc::default();

    ans.push(section_header_comment("impl IntoDart"));
    ans.extend(
        cache
            .distinct_output_types
            .iter()
            .filter_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_impl_into_dart()),
    );

    ans
}
