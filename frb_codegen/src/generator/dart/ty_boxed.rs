use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::IrType::{Primitive, StructRef};
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeBoxedGenerator(pub IrTypeBoxed);

impl TypeDartGeneratorTrait for TypeBoxedGenerator {
    fn api2wire_body(&self) -> String {
        match &*self.0.inner {
            Primitive(_) => {
                format!("return inner.new_{}(raw);", self.0.safe_ident())
            }
            inner => {
                format!(
                    "final ptr = inner.new_{}();
                    _api_fill_to_wire_{}(raw, ptr.ref);
                    return ptr;",
                    self.0.safe_ident(),
                    inner.safe_ident(),
                )
            }
        }
    }

    fn api_fill_to_wire_body(&self) -> String {
        if !matches!(*self.0.inner, Primitive(_)) {
            format!(
                " _api_fill_to_wire_{}(apiObj, wireObj.ref);",
                self.0.inner.safe_ident()
            )
        } else {
            "".to_string()
        }
    }

    fn wire2api_body(&self) -> String {
        match &*self.0.inner {
            StructRef(inner) => format!("return _wire2api_{}(raw);", inner.safe_ident()),
            _ => gen_wire2api_simple_type_cast(&self.0.dart_api_type()),
        }
    }
}
