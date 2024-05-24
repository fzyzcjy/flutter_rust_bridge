use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::rust_opaque::{
    generate_generalized_rust_opaque_decode, generate_generalized_rust_opaque_encode,
};
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::ir::func::OwnershipMode;
use convert_case::{Case, Casing};

impl<'a> CodecSseTyTrait for RustAutoOpaqueImplicitCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        match lang {
            Lang::DartLang(_) => {
                let needs_move = self.ir.needs_move();
                Some(generate_generalized_rust_opaque_encode(
                    lang,
                    &format!("{needs_move}"),
                ))
            }
            Lang::RustLang(_) => {
                if self.ir.ownership_mode == OwnershipMode::Owned {
                    Some(simple_delegate_encode(
                        lang,
                        &RustOpaque(self.ir.inner.to_owned()),
                        &generate_encode_rust_auto_opaque(&self.ir, "self"),
                    ))
                } else {
                    None
                }
            }
        }
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        match lang {
            Lang::DartLang(_) => Some(generate_generalized_rust_opaque_decode(
                lang,
                self.ir.clone().into(),
                self.ir.inner.codec,
                self.context,
            )),
            Lang::RustLang(_) => {
                if self.ir.ownership_mode == OwnershipMode::Owned {
                    Some(simple_delegate_decode(
                        lang,
                        &RustOpaque(self.ir.inner.to_owned()),
                        &generate_decode_rust_auto_opaque(&self.ir, "inner"),
                    ))
                } else {
                    None
                }
            }
        }
    }
}

pub(crate) fn generate_encode_rust_auto_opaque(
    ir: &IrTypeRustAutoOpaqueImplicit,
    variable: &str,
) -> String {
    let arc = ir.inner.codec.arc_ty();
    format!(
        "flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, {arc}<_>>({variable})"
    )
}

pub(crate) fn generate_decode_rust_auto_opaque(
    ir: &IrTypeRustAutoOpaqueImplicit,
    variable: &str,
) -> String {
    format!(
        "{variable}.rust_auto_opaque_decode_{}()",
        ir.ownership_mode.to_string().to_case(Case::Snake)
    )
}
