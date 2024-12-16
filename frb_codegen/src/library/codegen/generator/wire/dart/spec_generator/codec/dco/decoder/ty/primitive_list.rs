use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for PrimitiveListWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        match &self.mir.primitive {
            MirTypePrimitive::I64 => "return dcoDecodeInt64List(raw);".into(),
            MirTypePrimitive::U64 => "return dcoDecodeUint64List(raw);".into(),
            _ => gen_decode_simple_type_cast(self.mir.clone().into(), self.context),
        }
    }
}
