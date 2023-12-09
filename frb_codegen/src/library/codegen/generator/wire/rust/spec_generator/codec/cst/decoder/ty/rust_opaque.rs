use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use std::borrow::Cow;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for RustOpaqueWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(generalized_rust_opaque_generate_impl_decode_body().into()),
            ..Default::default()
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<Cow<str>> {
        Some(generalized_rust_opaque_generate_impl_decode_body().into())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        dart_opaque_or_generalized_rust_opaque_rust_wire_type(target)
    }
}

pub(super) fn generalized_rust_opaque_generate_impl_decode_body() -> &'static str {
    r#"unsafe { flutter_rust_bridge::for_generated::cst_decode_rust_opaque(self) }"#
}

pub(super) fn dart_opaque_or_generalized_rust_opaque_rust_wire_type(target: Target) -> String {
    match target {
        Target::Io => "*const std::ffi::c_void",
        Target::Wasm => JS_VALUE,
    }
    .into()
}
