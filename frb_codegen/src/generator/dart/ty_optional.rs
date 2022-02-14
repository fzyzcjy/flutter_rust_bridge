use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeOptionalGenerator(IrTypeOptional);

impl TypeDartGeneratorTrait for TypeOptionalGenerator {
    fn api2wire_body(&self) -> String {
        format!(
            "return raw == null ? ffi.nullptr : _api2wire_{}(raw);",
            self.0.inner.safe_ident()
        )
    }

    fn api_fill_to_wire_body(&self) -> String {
        if !self.0.needs_initialization() || self.0.is_list() {
            return String::new();
        }
        format!(
            "if (apiObj != null) _api_fill_to_wire_{}(apiObj, wireObj);",
            self.0.inner.safe_ident()
        )
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return raw == null ? null : _wire2api_{}(raw);",
            self.0.inner.safe_ident()
        )
    }
}
