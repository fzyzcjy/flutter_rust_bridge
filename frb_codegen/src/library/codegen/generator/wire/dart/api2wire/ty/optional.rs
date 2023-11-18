use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorApi2wireTrait for OptionalWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            Target::Io | Target::Wasm => Some(format!(
                "return raw == null ? {} : api2wire_{}(raw);",
                if target == Target::Wasm {
                    "null"
                } else {
                    "ffi.nullptr"
                },
                self.ir.inner.safe_ident()
            )),
            _ => None,
        })
    }
}
