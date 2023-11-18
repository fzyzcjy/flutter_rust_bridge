use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;

impl<'a> WireDartGeneratorApi2wireTrait for UnencodableWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        unreachable!()
    }
}
