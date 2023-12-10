use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;

impl<'a> WireDartGeneratorMiscTrait for DartFnWireDartGenerator<'a> {
    fn generate_extra_functions(&self) -> Option<Acc<WireDartOutputCode>> {
        let api_impl_body = format!(
            "
            "
        );
        Some(Acc::new_common(WireDartOutputCode {
            api_impl_body,
            ..Default::default()
        }))
    }
}
