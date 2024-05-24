use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for PrimitiveWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        match self.ir {
            IrTypePrimitive::Unit => "return;".to_owned(),
            IrTypePrimitive::I64 | IrTypePrimitive::Isize => "return dcoDecodeI64(raw);".to_owned(),
            IrTypePrimitive::U64 | IrTypePrimitive::Usize => "return dcoDecodeU64(raw);".to_owned(),
            _ => gen_decode_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
