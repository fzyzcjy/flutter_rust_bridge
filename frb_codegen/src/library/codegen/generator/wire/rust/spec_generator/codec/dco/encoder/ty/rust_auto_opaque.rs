use crate::codegen::generator::codec::sse::ty::rust_auto_opaque::generate_encode_rust_auto_opaque;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::ty::rust_auto_opaque::OwnershipMode;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for RustAutoOpaqueWireRustCodecDcoGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        if self.ir.ownership_mode == OwnershipMode::Owned {
            let name = self.ir.rust_api_type();
            let body = format!("{}.into_dart()", generate_encode_rust_auto_opaque(&self.ir));
            Some(
                generate_impl_into_dart(&name, &body) + &generate_impl_into_into_dart(&name, &None),
            )
        } else {
            None
        }
    }
}
