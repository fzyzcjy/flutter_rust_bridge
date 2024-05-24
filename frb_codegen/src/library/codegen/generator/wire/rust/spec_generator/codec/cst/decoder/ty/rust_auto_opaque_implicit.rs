use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::ty::rust_auto_opaque_implicit::generate_decode_rust_auto_opaque;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::rust_opaque::generalized_rust_opaque_rust_wire_type;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::func::OwnershipMode;
use crate::codegen::ir::ty::rust_auto_opaque_implicit::IrTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::ty::IrTypeTrait;
use std::borrow::Cow;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for RustAutoOpaqueImplicitWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc {
            io: generate_decode(&self.ir),
            ..Default::default()
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<Cow<str>> {
        generate_decode(&self.ir).map(Cow::from)
    }

    fn rust_wire_type(&self, target: Target) -> String {
        generalized_rust_opaque_rust_wire_type(target)
    }
}

fn generate_decode(ir: &IrTypeRustAutoOpaqueImplicit) -> Option<String> {
    if ir.ownership_mode == OwnershipMode::Owned {
        let inner = format!(
            "CstDecode::<{}>::cst_decode(self)",
            ir.inner.rust_api_type()
        );
        Some(generate_decode_rust_auto_opaque(ir, &inner))
    } else {
        None
    }
}
