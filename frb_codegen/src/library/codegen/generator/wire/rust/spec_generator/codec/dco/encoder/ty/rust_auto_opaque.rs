use crate::codegen::generator::codec::sse::ty::rust_auto_opaque::generate_encode_rust_auto_opaque;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::rust_auto_opaque::{IrTypeRustAutoOpaque, OwnershipMode};
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for RustAutoOpaqueWireRustCodecDcoGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        if self.ir.ownership_mode == OwnershipMode::Owned {
            let rust_api_type = self.ir.rust_api_type();
            let local_struct_type = rust_auto_opaque_local_struct_type(&self.ir);
            let body = format!(
                "{}.into_dart()",
                generate_encode_rust_auto_opaque(&self.ir, "self.0")
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

    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
        if self.ir.ownership_mode == OwnershipMode::Owned {
            rust_auto_opaque_local_struct_type(&self.ir)
        } else {
            // We do not generate IntoDart for this, so this should not be called
            // frb-coverage:ignore-start
            unreachable!()
            // frb-coverage:ignore-end
        }
    }
}

// Similar to "mirror"
fn rust_auto_opaque_local_struct_type(ir: &IrTypeRustAutoOpaque) -> String {
    format!("FrbWrapper<{}>", ir.rust_api_type())
}
