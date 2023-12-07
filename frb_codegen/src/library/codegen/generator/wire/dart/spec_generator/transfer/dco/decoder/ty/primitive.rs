use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for PrimitiveWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        match self.ir {
            IrTypePrimitive::Unit => "return;".to_owned(),
            IrTypePrimitive::I64 | IrTypePrimitive::U64 | IrTypePrimitive::Usize => {
                "return dcoDecodeI64OrU64(raw);".to_owned()
            }
            _ => gen_decode_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
