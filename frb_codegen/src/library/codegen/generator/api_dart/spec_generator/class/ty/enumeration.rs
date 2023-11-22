use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::ty::enumeration::IrEnumMode;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;

impl<'a> ApiDartGeneratorClassTrait for EnumRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let src = self.ir.get(self.context.ir_pack);
        match src.mode {
            IrEnumMode::Simple => self.generate_mode_simple(src),
            IrEnumMode::Complex => self.generate_mode_complex(src),
        }
    }
}
