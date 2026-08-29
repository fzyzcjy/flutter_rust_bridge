use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

impl WireDartCodecDcoGeneratorDecoderTrait for PrimitiveWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        match self.mir {
            MirTypePrimitive::Unit => "return;".to_owned(),
            MirTypePrimitive::I64 | MirTypePrimitive::Isize => {
                "return dcoDecodeI64(raw);".to_owned()
            }
            MirTypePrimitive::U64 | MirTypePrimitive::Usize => {
                "return dcoDecodeU64(raw);".to_owned()
            }
            _ => gen_decode_simple_type_cast(self.mir.clone().into(), self.context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;

    /// Emits the dedicated scalar and fallback casts for every primitive family.
    #[test]
    fn primitive_decoder_covers_unit_integer_and_simple_cast_branches() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let context = test_utils::context(&pack, &config);

        for (primitive, expected) in [
            (MirTypePrimitive::Unit, "return;"),
            (MirTypePrimitive::I64, "return dcoDecodeI64(raw);"),
            (MirTypePrimitive::Isize, "return dcoDecodeI64(raw);"),
            (MirTypePrimitive::U64, "return dcoDecodeU64(raw);"),
            (MirTypePrimitive::Usize, "return dcoDecodeU64(raw);"),
            (MirTypePrimitive::I32, "return raw as int;"),
        ] {
            let generator = PrimitiveWireDartCodecDcoGenerator::new(primitive, context);
            assert_eq!(generator.generate_impl_decode_body(), expected);
        }
    }
}
