use crate::codegen::generator::codec::sse::ty::rust_auto_opaque_implicit::generate_encode_rust_auto_opaque;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::mir::func::OwnershipMode;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait
    for RustAutoOpaqueImplicitWireRustCodecDcoGenerator<'a>
{
    fn generate_impl_into_dart(&self) -> Option<String> {
        if self.mir.ownership_mode == OwnershipMode::Owned {
            let rust_api_type = self.mir.rust_api_type();
            let local_struct_type = rust_auto_opaque_local_struct_type(&self.mir);
            let body = format!(
                "{}.into_dart()",
                generate_encode_rust_auto_opaque(&self.mir, "self.0")
            );
            Some(format!(
                r###"
                {}
                {}
                "###,
                generate_impl_into_dart(&local_struct_type, &body),
                generate_impl_into_into_dart(&rust_api_type, &Some(local_struct_type))
            ))
        } else {
            None
        }
    }
}

// Similar to "mirror"
fn rust_auto_opaque_local_struct_type(mir: &MirTypeRustAutoOpaqueImplicit) -> String {
    format!("FrbWrapper<{}>", mir.rust_api_type())
}
