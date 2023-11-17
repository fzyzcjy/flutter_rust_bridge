use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::dart_api::class::ty::structure_method::generate_api_method;
use crate::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;
use crate::codegen::generator::dart_api::internal_config::GeneratorDartApiInternalConfig;
use crate::codegen::generator::dart_api::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception, generate_dart_metadata,
};
use crate::codegen::ir::func::{
    IrFunc, IrFuncMode, IrFuncOwnerInfo, IrFuncOwnerInfoMethod, IrFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

impl<'a> DartApiGeneratorClassTrait for StructRefDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let methods = self.context.ir_pack
            .funcs
            .iter()
            .filter(|f| {
                matches!(&f.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod{ struct_name, .. }) if struct_name == &src.name)
            })
            .collect_vec();

        let has_methods = !methods.is_empty();

        let methods = methods
            .iter()
            .map(|func| generate_api_method(func, src, &self.context))
            .collect_vec()
            .concat();

        let extra_argument = if self.context.config.use_bridge_in_method {
            "required this.bridge,".to_string()
        } else {
            "".to_string()
        };

        let field_bridge = if self.context.config.use_bridge_in_method {
            format!("final {} bridge;", self.context.config.dart_api_class_name)
        } else {
            String::new()
        };

        Some(if src.using_freezed() {
            self.generate_mode_freezed(src, &comments, &metadata, has_methods, &methods)
        } else {
            self.generate_mode_non_freezed(
                src,
                &comments,
                &metadata,
                has_methods,
                &methods,
                &extra_argument,
                field_bridge,
            )
        })
    }
}
