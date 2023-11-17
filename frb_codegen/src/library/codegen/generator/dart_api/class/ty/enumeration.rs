use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;
use crate::codegen::generator::dart_api::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception,
};
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumMode, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;
use crate::utils::dart_keywords::make_string_keyword_safe;

impl<'a> DartApiGeneratorClassTrait for EnumRefDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        match src.mode {
            IrEnumMode::Simple => self.generate_mode_simple(src),
            IrEnumMode::Complex => self.generate_mode_complex(src),
        }
    }
}
