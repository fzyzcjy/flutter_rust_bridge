use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::ty::IrType;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for OptionalWireRustCodecDcoGenerator<'a> {
    fn intodart_type(&self, ir_pack: &crate::codegen::ir::pack::IrPack) -> String {
        let inner =
            WireRustCodecDcoGenerator::new(IrType::from(self.ir.inner.clone()), self.context);
        format!("Option<{}>", inner.intodart_type(ir_pack))
    }
}
