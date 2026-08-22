use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::rust_opaque::rust_opaque_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl WireDartCodecCstGeneratorEncoderTrait for RustAutoOpaqueImplicitWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        let needs_move = self.mir.needs_move();
        Acc::new_common(Some(format!(
            "// ignore: invalid_use_of_internal_member
            return (raw as {}Impl).frbInternalCstEncode(move: {needs_move});",
            ApiDartGenerator::new(self.mir.clone(), self.context.as_api_dart_context())
                .dart_api_type(),
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        rust_opaque_dart_wire_type(target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::func::OwnershipMode;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
        MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
    };
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::utils::namespace::Namespace;

    /// Requests a move only for owned implicit opaque values during CST encoding.
    #[test]
    fn implicit_rust_opaque_encoder_preserves_ownership_move_flag() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(false);
        let wire_rust_config = test_utils::wire_rust_config(false);
        let inner = MirTypeRustOpaque {
            namespace: Namespace::default(),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("crate::api::Handle".into())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: Some("Handle".into()),
            brief_name: false,
        };
        let generator = RustAutoOpaqueImplicitWireDartCodecCstGenerator::new(
            MirTypeRustAutoOpaqueImplicit {
                ownership_mode: OwnershipMode::Owned,
                inner,
                raw: MirRustAutoOpaqueRaw {
                    string: MirLifetimeAwareType::new("crate::api::Handle".into()),
                    segments: vec![],
                },
                reason: None,
                ignore: false,
            },
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );

        assert_eq!(
            generator.generate_encode_func_body().common.as_deref(),
            Some(
                "// ignore: invalid_use_of_internal_member\n            return (raw as HandleImpl).frbInternalCstEncode(move: true);"
            )
        );
    }
}
