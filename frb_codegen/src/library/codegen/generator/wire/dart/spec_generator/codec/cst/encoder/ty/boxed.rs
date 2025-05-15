use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::primitive::dart_native_type_of_primitive;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::WireRustCodecCstGenerator;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType::StructRef;
use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl WireDartCodecCstGeneratorEncoderTrait for BoxedWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        let mir_safe_ident = self.mir.safe_ident();
        let inner_safe_ident = self.mir.inner.safe_ident();
        let empty_struct = is_empty_struct(self);

        Acc {
            io: Some(
                if self.mir.inner.is_primitive()
                    || matches!(
                        *self.mir.inner,
                        MirType::RustOpaque(_)
                            | MirType::RustAutoOpaqueImplicit(_)
                            | MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(_))
                            | MirType::DartOpaque(_)
                    )
                {
                    format!(
                        "return wire.cst_new_{mir_safe_ident}(cst_encode_{inner_safe_ident}(raw));"
                    )
                } else if self.mir.inner.is_array() {
                    format!("return cst_encode_{inner_safe_ident}(raw);")
                } else {
                    format!(
                        "final ptr = wire.cst_new_{mir_safe_ident}();
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
        let inner_safe_ident = self.mir.inner.safe_ident();

        if self.mir.inner.is_array() {
            Some(format!("wireObj = cst_encode_{inner_safe_ident}(apiObj);"))
        } else if !self.mir.inner.is_primitive()
            && !matches!(
                *self.mir.inner,
                MirType::RustOpaque(_)
                    | MirType::RustAutoOpaqueImplicit(_)
                    | MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(_))
                    | MirType::DartOpaque(_)
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
                if is_js_value(&self.mir.inner)
                    || self.mir.inner.is_array()
                    || self.mir.inner.is_primitive()
                {
                    WireDartCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                        .dart_wire_type(target)
                } else {
                    format!(
                        "int /* *{} */",
                        WireRustCodecCstGenerator::new(
                            self.mir.inner.clone(),
                            self.context.as_wire_rust_context()
                        )
                        .rust_wire_type(target)
                    )
                }
            }
            Target::Io => {
                if self.mir.inner.is_array() {
                    return WireDartCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                        .dart_wire_type(Target::Io);
                }
                let wire_type = self
                    .mir
                    .inner
                    .as_primitive()
                    .map(|prim| dart_native_type_of_primitive(prim).to_owned())
                    .unwrap_or_else(|| {
                        WireDartCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                            .dart_wire_type(target)
                    });
                format!("ffi.Pointer<{wire_type}>")
            }
        }
    }
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn is_empty_struct(ty: &BoxedWireDartCodecCstGenerator) -> bool {
    // frb-coverage:ignore-end
    if let StructRef(ref s) = ty.mir.inner.as_ref() {
        s.get(ty.context.mir_pack).fields.is_empty()
    } else {
        false
    }
}
