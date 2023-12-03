use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::misc::ty::rust_opaque::generate_rust_arc_functions;
use crate::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;

impl<'a> WireDartGeneratorMiscTrait for RustAutoOpaqueWireDartGenerator<'a> {
    fn generate_extra_functions(&self) -> Option<Acc<WireDartOutputCode>> {
        Some(generate_rust_arc_functions(
            self.ir.clone().into(),
            self.context,
        ))
    }
}
