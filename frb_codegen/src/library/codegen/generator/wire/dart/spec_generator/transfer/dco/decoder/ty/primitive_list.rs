use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait
    for PrimitiveListWireDartTransferDcoGenerator<'a>
{
    fn generate_impl_decode_body(&self) -> String {
        match &self.ir.primitive {
            IrTypePrimitive::I64 => "return Int64List.from(raw);".into(),
            IrTypePrimitive::U64 => "return Uint64List.from(raw);".into(),
            _ => gen_decode_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
