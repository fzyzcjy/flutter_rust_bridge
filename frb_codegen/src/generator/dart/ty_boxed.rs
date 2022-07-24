use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::IrType::{EnumRef, StructRef};
use crate::ir::*;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeDartGeneratorTrait for TypeBoxedGenerator<'_> {
    fn api2wire_body(&self) -> Option<String> {
        match (self.ir.inner.is_primitive(), self.context.config.wasm) {
            (true, _) => Some(format!(
                "return inner.new_{}_{}(_api2wire_{}(raw));",
                self.ir.safe_ident(),
                self.context.config.block_index,
                self.ir.inner.safe_ident(),
            )),
            (false, false) => Some(format!(
                "final ptr = inner.new_{}_{}();
                _api_fill_to_wire_{}(raw, ptr.ref);
                return ptr;",
                self.ir.safe_ident(),
                self.context.config.block_index,
                self.ir.inner.safe_ident(),
            )),
            (false, true) => Some("return const [];".into()),
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        (!self.ir.inner.is_primitive() && !self.context.config.wasm).then(|| {
            format!(
                " _api_fill_to_wire_{}(apiObj, wireObj.ref);",
                self.ir.inner.safe_ident()
            )
        })
    }

    fn wire2api_body(&self) -> String {
        match &*self.ir.inner {
            StructRef(inner) => format!("return _wire2api_{}(raw);", inner.safe_ident()),
            EnumRef(inner) => format!("return _wire2api_{}(raw);", inner.safe_ident()),
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
