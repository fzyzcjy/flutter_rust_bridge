use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::IrType::{EnumRef, Primitive, StructRef};
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeDartGeneratorTrait for TypeBoxedGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let as_primitive = self.ir.inner.is_primitive().then(|| {
            format!(
                "return inner.new_{}_{}(api2wire_{}(raw));",
                self.ir.safe_ident(),
                self.context.config.block_index,
                self.ir.inner.safe_ident(),
            )
        });
        Acc {
            io: Some(as_primitive.clone().unwrap_or_else(|| {
                format!(
                    "final ptr = inner.new_{}_{}();
                    _api_fill_to_wire_{}(raw, ptr.ref);
                    return ptr;",
                    self.ir.safe_ident(),
                    self.context.config.block_index,
                    self.ir.inner.safe_ident(),
                )
            })),
            wasm: Some(as_primitive.unwrap_or_else(|| {
                format!("return api2wire_{}(raw);", self.ir.inner.safe_ident())
            })),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        (!self.ir.inner.is_primitive()).then(|| {
            format!(
                "_api_fill_to_wire_{}(apiObj, wireObj.ref);",
                self.ir.inner.safe_ident()
            )
        })
    }

    fn wire2api_body(&self) -> String {
        match &*self.ir.inner {
            StructRef(_)
            | EnumRef(_)
            | Primitive(IrTypePrimitive::I64 | IrTypePrimitive::U64 | IrTypePrimitive::Usize) => {
                format!("return _wire2api_{}(raw);", self.ir.inner.safe_ident())
            }
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
