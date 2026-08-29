use crate::codegen::generator::codec::sse::ty::rust_auto_opaque_implicit::generate_encode_rust_auto_opaque;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::mir::func::OwnershipMode;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl WireRustCodecDcoGeneratorEncoderTrait for RustAutoOpaqueImplicitWireRustCodecDcoGenerator<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::pack::MirPack;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirRustAutoOpaqueRaw;
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::utils::namespace::Namespace;

    fn implicit_opaque(ownership_mode: OwnershipMode) -> MirTypeRustAutoOpaqueImplicit {
        MirTypeRustAutoOpaqueImplicit {
            ownership_mode,
            inner: MirTypeRustOpaque {
                namespace: Namespace::default(),
                inner: MirRustOpaqueInner(MirLifetimeAwareType::new("crate::api::Handle".into())),
                codec: RustOpaqueCodecMode::Nom,
                dart_api_type: None,
                brief_name: false,
            },
            raw: MirRustAutoOpaqueRaw {
                string: MirLifetimeAwareType::new("crate::api::Handle".into()),
                segments: vec![],
            },
            reason: None,
            ignore: false,
        }
    }

    fn context(mir_pack: &MirPack) -> WireRustCodecDcoGeneratorContext<'_> {
        WireRustCodecDcoGeneratorContext { mir_pack }
    }

    /// Wraps the owned opaque API type with the local DCO conversion wrapper.
    #[test]
    fn rust_auto_opaque_local_struct_type_uses_owned_api_type() {
        assert_eq!(
            rust_auto_opaque_local_struct_type(&implicit_opaque(OwnershipMode::Owned)),
            "FrbWrapper<crate::api::Handle>"
        );
    }

    /// Generates DCO conversions only for owned implicit opaque values.
    #[test]
    fn generate_impl_into_dart_distinguishes_owned_and_borrowed_implicit_opaque_values() {
        let mir_pack = MirPack {
            funcs_all: vec![],
            extra_types_all: vec![],
            struct_pool: Default::default(),
            enum_pool: Default::default(),
            dart_code_of_type: Default::default(),
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: Default::default(),
        };
        let owned = RustAutoOpaqueImplicitWireRustCodecDcoGenerator::new(
            implicit_opaque(OwnershipMode::Owned),
            context(&mir_pack),
        )
        .generate_impl_into_dart()
        .expect("owned implicit opaque values must generate DCO conversions");

        assert!(owned.contains("IntoDart for FrbWrapper<crate::api::Handle>"));
        assert!(owned.contains("rust_auto_opaque_encode::<_, StdArc<_>>(self.0).into_dart()"));
        assert!(
            owned.contains("IntoIntoDart<FrbWrapper<crate::api::Handle>> for crate::api::Handle")
        );
        assert_eq!(
            RustAutoOpaqueImplicitWireRustCodecDcoGenerator::new(
                implicit_opaque(OwnershipMode::Ref),
                context(&mir_pack),
            )
            .generate_impl_into_dart(),
            None
        );
        assert_eq!(
            RustAutoOpaqueImplicitWireRustCodecDcoGenerator::new(
                implicit_opaque(OwnershipMode::RefMut),
                context(&mir_pack),
            )
            .generate_impl_into_dart(),
            None
        );
    }
}
