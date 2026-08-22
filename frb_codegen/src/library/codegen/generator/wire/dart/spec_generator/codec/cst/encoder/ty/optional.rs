use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl WireDartCodecCstGeneratorEncoderTrait for OptionalWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Web => Some(format!(
                "return raw == null ? {} : cst_encode_{}(raw);",
                if target == TargetOrCommon::Web {
                    "null"
                } else {
                    "ffi.nullptr"
                },
                self.mir.inner.safe_ident()
            )),
            _ => None,
        })
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Web {
            format!(
                "{}?",
                WireDartCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                    .dart_wire_type(target)
            )
        } else {
            WireDartCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                .dart_wire_type(target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::ty::optional::MirTypeOptional;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

    /// Emits platform-specific null sentinels before encoding optional inner values.
    #[test]
    fn optional_encoder_uses_pointer_and_web_null_sentinels() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let generator = OptionalWireDartCodecCstGenerator::new(
            MirTypeOptional::new(MirType::Primitive(MirTypePrimitive::I32)),
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );

        let output = generator.generate_encode_func_body();
        assert_eq!(
            output.io.as_deref(),
            Some("return raw == null ? ffi.nullptr : cst_encode_i_32(raw);")
        );
        assert_eq!(
            output.web.as_deref(),
            Some("return raw == null ? null : cst_encode_i_32(raw);")
        );
    }
}
