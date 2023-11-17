use crate::codegen::generator::dart_api::base::*;

use crate::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;

use crate::codegen::ir::ty::enumeration::IrEnumMode;

impl<'a> DartApiGeneratorClassTrait for EnumRefDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        match src.mode {
            IrEnumMode::Simple => self.generate_mode_simple(src),
            IrEnumMode::Complex => self.generate_mode_complex(src),
        }
    }
}
