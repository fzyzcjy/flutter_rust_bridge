use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;

pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecWire2api {}

pub(super) fn generate(
    _context: WireDartGeneratorContext,
    _cache: &IrPackComputedCache,
) -> WireDartOutputSpecWire2api {
    todo!()
}
