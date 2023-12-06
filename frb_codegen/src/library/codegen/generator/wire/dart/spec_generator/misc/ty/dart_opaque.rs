use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;

impl<'a> WireDartGeneratorMiscTrait for DartOpaqueWireDartGenerator<'a> {
    fn generate_extra_functions(&self) -> Option<Acc<WireDartOutputCode>> {
        Some(Acc {
            io: "".into(),
            ..Default::default()
        })
    }
}
