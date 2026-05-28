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
            // Fixed-width integers: on dart2wasm, `dartify()` returns every JS
            // number as Dart `double`, so a direct `raw as int` cast throws.
            // Narrow through `num` — a no-op on dart2js, an explicit `.toInt()`
            // on dart2wasm.
            MirTypePrimitive::I8
            | MirTypePrimitive::I16
            | MirTypePrimitive::I32
            | MirTypePrimitive::U8
            | MirTypePrimitive::U16
            | MirTypePrimitive::U32 => "return (raw as num).toInt();".to_owned(),
            _ => gen_decode_simple_type_cast(self.mir.clone().into(), self.context),
        }
    }
}
