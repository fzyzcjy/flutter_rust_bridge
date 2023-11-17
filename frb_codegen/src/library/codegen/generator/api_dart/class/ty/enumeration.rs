use crate::codegen::generator::api_dart::base::*;
use crate::codegen::generator::api_dart::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::ir::ty::enumeration::IrEnumMode;

impl<'a> ApiDartGeneratorClassTrait for EnumRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        match src.mode {
            IrEnumMode::Simple => self.generate_mode_simple(src),
            IrEnumMode::Complex => self.generate_mode_complex(src),
        }
    }
}
