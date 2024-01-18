use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::ty::rust_auto_opaque::generate_decode_rust_auto_opaque;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::rust_opaque::generalized_rust_opaque_rust_wire_type;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::IrTypeTrait;
use std::borrow::Cow;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for RustAutoOpaqueWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(generate_decode(&self.ir)),
            ..Default::default()
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<Cow<str>> {
        Some(generate_decode(&self.ir).into())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        generalized_rust_opaque_rust_wire_type(target)
    }
}

fn generate_decode(ir: &IrTypeRustAutoOpaque) -> String {
    let inner = format!(
        "CstDecode::<{}>::cst_decode(self)",
        ir.inner.rust_api_type()
    );
    generate_decode_rust_auto_opaque(ir, &inner)
}
