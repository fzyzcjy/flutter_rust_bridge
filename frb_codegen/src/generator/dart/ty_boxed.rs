use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::IrType::{DartOpaque, Delegate, EnumRef, Primitive, RustOpaque, StructRef};
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
        let ident = self.ir.safe_ident();
        let context = self.context.config.block_index;
        let inner = self.ir.inner.safe_ident();
        Acc {
            io: Some(as_primitive.unwrap_or_else(|| {
                if self.ir.inner.is_array() {
                    format!("return api2wire_{inner}(raw);")
                } else {
                    format!(
                        "final ptr = inner.new_{ident}_{context}();
                        _api_fill_to_wire_{inner}(raw, ptr.ref);
                        return ptr;",
                    )
                }
            })),
            wasm: Some(format!(
                "return api2wire_{}(raw);",
                self.ir.inner.safe_ident()
            )),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        if self.ir.inner.is_array() {
            return Some(format!(
                "wireObj = api2wire_{}(apiObj);",
                self.ir.inner.safe_ident()
            ));
        }
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
            | DartOpaque(_)
            | RustOpaque(_)
            | EnumRef(_)
            | Primitive(IrTypePrimitive::I64 | IrTypePrimitive::U64 | IrTypePrimitive::Usize)
            | Delegate(IrTypeDelegate::Array(_) | IrTypeDelegate::PrimitiveEnum { .. }) => {
                format!("return _wire2api_{}(raw);", self.ir.inner.safe_ident())
            }
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
