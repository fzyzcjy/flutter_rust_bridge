use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl WireDartCodecCstGeneratorEncoderTrait for DartOpaqueWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(
            "return encodeDartOpaque(raw, portManager.dartHandlerPort, generalizedFrbRustBinding);"
                .to_owned(),
        ))
    }

    fn dart_wire_type(&self, _target: Target) -> String {
        "PlatformPointer".into()
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::ty::dart_opaque::MirTypeDartOpaque;

    /// Encodes Dart opaque values through the handler port and platform binding.
    #[test]
    fn dart_opaque_encoder_emits_handler_port_call() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(false);
        let wire_rust_config = test_utils::wire_rust_config(false);
        let generator = DartOpaqueWireDartCodecCstGenerator::new(
            MirTypeDartOpaque,
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );

        assert_eq!(generator.generate_encode_func_body().common.as_deref(), Some("return encodeDartOpaque(raw, portManager.dartHandlerPort, generalizedFrbRustBinding);"));
        assert_eq!(generator.dart_wire_type(Target::Io), "PlatformPointer");
    }
}
