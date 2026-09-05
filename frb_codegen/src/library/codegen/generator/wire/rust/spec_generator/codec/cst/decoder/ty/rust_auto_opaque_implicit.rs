use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::ty::rust_auto_opaque_implicit::generate_decode_rust_auto_opaque;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::rust_opaque::generalized_rust_opaque_rust_wire_type;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::mir::func::OwnershipMode;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use std::borrow::Cow;

impl WireRustCodecCstGeneratorDecoderTrait for RustAutoOpaqueImplicitWireRustCodecCstGenerator<'_> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc {
            io: generate_decode(&self.mir),
            ..Default::default()
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<Cow<'_, str>> {
        generate_decode(&self.mir).map(Cow::from)
    }

    fn rust_wire_type(&self, target: Target) -> String {
        generalized_rust_opaque_rust_wire_type(target)
    }
}

fn generate_decode(mir: &MirTypeRustAutoOpaqueImplicit) -> Option<String> {
    if mir.ownership_mode == OwnershipMode::Owned {
        let inner = format!(
            "CstDecode::<{}>::cst_decode(self)",
            mir.inner.rust_api_type()
        );
        Some(generate_decode_rust_auto_opaque(mir, &inner))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
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

    /// Generates decoding only for owned implicit opaque values.
    #[test]
    fn generate_decode_distinguishes_owned_and_borrowed_implicit_opaque_values() {
        let owned = generate_decode(&implicit_opaque(OwnershipMode::Owned))
            .expect("owned implicit opaque values must decode");

        assert!(owned.contains("CstDecode::<RustOpaqueNom<crate::api::Handle>>::cst_decode(self)"));
        assert_eq!(generate_decode(&implicit_opaque(OwnershipMode::Ref)), None);
        assert_eq!(
            generate_decode(&implicit_opaque(OwnershipMode::RefMut)),
            None
        );
    }
}
