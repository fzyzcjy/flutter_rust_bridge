use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

impl WireDartCodecDcoGeneratorDecoderTrait for PrimitiveListWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        match &self.mir.primitive {
            MirTypePrimitive::I64 => "return dcoDecodeInt64List(raw);".into(),
            MirTypePrimitive::U64 => "return dcoDecodeUint64List(raw);".into(),
            _ => gen_decode_simple_type_cast(self.mir.clone().into(), self.context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;

    /// Uses 64-bit list adapters while retaining ordinary typed-list casts.
    #[test]
    fn primitive_list_decoder_covers_64_bit_and_fallback_branches() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let context = test_utils::context(&pack, &config);

        for (primitive, expected) in [
            (MirTypePrimitive::I64, "return dcoDecodeInt64List(raw);"),
            (MirTypePrimitive::U64, "return dcoDecodeUint64List(raw);"),
            (MirTypePrimitive::I32, "return raw as Int32List;"),
        ] {
            let generator = PrimitiveListWireDartCodecDcoGenerator::new(
                crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList {
                    primitive,
                    strict_dart_type: true,
                },
                context,
            );
            assert_eq!(generator.generate_impl_decode_body(), expected);
        }
    }
}
