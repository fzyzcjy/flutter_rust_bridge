use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;

pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecApi2wire {
    api2wire_funcs: Acc<Vec<WireDartOutputCode>>,
    api_fill_to_wire_funcs: Acc<Vec<WireDartOutputCode>>,
}

pub(super) fn generate(
    _context: WireDartGeneratorContext,
    _cache: &IrPackComputedCache,
) -> WireDartOutputSpecApi2wire {
    todo!()
}
