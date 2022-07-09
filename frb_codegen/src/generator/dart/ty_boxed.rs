use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::IrType::{EnumRef, StructRef};
use crate::ir::*;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeDartGeneratorTrait for TypeBoxedGenerator<'_> {
    fn api2wire_body(&self, block_index: BlockIndex) -> Option<String> {
        if self.ir.inner.is_primitive() {
            Some(format!(
                "return inner.new_{}_{}(_api2wire_{}(raw));",
                self.ir.safe_ident(),
                block_index,
                self.ir.inner.safe_ident(),
            ))
        } else {
            Some(format!(
                "final ptr = inner.new_{}_{}();
                _api_fill_to_wire_{}(raw, ptr.ref);
                return ptr;",
                self.ir.safe_ident(),
                block_index,
                self.ir.inner.safe_ident(),
            ))
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        (!self.ir.inner.is_primitive()).then(|| {
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
