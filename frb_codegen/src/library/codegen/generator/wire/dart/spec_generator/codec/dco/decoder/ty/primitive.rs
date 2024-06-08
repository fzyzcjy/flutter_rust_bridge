use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for PrimitiveWireDartCodecDcoGenerator<'a> {
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
