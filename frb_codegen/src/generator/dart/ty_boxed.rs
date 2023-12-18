use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::IrType::{DartOpaque, Delegate, EnumRef, Primitive, RustOpaque, StructRef};
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

#[cfg(feature = "chrono")]
use super::gen_wire2api_chrono;

type_dart_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

fn is_empty_struct(ty: &TypeBoxedGenerator) -> bool {
    if let StructRef(ref s) = ty.ir.inner.as_ref() {
        let s = s.get(ty.context.ir_file);
        s.fields.is_empty()
    } else {
        false
    }
}

impl TypeDartGeneratorTrait for TypeBoxedGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let as_primitive = self.ir.inner.is_primitive().then(|| {
            format!(
                "return inner.new_{}(api2wire_{}(raw));",
                self.ir.safe_ident(),
                self.ir.inner.safe_ident()
            )
        });
        let ident = self.ir.safe_ident();
        let inner = self.ir.inner.safe_ident();
        let empty_struct = is_empty_struct(self);
        Acc {
            io: Some(as_primitive.unwrap_or_else(|| {
                if self.ir.inner.is_array() {
                    format!("return api2wire_{inner}(raw);")
                } else if empty_struct {
                    format!(
                        "final ptr = inner.new_{ident}();
                        return ptr;",
                    )
                } else {
                    let prefix = if self.context.config.shared { "" } else { "_" };
                    format!(
                        "final ptr = inner.new_{ident}();
                        {prefix}api_fill_to_wire_{inner}(raw, ptr.ref);
                        return ptr;"
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
        (!self.ir.inner.is_primitive() && !is_empty_struct(self)).then(|| {
            let prefix = if self.context.config.shared { "" } else { "_" };
            format!(
                "{}api_fill_to_wire_{}(apiObj, wireObj.ref);",
                prefix,
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
                format!(
                    "return {}wire2api_{}(raw);",
                    self.get_private_prefix(),
                    self.ir.inner.safe_ident()
                )
            }
            #[cfg(feature = "chrono")]
            Delegate(IrTypeDelegate::Time(time)) => gen_wire2api_chrono(time),
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
