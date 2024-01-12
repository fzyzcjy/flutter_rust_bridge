use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use convert_case::{Case, Casing};
use std::borrow::Cow;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for RustOpaqueWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "unsafe {{ flutter_rust_bridge::for_generated::decode_rust_opaque_{}(self as _) }}",
                self.ir.codec.to_string().to_case(Case::Snake),
            )),
            ..Default::default()
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<Cow<str>> {
        Some(
            format!(
                r#"
                #[cfg(target_pointer_width = "64")]
                {{ compile_error!("64-bit pointers are not supported."); }}
                unsafe {{ flutter_rust_bridge::for_generated::decode_rust_opaque_{}((self.as_f64().unwrap() as usize) as _) }}
                "#,
                self.ir.codec.to_string().to_case(Case::Snake),
            )
            .into(),
        )
    }

    fn rust_wire_type(&self, target: Target) -> String {
        generalized_rust_opaque_rust_wire_type(target)
    }
}

pub(super) fn generalized_rust_opaque_rust_wire_type(target: Target) -> String {
    match target {
        Target::Io => "usize",
        Target::Web => JS_VALUE,
    }
    .into()
}
