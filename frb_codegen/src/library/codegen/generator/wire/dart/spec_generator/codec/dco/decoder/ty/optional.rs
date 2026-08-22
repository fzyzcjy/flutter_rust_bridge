use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for OptionalWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        format!(
            "return raw == null ? null : dco_decode_{}(raw);",
            self.mir.inner.safe_ident()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::{optional::MirTypeOptional, MirType};

    /// Preserves null values and delegates non-null inputs to the inner decoder.
    #[test]
    fn optional_decoder_emits_null_guard_for_inner_type() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let generator = OptionalWireDartCodecDcoGenerator::new(
            MirTypeOptional::new(MirType::Primitive(MirTypePrimitive::I32)),
            test_utils::context(&pack, &config),
        );

        assert_eq!(
            generator.generate_impl_decode_body(),
            "return raw == null ? null : dco_decode_i_32(raw);"
        );
    }
}
