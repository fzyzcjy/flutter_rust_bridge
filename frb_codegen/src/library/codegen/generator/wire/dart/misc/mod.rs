use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;

pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecMisc {
    needs_freezed: bool,
}

pub(super) fn generate(
    _context: WireDartGeneratorContext,
    _cache: &IrPackComputedCache,
) -> WireDartOutputSpecMisc {
    todo!()
}
