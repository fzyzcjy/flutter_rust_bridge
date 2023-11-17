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

impl<'a> StructRefDartApiGenerator<'a> {
    pub(super) fn generate_mode_freezed(
        &self,
        src: &IrStruct,
        comments: &str,
        metadata: &str,
        has_methods: bool,
        methods: &str,
    ) -> String {
        let private_constructor = if has_methods {
            format!("const {}._();", self.ir.ident.0)
        } else {
            "".to_owned()
        };

        let mut constructor_params = src
            .fields
            .iter()
            .map(|field| {
                let r#default =
                    generate_field_default(field, true, self.context.config.dart_enums_style);
                format!(
                    "{default} {} {} {},",
                    generate_field_required_modifier(field),
                    DartApiGenerator::new(field.ty.clone(), self.context.clone()).dart_api_type(),
                    field.name.dart_style()
                )
            })
            .collect_vec();
        if has_methods && self.context.config.use_bridge_in_method {
            constructor_params.insert(
                0,
                // TODO merge with extra_argument
                format!(
                    "required {} bridge,",
                    self.context.config.dart_api_class_name
                ),
            );
        }
        let constructor_params = constructor_params.join("");
        let name_str = &self.ir.ident.0;
        let implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception);

        format!(
            "{comments}{metadata}class {name_str} with _${name_str} {implements_exception} {{
                {private_constructor}
                const factory {name_str}({{{constructor_params}}}) = _{name_str};
                {methods}
            }}",
        )
    }
}
