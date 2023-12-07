use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::primitive::dart_native_type_of_primitive;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::WireRustTransferCstGenerator;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::IrType::StructRef;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

impl<'a> WireDartTransferCstGeneratorEncoderTrait for BoxedWireDartTransferCstGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let ir_safe_ident = self.ir.safe_ident();
        let inner_safe_ident = self.ir.inner.safe_ident();
        let empty_struct = is_empty_struct(self);

        Acc {
            io: Some(
                if self.ir.inner.is_primitive()
                    || matches!(
                        *self.ir.inner,
                        IrType::RustOpaque(_) | IrType::DartOpaque(_)
                    )
                {
                    format!("return wire.new_{ir_safe_ident}(api2wire_{inner_safe_ident}(raw));")
                } else if self.ir.inner.is_array() {
                    format!("return api2wire_{inner_safe_ident}(raw);")
                } else {
                    format!(
                        "final ptr = wire.new_{ir_safe_ident}();
                    {}
                    return ptr;",
                        if empty_struct {
                            "".to_owned()
                        } else {
                            format!("_api_fill_to_wire_{inner_safe_ident}(raw, ptr.ref);")
                        }
                    )
                },
            ),
            wasm: Some(format!("return api2wire_{inner_safe_ident}(raw);")),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let inner_safe_ident = self.ir.inner.safe_ident();

        if self.ir.inner.is_array() {
            Some(format!("wireObj = api2wire_{inner_safe_ident}(apiObj);"))
        } else if !self.ir.inner.is_primitive()
            && !matches!(
                *self.ir.inner,
                IrType::RustOpaque(_) | IrType::DartOpaque(_)
            )
            && !is_empty_struct(self)
        {
            Some(format!(
                "_api_fill_to_wire_{inner_safe_ident}(apiObj, wireObj.ref);"
            ))
        } else {
            None
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Wasm => {
                if is_js_value(&self.ir.inner)
                    || self.ir.inner.is_array()
                    || self.ir.inner.is_primitive()
                {
                    WireDartTransferCstGenerator::new(self.ir.inner.clone(), self.context)
                        .dart_wire_type(target)
                } else {
                    format!(
                        "int /* *{} */",
                        WireRustTransferCstGenerator::new(
                            self.ir.inner.clone(),
                            self.context.as_wire_rust_context()
                        )
                        .rust_wire_type(target)
                    )
                }
            }
            Target::Io => {
                if self.ir.inner.is_array() {
                    return WireDartTransferCstGenerator::new(self.ir.inner.clone(), self.context)
                        .dart_wire_type(Target::Io);
                }
                let wire_type = self
                    .ir
                    .inner
                    .as_primitive()
                    .map(|prim| dart_native_type_of_primitive(prim).to_owned())
                    .unwrap_or_else(|| {
                        WireDartTransferCstGenerator::new(self.ir.inner.clone(), self.context)
                            .dart_wire_type(target)
                    });
                format!("ffi.Pointer<{wire_type}>")
            }
        }
    }
}

fn is_empty_struct(ty: &BoxedWireDartTransferCstGenerator) -> bool {
    if let StructRef(ref s) = ty.ir.inner.as_ref() {
        s.get(ty.context.ir_pack).fields.is_empty()
    } else {
        false
    }
}
