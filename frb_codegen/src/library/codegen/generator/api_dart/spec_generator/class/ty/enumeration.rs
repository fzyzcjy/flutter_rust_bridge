use crate::codegen::generator::api_dart::spec_generator::class::method::{
    generate_api_methods, GenerateApiMethodConfig,
};
use crate::codegen::generator::api_dart::spec_generator::class::misc::generate_class_extra_body;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::mir::ty::enumeration::MirEnumMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;

impl<'a> ApiDartGeneratorClassTrait for EnumRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let src = self.mir.get(self.context.mir_pack);

        let methods = generate_api_methods(
            &MirType::EnumRef(self.mir.clone()),
            self.context,
            &GenerateApiMethodConfig::COMBINED,
            &src.name.name,
        );
        let extra_code =
            generate_class_extra_body(self.mir_type(), &self.context.mir_pack.dart_code_of_type);

        let body = methods.code + &extra_code.body;
        let header = methods.header + extra_code.header;

        match src.mode {
            MirEnumMode::Simple => self.generate_mode_simple(src, &body, header),
            MirEnumMode::Complex => self.generate_mode_complex(src, &body, header),
        }
    }
}
