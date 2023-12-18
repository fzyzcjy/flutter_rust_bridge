use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::pack::IrPack;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for GeneralListWireRustCodecDcoGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        format!(
            "Vec<{}>",
            WireRustCodecDcoGenerator::new(self.ir.inner.clone(), self.context)
                .intodart_type(ir_pack)
        )
    }
}
