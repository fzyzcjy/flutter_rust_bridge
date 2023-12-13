use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::primitive::dart_native_type_of_primitive;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::WireRustCodecCstGenerator;
use crate::codegen::ir::ty::IrType::StructRef;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for BoxedWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
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
                    format!(
                        "return wire.cst_new_{ir_safe_ident}(cst_encode_{inner_safe_ident}(raw));"
                    )
                } else if self.ir.inner.is_array() {
                    format!("return cst_encode_{inner_safe_ident}(raw);")
                } else {
                    format!(
                        "final ptr = wire.cst_new_{ir_safe_ident}();
                    {}
                    return ptr;",
                        if empty_struct {
                            "".to_owned()
                        } else {
                            format!("cst_api_fill_to_wire_{inner_safe_ident}(raw, ptr.ref);")
                        }
                    )
                },
            ),
            web: Some(format!("return cst_encode_{inner_safe_ident}(raw);")),
            ..Default::default()
        }
    }

    fn generate_encode_api_fill_to_wire_body(&self) -> Option<String> {
        let inner_safe_ident = self.ir.inner.safe_ident();

        if self.ir.inner.is_array() {
            Some(format!("wireObj = cst_encode_{inner_safe_ident}(apiObj);"))
        } else if !self.ir.inner.is_primitive()
            && !matches!(
                *self.ir.inner,
                IrType::RustOpaque(_) | IrType::DartOpaque(_)
            )
            && !is_empty_struct(self)
        {
            Some(format!(
                "cst_api_fill_to_wire_{inner_safe_ident}(apiObj, wireObj.ref);"
            ))
        } else {
            None
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Web => {
                if is_js_value(&self.ir.inner)
                    || self.ir.inner.is_array()
                    || self.ir.inner.is_primitive()
                {
                    WireDartCodecCstGenerator::new(self.ir.inner.clone(), self.context)
                        .dart_wire_type(target)
                } else {
                    format!(
                        "int /* *{} */",
                        WireRustCodecCstGenerator::new(
                            self.ir.inner.clone(),
                            self.context.as_wire_rust_context()
                        )
                        .rust_wire_type(target)
                    )
                }
            }
            Target::Io => {
                if self.ir.inner.is_array() {
                    return WireDartCodecCstGenerator::new(self.ir.inner.clone(), self.context)
                        .dart_wire_type(Target::Io);
                }
                let wire_type = self
                    .ir
                    .inner
                    .as_primitive()
                    .map(|prim| dart_native_type_of_primitive(prim).to_owned())
                    .unwrap_or_else(|| {
                        WireDartCodecCstGenerator::new(self.ir.inner.clone(), self.context)
                            .dart_wire_type(target)
                    });
                format!("ffi.Pointer<{wire_type}>")
            }
        }
    }
}

fn is_empty_struct(ty: &BoxedWireDartCodecCstGenerator) -> bool {
    if let StructRef(ref s) = ty.ir.inner.as_ref() {
        s.get(ty.context.ir_pack).fields.is_empty()
    } else {
        false
    }
}
