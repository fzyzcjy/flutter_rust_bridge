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
            let rust_api_type = self.ir.rust_api_type();
            let local_struct_type = format!("Local_{}", self.ir.safe_ident()); // Similar to "mirror"
            let body = format!(
                "{}.into_dart()",
                generate_encode_rust_auto_opaque(&self.ir, "self.0")
            );
            Some(
                format!("struct {local_struct_type}({rust_api_type});\n")
                    + &generate_impl_into_dart(&local_struct_type, &body)
                    + &generate_impl_into_into_dart(&rust_api_type, &Some(local_struct_type)),
            )
        } else {
            None
        }
    }
}
