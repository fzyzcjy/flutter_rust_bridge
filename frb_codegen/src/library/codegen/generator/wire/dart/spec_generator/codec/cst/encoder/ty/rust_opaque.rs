use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ApiDartGenerator;

impl WireDartCodecCstGeneratorEncoderTrait for RustOpaqueWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(format!(
            "// ignore: invalid_use_of_internal_member
                return (raw as {}Impl).frbInternalCstEncode();",
            ApiDartGenerator::new(self.mir.clone(), self.context.as_api_dart_context())
                .dart_api_type(),
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        rust_opaque_dart_wire_type(target)
    }
}

pub(super) fn rust_opaque_dart_wire_type(target: Target) -> String {
    match target {
        Target::Io => "int",
        // Target::Web => "Object",
        Target::Web => "int",
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::utils::namespace::Namespace;

    fn opaque() -> MirTypeRustOpaque {
        MirTypeRustOpaque {
            namespace: Namespace::default(),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("crate::api::Handle".into())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: Some("Handle".into()),
            brief_name: false,
        }
    }

    /// Uses an integer handle for both supported Dart targets.
    #[test]
    fn rust_opaque_wire_type_is_an_integer_handle_on_every_target() {
        assert_eq!(rust_opaque_dart_wire_type(Target::Io), "int");
        assert_eq!(rust_opaque_dart_wire_type(Target::Web), "int");
    }

    /// Encodes Rust opaque values through their generated API implementation.
    #[test]
    fn rust_opaque_encoder_calls_api_implementation() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(false);
        let wire_rust_config = test_utils::wire_rust_config(false);
        let generator = RustOpaqueWireDartCodecCstGenerator::new(
            opaque(),
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
                "// ignore: invalid_use_of_internal_member\n                return (raw as HandleImpl).frbInternalCstEncode();"
            )
        );
    }
}
