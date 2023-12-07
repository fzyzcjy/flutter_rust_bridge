use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::encoder::ty::WireRustTransferDcoGeneratorEncoderTrait;
use crate::codegen::ir::pack::IrPack;

impl<'a> WireRustTransferDcoGeneratorEncoderTrait for OptionalListWireRustTransferDcoGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        format!(
            "Vec<Option<{}>>",
            WireRustTransferDcoGenerator::new(self.ir.inner.clone(), self.context)
                .intodart_type(ir_pack)
        )
    }
}
