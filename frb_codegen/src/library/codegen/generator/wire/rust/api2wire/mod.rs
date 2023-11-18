use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::misc::section_header_comment;
use crate::codegen::generator::wire::rust::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::library::codegen::generator::wire::rust::api2wire::ty::WireRustGeneratorApi2wireTrait;

mod misc;
pub(crate) mod ty;

pub(super) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> Acc<Vec<WireRustOutputCode>> {
    let mut ans = Acc::<Vec<WireRustOutputCode>>::default();

    ans.push(section_header_comment("impl IntoDart"));
    ans.extend(
        cache
            .distinct_output_types
            .iter()
            .filter_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_impl_into_dart())
            .map(|x| x.into()),
    );

    ans
}
